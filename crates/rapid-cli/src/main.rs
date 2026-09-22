use anyhow::Result;
use clap::{Parser, Subcommand};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rapid_core::{DownloadConfig, DownloadStatus, DownloadTask, Probe};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::signal;

#[derive(Parser, Debug)]
#[command(name = "rapid")]
#[command(about = "High-Performance Multi-Segment Download Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Download a file with multi-connection acceleration
    Download {
        /// URL of the file to download
        url: String,

        /// Number of parallel segments (connections)
        #[arg(short = 's', long, default_value_t = 8)]
        segments: usize,

        /// Output directory
        #[arg(short = 'o', long)]
        output_dir: Option<PathBuf>,

        /// Custom filename
        #[arg(short = 'f', long)]
        filename: Option<String>,
    },

    /// Inspect URL headers, file size, and range support
    Probe {
        /// URL to inspect
        url: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Probe { url } => {
            println!("🔍 Probing URL: {}", url);

            match Probe::inspect_url(&url).await {
                Ok(meta) => {
                    println!("\n✅ Metadata retrieved:");
                    println!("  • Filename:       {}", meta.filename);
                    if let Some(len) = meta.content_length {
                        println!("  • Size:           {} bytes ({:.2} MB)", len, len as f64 / (1024.0 * 1024.0));
                    } else {
                        println!("  • Size:           Unknown (streaming)");
                    }
                    println!("  • Resume/Ranges:  {}", if meta.accept_ranges { "Supported (Multi-part available)" } else { "Not supported (Single stream only)" });
                    if let Some(etag) = meta.etag {
                        println!("  • ETag:           {}", etag);
                    }
                    if let Some(lm) = meta.last_modified {
                        println!("  • Last-Modified:  {}", lm);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Probe failed: {}", e);
                }
            }
        }

        Commands::Download {
            url,
            segments,
            output_dir,
            filename,
        } => {
            let out = output_dir.unwrap_or_else(|| PathBuf::from("./downloads"));
            let config = DownloadConfig {
                url: url.clone(),
                output_dir: out,
                custom_filename: filename,
                num_segments: segments,
                ..Default::default()
            };

            println!("⚡ Rapid Download Manager");
            println!("Target URL: {}", config.url);
            println!("Target Folder: {}", config.output_dir.display());
            println!("Connections: {}", config.num_segments);

            let task = Arc::new(DownloadTask::create(uuid_simple(), config).await?);

            println!("File: {}", task.target_file.display());
            if let Some(size) = task.total_bytes {
                println!("Total Size: {:.2} MB", size as f64 / (1024.0 * 1024.0));
            }

            // Setup multi-progress visual display
            let mp = MultiProgress::new();
            let total_bytes = task.total_bytes.unwrap_or(0);

            let main_bar = mp.add(ProgressBar::new(total_bytes));
            main_bar.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({percent}%) | {msg}")
                    .unwrap()
                    .progress_chars("#>-"),
            );

            let mut seg_bars = Vec::new();
            let seg_snapshots = task.snapshot_segments().await;
            for seg in &seg_snapshots {
                let sb = mp.add(ProgressBar::new(seg.total_bytes()));
                sb.set_style(
                    ProgressStyle::default_bar()
                        .template("  Part {prefix:>2}: [{bar:25.yellow/dim}] {bytes}/{total_bytes} ({percent}%)")
                        .unwrap()
                        .progress_chars("=>-"),
                );
                sb.set_prefix(format!("{}", seg.index + 1));
                sb.set_position(seg.downloaded_bytes);
                seg_bars.push(sb);
            }

            let mut rx = task.subscribe();
            let task_for_ctrlc = Arc::clone(&task);

            // Handle Ctrl+C gracefully for Pause/Resume
            tokio::spawn(async move {
                if signal::ctrl_c().await.is_ok() {
                    println!("\n⏸ Pause signal received! Saving state...");
                    task_for_ctrlc.pause();
                }
            });

            // UI update task
            let ui_handle = tokio::spawn(async move {
                while let Ok(progress) = rx.recv().await {
                    main_bar.set_position(progress.downloaded_bytes);
                    let speed_mb = progress.speed_bps as f64 / (1024.0 * 1024.0);
                    let eta = progress.eta_seconds.map(|s| format!("{}s", s)).unwrap_or_else(|| "--".to_string());
                    main_bar.set_message(format!("{:.2} MB/s | ETA: {}", speed_mb, eta));

                    for seg in &progress.segments {
                        if seg.index < seg_bars.len() {
                            seg_bars[seg.index].set_position(seg.downloaded_bytes);
                            if seg.is_complete {
                                seg_bars[seg.index].finish_with_message("Done");
                            }
                        }
                    }

                    if progress.status == DownloadStatus::Completed {
                        main_bar.finish_with_message("✨ Complete!");
                        break;
                    } else if progress.status == DownloadStatus::Paused {
                        main_bar.abandon_with_message("⏸ Paused (State saved)");
                        break;
                    }
                }
            });

            match task.run().await {
                Ok(_) => {
                    let _ = ui_handle.await;
                    println!("\n🎉 Download successfully finished: {}", task.target_file.display());
                }
                Err(rapid_core::RapidError::Cancelled) => {
                    let _ = ui_handle.await;
                    println!("\n⏸ Download paused. Run the same command again to resume!");
                }
                Err(e) => {
                    let _ = ui_handle.await;
                    eprintln!("\n❌ Download encountered error: {}", e);
                }
            }
        }
    }

    Ok(())
}

fn uuid_simple() -> String {
    use std::time::SystemTime;
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    format!("task-{}", duration.as_millis())
}
