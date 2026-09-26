# Privacy Policy — Rapid Download Manager

**Last Updated:** September 2026  
**Publisher:** Promahbubul  
**Product:** Rapid Download Manager  

## 1. Overview
Rapid Download Manager is committed to protecting your privacy. We believe your downloads, files, and personal data belong exclusively to you. **Rapid Download Manager does not collect, track, store, or transmit any personally identifiable information (PII) or telemetry to external servers.**

## 2. Information We DO NOT Collect
* ❌ No Personal Data: We do not collect names, email addresses, phone numbers, or account credentials.
* ❌ No Download Tracking: We do not log, monitor, or transmit the URLs you download or the files you save.
* ❌ No Analytics or Telemetry: The software contains zero third-party analytics trackers, advertising SDKs, or background telemetry beacons.
* ❌ No IP or Geolocation Tracking: We do not track or record your IP address or geographic location.

## 3. Local Data Storage
All application configurations, scheduler timings, and download history are stored **exclusively on your local device** in standard Windows application directories:
* `%APPDATA%\RapidDownloadManager\history.json` (Download history)
* `%APPDATA%\RapidDownloadManager\config.json` (User preferences)
* `%LOCALAPPDATA%\RapidDownloadManager\logs\app.log` (Local diagnostic logs)

This data never leaves your computer and can be completely wiped at any time by uninstalling the application or clearing the history from within the app.

## 4. Sensitive Data Sanitization
Rapid Download Manager includes a built-in Zero-Leak Sanitizer in its logging system. If a download URL contains query parameters such as `token=`, `api_key=`, or `password=`, the diagnostic logger automatically redacts them to `[REDACTED]` before writing to local diagnostic files.

## 5. Network Connections
Rapid Download Manager establishes network connections solely to:
1. Fetch the files you explicitly request to download.
2. Query the official GitHub Releases API (`api.github.com`) only when you manually check for updates or open the About dialog.

All network connections use pure, memory-safe **Rustls TLS** for end-to-end cryptographic encryption.

## 6. Contact & Inquiries
If you have any questions about this Privacy Policy, please open an issue on our official GitHub repository:
https://github.com/promahbubul/rapid_download_manager/issues
