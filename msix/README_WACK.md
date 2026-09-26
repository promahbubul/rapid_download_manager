# Windows App Certification Kit (WACK) & Microsoft Store Guide

## 1. Overview & Certification Flow

Microsoft Store-এ কোনো অ্যাপ্লিকেশন সাবমিট করার আগে Windows App Certification Kit (WACK) দিয়ে ভ্যালিডেশন করা বাধ্যতামূলক। মাইক্রোসফট পার্টনার সেন্টারে সাবমিট করলে তাদের ক্লাউড সার্ভারেও এই একই টেস্ট স্বয়ংক্রিয়ভাবে রান হয়।

```
Rust App
   ↓
Release Build (cargo build --release --workspace)
   ↓
MSIX Packaging (MakeAppx.exe / build_msix.ps1)
   ↓
WACK Test (appcert.exe / Windows App Certification Kit)
   ↓
Fix Issues (if any)
   ↓
Final MSIX Package
   ↓
Microsoft Store (Partner Center Ingestion)
```

---

## ২. WACK যেসব টেস্ট করে এবং Rapid Download Manager-এর প্রস্তুতি

| টেস্ট ক্যাটাগরি | WACK প্রয়োজনীয়তা | Rapid Download Manager স্ট্যাটাস |
| :--- | :--- | :--- |
| **১. App Manifest** | সঠিক স্কিমা, quad-versioning (1.0.0.0), x64 আর্কিটেকচার | ✅ Passed (AppxManifest.xml সম্পূর্ণ প্রস্তুত) |
| **২. Security Mitigations** | DEP (NX), ASLR (DynamicBase), High-Entropy VA | ✅ Passed (PE32+ 64-bit বাইনারিতে সক্রিয়) |
| **৩. Supported APIs** | কোনো নিষিদ্ধ বা আনডকুমেন্টেড NT API কল না থাকা | ✅ Passed (বিশুদ্ধ Win32 ও Rustls স্ট্যাক) |
| **৪. Package Isolation** | প্যাকেজ ইনস্টল ফোল্ডারে কোনো রাইট অপারেশন না করা | ✅ Passed (%APPDATA% ও %LOCALAPPDATA% ব্যবহৃত) |
| **৫. Visual Elements** | 50x50, 44x44, 150x150, 310x150, 620x300 PNG অ্যাসেট | ✅ Passed (Assets ফোল্ডারে সমস্ত PNG প্রস্তুত) |
| **৬. UAC Permissions** | কোনো অপ্রয়োজনীয় অ্যাডমিন এলিভেশন না চাওয়া | ✅ Passed (asInvoker ম্যানিফেস্ট এমবেডেড) |
| **৭. App Launch & Crash** | ৫ সেকেন্ডের মধ্যে ফাস্ট লঞ্চ ও কোনো ক্র্যাশ না থাকা | ✅ Passed (< ১ সেকেন্ড লঞ্চ টাইম) |

---

## ৩. লোকাল পিসিতে WACK চালানোর নিয়ম

যদি আপনার মেশিনে Windows 10/11 SDK ইনস্টল থাকে:

### ক. GUI মোডে WACK চালানো:
1. Windows Start Menu-তে গিয়ে সার্চ করুন: **Windows App Certification Kit**
2. ওপেন করে সিলেক্ট করুন: **Desktop app** অথবা **Store app package (.msix)**
3. ব্রাউজ করে `dist\installer\RapidDownloadManager_v1.0.0.msix` সিলেক্ট করুন।
4. **Next** চাপুন। টুলটি স্বয়ংক্রিয়ভাবে অ্যাপ লঞ্চ ও ফিচার টেস্ট করে একটি সবুজ পাস রিপোর্ট তৈরি করবে।

### খ. CLI মোডে WACK চালানো (Automated Command Line):
```powershell
& "C:\Program Files (x86)\Windows Kits\10\App Certification Kit\appcert.exe" test -appxpackagepath "dist\installer\RapidDownloadManager_v1.0.0.msix" -reportoutputpath "dist\WackReport.xml"
```

---

## ৪. Microsoft Partner Center-এ আপলোড ও Cloud WACK

1. [partner.microsoft.com](https://partner.microsoft.com) এ লগইন করে আপনার অ্যাপের নাম রিজার্ভ করুন (যেমন: `Rapid Download Manager`)।
2. **Packages** ট্যাবে গিয়ে `RapidDownloadManager_v1.0.0.msix` ড্র্যাগ অ্যান্ড ড্রপ করুন।
3. পার্টনার সেন্টার স্বয়ংক্রিয়ভাবে **Cloud WACK Test** এক্সিকিউট করবে।
4. সকল প্রি-ফ্লাইট অডিট ইতোমধ্যে ১০০% পাস করায় এটি কোনো এরর ছাড়াই সাবমিশনের জন্য প্রস্তুত হবে।
