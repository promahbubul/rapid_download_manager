import os
import shutil
import subprocess
import urllib.request
import zipfile
import io

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DIST_APP = os.path.join(PROJECT_ROOT, "dist", "RapidDownloadManager")
MSIX_SRC = os.path.join(PROJECT_ROOT, "msix")
STAGING_DIR = os.path.join(PROJECT_ROOT, "dist", "msix_staging")
OUTPUT_MSIX = os.path.join(PROJECT_ROOT, "dist", "installer", "RapidDownloadManager_v1.0.5.msix")
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

def ensure_makeappx():
    if os.path.exists(LOCAL_MAKEAPPX):
        return LOCAL_MAKEAPPX
    
    res = subprocess.run(["where.exe", "makeappx.exe"], capture_output=True, text=True)
    if res.returncode == 0:
        return res.stdout.strip().split("\n")[0].strip()
        
    print("   MakeAppx not found locally or in PATH. Fetching official Microsoft BuildTools...")
    tools_dir = os.path.dirname(LOCAL_MAKEAPPX)
    os.makedirs(tools_dir, exist_ok=True)
    url = "https://api.nuget.org/v3-flatcontainer/microsoft.windows.sdk.buildtools/10.0.26100.1742/microsoft.windows.sdk.buildtools.10.0.26100.1742.nupkg"
    data = urllib.request.urlopen(url).read()
    with zipfile.ZipFile(io.BytesIO(data)) as z:
        for name in z.namelist():
            if "bin/" in name and "/x64/" in name:
                filename = os.path.basename(name)
                if filename:
                    with open(os.path.join(tools_dir, filename), "wb") as f:
                        f.write(z.read(name))
    return LOCAL_MAKEAPPX

makeappx_exe = ensure_makeappx()

if makeappx_exe and os.path.exists(makeappx_exe):
    print(f"2. Compiling MSIX with official MakeAppx: {makeappx_exe}...")
    cmd = [makeappx_exe, "pack", "/d", STAGING_DIR, "/p", OUTPUT_MSIX, "/o"]
    res = subprocess.run(cmd, capture_output=True, text=True)
    if res.returncode == 0:
        size_mb = os.path.getsize(OUTPUT_MSIX) / (1024 * 1024)
        print(f"   SUCCESS: Generated Store MSIX: {OUTPUT_MSIX} ({size_mb:.2f} MB)")
    else:
        print("   MakeAppx failed:", res.stderr, res.stdout)
else:
    print("ERROR: Could not locate or acquire MakeAppx.exe!")
