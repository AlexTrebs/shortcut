[Setup]
AppName=Shortcut
AppVersion={#AppVersion}
DefaultDirName={pf}\Shortcut
OutputDir=.
OutputBaseFilename=ShortcutInstaller
Compression=lzma
SolidCompression=yes
SetupIconFile=docs\icon.ico

[Files]
Source: "target/x86_64-pc-windows-gnu/release/shortcut.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "../target/x86_64-pc-windows-gnu/release/ui/*"; DestDir: "{app}/ui/"; Flags: ignoreversion recursesubdirs
Source: "../.env"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs

[Icons]
Name: "{group}\Shortcut"; Filename: "{app}\shortcut.exe"