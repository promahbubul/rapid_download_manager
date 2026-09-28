; Inno Setup Script for Rapid Download Manager
#define MyAppName "Rapid Download Manager"
#define MyAppVersion "1.0.3"
#define MyAppQuadVersion "1.0.3.0"
#define MyAppPublisher "Promahbubul"
#define MyAppURL "https://github.com/promahbubul/rapid_download_manager"
#define MyAppExeName "rapid-gui.exe"

[Setup]
AppId={{D37E84C1-39F2-43C2-A62E-064C63E3F811}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
VersionInfoVersion={#MyAppQuadVersion}
VersionInfoProductVersion={#MyAppQuadVersion}
VersionInfoTextVersion={#MyAppVersion}
VersionInfoCompany={#MyAppPublisher}
VersionInfoDescription={#MyAppName} Setup
VersionInfoCopyright=Copyright (C) 2026 {#MyAppPublisher}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\Rapid Download Manager
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
OutputDir=..\dist\installer
OutputBaseFilename=RapidDownloadManager_Setup_v{#MyAppVersion}
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
CloseApplications=yes
RestartApplications=no
CloseApplicationsFilter=*.exe
DisableProgramGroupPage=yes

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "autostart"; Description: "Start Rapid Download Manager when Windows starts"; GroupDescription: "System Startup:"
Name: "browserintegration"; Description: "Automatically integrate extension into Google Chrome, Microsoft Edge, Brave & Firefox"; GroupDescription: "Browser Integration:"; Flags: checkedonce

[Files]
Source: "..\dist\RapidDownloadManager\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\Install Browser Extension"; Filename: "{app}\install_extension.bat"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Registry]
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "RapidDownloadManager"; ValueData: """{app}\{#MyAppExeName}"""; Flags: uninsdeletevalue; Tasks: autostart

; Browser Extension Auto-Registrations
Root: HKCU; Subkey: "Software\Google\Chrome\Extensions\rapid_download_manager"; ValueType: string; ValueName: "path"; ValueData: "{app}\extension"; Flags: uninsdeletekey; Tasks: browserintegration
Root: HKCU; Subkey: "Software\Microsoft\Edge\Extensions\rapid_download_manager"; ValueType: string; ValueName: "path"; ValueData: "{app}\extension"; Flags: uninsdeletekey; Tasks: browserintegration
Root: HKCU; Subkey: "Software\BraveSoftware\Brave-Browser\Extensions\rapid_download_manager"; ValueType: string; ValueName: "path"; ValueData: "{app}\extension"; Flags: uninsdeletekey; Tasks: browserintegration
Root: HKCU; Subkey: "Software\Opera Software\Extensions\rapid_download_manager"; ValueType: string; ValueName: "path"; ValueData: "{app}\extension"; Flags: uninsdeletekey; Tasks: browserintegration
Root: HKCU; Subkey: "Software\Mozilla\Firefox\Extensions"; ValueType: string; ValueName: "rapid-downloader@promahbubul.com"; ValueData: "{app}\extension"; Flags: uninsdeletevalue; Tasks: browserintegration

[Run]
Filename: "powershell.exe"; Parameters: "-ExecutionPolicy Bypass -NoProfile -File ""{app}\auto_integrate_browsers.ps1"" -Silent"; Flags: runhidden; Tasks: browserintegration
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent
[UninstallDelete]
Type: filesandordirs; Name: "{app}"
[Code]
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  AppDataDir: String;
  LocalAppDataDir: String;
begin
  if CurUninstallStep = usPostUninstall then
  begin
    if MsgBox('Would you like to completely delete your download history, logs and settings?' + #13#10 + 'Choose No if you plan to reinstall or update later.', mbConfirmation, MB_YESNO or MB_DEFBUTTON2) = IDYES then
    begin
      AppDataDir := ExpandConstant('{userappdata}\RapidDownloadManager');
      LocalAppDataDir := ExpandConstant('{localappdata}\RapidDownloadManager');
      DelTree(AppDataDir, True, True, True);
      DelTree(LocalAppDataDir, True, True, True);
    end;
  end;
end;
