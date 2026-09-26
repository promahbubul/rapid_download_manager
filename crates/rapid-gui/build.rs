fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("windows") {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let res_file = format!("{}/app_res.o", out_dir);
        let status = std::process::Command::new("windres")
            .args(&["app.rc", "-o", &res_file])
            .status();
        if let Ok(s) = status {
            if s.success() {
                println!("cargo:rustc-link-arg={}", res_file);
            }
        }
    }
}
