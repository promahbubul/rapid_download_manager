import os
import shutil
import subprocess

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DIST_APP = os.path.join(PROJECT_ROOT, "dist", "RapidDownloadManager")
MSIX_SRC = os.path.join(PROJECT_ROOT, "msix")
STAGING_DIR = os.path.join(PROJECT_ROOT, "dist", "msix_staging")
OUTPUT_MSIX = os.path.join(PROJECT_ROOT, "dist", "installer", "RapidDownloadManager_v1.0.0.msix")
LOCAL_MAKEAPPX = os.path.join(MSIX_SRC, "tools", "x64", "makeappx.exe")

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

makeappx_exe = None
if os.path.exists(LOCAL_MAKEAPPX):
    makeappx_exe = LOCAL_MAKEAPPX
else:
    res = subprocess.run(["where.exe", "makeappx.exe"], capture_output=True, text=True)
    if res.returncode == 0:
        makeappx_exe = res.stdout.strip().split("\n")[0].strip()

if makeappx_exe:
    print(f"2. Compiling MSIX with Windows SDK MakeAppx: {makeappx_exe}...")
    cmd = [makeappx_exe, "pack", "/d", STAGING_DIR, "/p", OUTPUT_MSIX, "/o"]
    res = subprocess.run(cmd, capture_output=True, text=True)
    if res.returncode == 0:
        size_mb = os.path.getsize(OUTPUT_MSIX) / (1024 * 1024)
        print(f"   SUCCESS: Generated Store MSIX: {OUTPUT_MSIX} ({size_mb:.2f} MB)")
    else:
        print("   MakeAppx failed:", res.stderr, res.stdout)
else:
    print("ERROR: MakeAppx.exe not found! MSIX must be compiled using official MakeAppx tool.")
