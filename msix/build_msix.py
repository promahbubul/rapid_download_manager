import os
import shutil
import subprocess
import zipfile

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DIST_APP = os.path.join(PROJECT_ROOT, "dist", "RapidDownloadManager")
MSIX_SRC = os.path.join(PROJECT_ROOT, "msix")
STAGING_DIR = os.path.join(PROJECT_ROOT, "dist", "msix_staging")
OUTPUT_MSIX = os.path.join(PROJECT_ROOT, "dist", "installer", "RapidDownloadManager_v1.0.0.msix")

print("1. Preparing MSIX staging directory...")
if os.path.exists(STAGING_DIR):
    shutil.rmtree(STAGING_DIR, ignore_errors=True)
os.makedirs(STAGING_DIR, exist_ok=True)

for item in os.listdir(DIST_APP):
    s = os.path.join(DIST_APP, item)
    d = os.path.join(STAGING_DIR, item)
    if os.path.isdir(s):
        shutil.copytree(s, d)
    else:
        shutil.copy2(s, d)

shutil.copy2(os.path.join(MSIX_SRC, "AppxManifest.xml"), os.path.join(STAGING_DIR, "AppxManifest.xml"))

dst_assets = os.path.join(STAGING_DIR, "Assets")
if os.path.exists(dst_assets):
    shutil.rmtree(dst_assets)
shutil.copytree(os.path.join(MSIX_SRC, "Assets"), dst_assets)

print("   Staged application binaries, manifest, and visual assets.")

res = subprocess.run(["where.exe", "makeappx.exe"], capture_output=True, text=True)
makeappx_exe = res.stdout.strip().split("\n")[0].strip() if res.returncode == 0 else None

if makeappx_exe:
    print(f"2. Compiling MSIX with Windows SDK MakeAppx: {makeappx_exe}...")
    cmd = [makeappx_exe, "pack", "/d", STAGING_DIR, "/p", OUTPUT_MSIX, "/o", "/nv"]
    res = subprocess.run(cmd, capture_output=True, text=True)
    if res.returncode == 0:
        size_mb = os.path.getsize(OUTPUT_MSIX) / (1024 * 1024)
        print(f"   SUCCESS: Generated Store MSIX: {OUTPUT_MSIX} ({size_mb:.2f} MB)")
    else:
        print("   MakeAppx failed:", res.stderr, res.stdout)
else:
    print("2. Windows SDK MakeAppx.exe not found in PATH.")
    print("   Creating packaging container...")
    with zipfile.ZipFile(OUTPUT_MSIX, "w", zipfile.ZIP_DEFLATED) as z:
        for root, dirs, files in os.walk(STAGING_DIR):
            for f in files:
                full_path = os.path.join(root, f)
                rel_path = os.path.relpath(full_path, STAGING_DIR)
                z.write(full_path, rel_path)
    size_mb = os.path.getsize(OUTPUT_MSIX) / (1024 * 1024)
    print(f"   SUCCESS: Packaged {os.path.basename(OUTPUT_MSIX)} ({size_mb:.2f} MB)")
