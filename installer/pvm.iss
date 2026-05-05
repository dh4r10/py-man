; PVM — Python Version Manager
; Inno Setup Script
; Requiere Inno Setup 6+ (https://jrsoftware.org/isinfo.php)

#define AppName    "PVM"
#define AppFullName "PVM - Python Version Manager"
#define AppVersion  "1.0.0"
#define AppExe      "pvm.exe"
#define AppURL      "https://github.com/Schniz/fnm"

[Setup]
AppId={{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}
AppName={#AppFullName}
AppVersion={#AppVersion}
AppPublisherURL={#AppURL}
AppSupportURL={#AppURL}
AppUpdatesURL={#AppURL}

; Instala en %LOCALAPPDATA%\pvm\ — no requiere permisos de administrador
DefaultDirName={localappdata}\pvm
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes

; Sin necesidad de ser administrador
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog

; Notifica a Windows que el PATH cambió
ChangesEnvironment=yes

; Salida
OutputDir=..\dist
OutputBaseFilename=pvm-setup-{#AppVersion}
SetupIconFile=..\img\logo_sin_fondo.ico
WizardImageFile=..\img\logo_sin_fondo.png
WizardSmallImageFile=..\img\logo_sin_fondo.png
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern

; Información de la licencia / bienvenida
; LicenseFile=..\LICENSE

[Languages]
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "addprofile"; \
  Description: "Añadir pvm al perfil de PowerShell ({code:GetProfilePath})"; \
  GroupDescription: "Integración con PowerShell:"

[Files]
Source: "..\target\release\pvm.exe"; \
  DestDir: "{app}"; \
  Flags: ignoreversion

[Registry]
; Añade {app} al PATH del usuario (permanente, sin admin)
Root: HKCU; \
  Subkey: "Environment"; \
  ValueType: expandsz; \
  ValueName: "PATH"; \
  ValueData: "{app};{olddata}"; \
  Check: NeedsAddPath(ExpandConstant('{app}'))

[Code]

{ ── Helpers ─────────────────────────────────────────────────────────────── }

function NeedsAddPath(Dir: string): Boolean;
var
  OldPath: string;
begin
  if not RegQueryStringValue(HKCU, 'Environment', 'PATH', OldPath) then
  begin
    Result := True;
    Exit;
  end;
  Result := Pos(';' + Uppercase(Dir) + ';', ';' + Uppercase(OldPath) + ';') = 0;
end;

function GetProfilePath(Param: string): string;
var
  Res: Integer;
begin
  // Pregunta a PowerShell cuál es la ruta del perfil
  if Exec(
    ExpandConstant('{sys}\WindowsPowerShell\v1.0\powershell.exe'),
    '-NonInteractive -Command "$PROFILE"',
    '',
    SW_HIDE,
    ewWaitUntilTerminated,
    Res
  ) then
    Result := '$PROFILE'
  else
    Result := '$PROFILE';
end;

function GetPowerShellProfile(): string;
var
  TempFile: string;
  Lines: TArrayOfString;
  ResultCode: Integer;
begin
  Result := '';
  TempFile := ExpandConstant('{tmp}\pvm_profile.txt');

  if Exec(
    ExpandConstant('{sys}\WindowsPowerShell\v1.0\powershell.exe'),
    '-NonInteractive -Command "$PROFILE | Out-File -FilePath ''' + TempFile + ''' -Encoding UTF8"',
    '',
    SW_HIDE,
    ewWaitUntilTerminated,
    ResultCode
  ) then
  begin
    if LoadStringsFromFile(TempFile, Lines) and (GetArrayLength(Lines) > 0) then
      Result := Trim(Lines[0]);
  end;

  DeleteFile(TempFile);
end;

const
  PvmProfileLine = 'pvm env | Out-String | Invoke-Expression';
  PvmProfileComment = '# pvm — Python Version Manager';

procedure AddToPowerShellProfile();
var
  ProfilePath: string;
  ProfileDir: string;
  Lines: TArrayOfString;
  Content: string;
  I: Integer;
  AlreadyPresent: Boolean;
begin
  ProfilePath := GetPowerShellProfile();
  if ProfilePath = '' then
    Exit;

  // Crear directorio del perfil si no existe
  ProfileDir := ExtractFileDir(ProfilePath);
  if not DirExists(ProfileDir) then
    CreateDir(ProfileDir);

  AlreadyPresent := False;

  if FileExists(ProfilePath) then
  begin
    if LoadStringsFromFile(ProfilePath, Lines) then
    begin
      for I := 0 to GetArrayLength(Lines) - 1 do
      begin
        if Pos(PvmProfileLine, Lines[I]) > 0 then
        begin
          AlreadyPresent := True;
          Break;
        end;
      end;
    end;
  end;

  if not AlreadyPresent then
  begin
    Content := #13#10 + PvmProfileComment + #13#10 + PvmProfileLine + #13#10;
    SaveStringToFile(ProfilePath, Content, True); // append
  end;
end;

procedure RemoveFromPowerShellProfile();
var
  ProfilePath: string;
  Lines: TArrayOfString;
  NewLines: TArrayOfString;
  I, J: Integer;
  Found: Boolean;
begin
  ProfilePath := GetPowerShellProfile();
  if ProfilePath = '' then
    Exit;
  if not FileExists(ProfilePath) then
    Exit;

  if not LoadStringsFromFile(ProfilePath, Lines) then
    Exit;

  J := 0;
  SetArrayLength(NewLines, GetArrayLength(Lines));
  for I := 0 to GetArrayLength(Lines) - 1 do
  begin
    if (Pos(PvmProfileLine, Lines[I]) = 0) and
       (Pos(PvmProfileComment, Lines[I]) = 0) then
    begin
      NewLines[J] := Lines[I];
      J := J + 1;
    end;
  end;
  SetArrayLength(NewLines, J);

  DeleteFile(ProfilePath);
  Found := False;
  for I := 0 to J - 1 do
  begin
    SaveStringToFile(ProfilePath, NewLines[I] + #13#10, Found);
    Found := True;
  end;
end;

procedure RemoveFromPath();
var
  OldPath: string;
  Dir: string;
  NewPath: string;
  P: Integer;
begin
  Dir := ExpandConstant('{app}');
  if not RegQueryStringValue(HKCU, 'Environment', 'PATH', OldPath) then
    Exit;

  // Eliminar todas las ocurrencias de dir en el PATH
  NewPath := OldPath;
  repeat
    P := Pos(';' + Uppercase(Dir), ';' + Uppercase(NewPath));
    if P > 0 then
    begin
      // P-1 porque compensamos el ';' extra que añadimos al inicio
      Delete(NewPath, P - 1, Length(Dir) + 1);
      if (Length(NewPath) > 0) and (NewPath[1] = ';') then
        Delete(NewPath, 1, 1);
    end;
  until P = 0;

  RegWriteStringValue(HKCU, 'Environment', 'PATH', NewPath);
end;

{ ── Eventos del instalador ───────────────────────────────────────────────── }

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssDone then
  begin
    if WizardIsTaskSelected('addprofile') then
      AddToPowerShellProfile();
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  PvmHome: string;
  Answer: Integer;
begin
  if CurUninstallStep = usPostUninstall then
  begin
    RemoveFromPath();
    RemoveFromPowerShellProfile();

    PvmHome := ExpandConstant('{%USERPROFILE}') + '\.pvm';

    if DirExists(PvmHome) then
    begin
      Answer := MsgBox(
        '¿Deseas eliminar también las versiones de Python instaladas?' + #13#10 + #13#10 + PvmHome + #13#10 + #13#10 + 'Esta carpeta contiene todas las versiones descargadas por PVM.',
        mbConfirmation,
        MB_YESNO
      );
      if Answer = IDYES then
        DelTree(PvmHome, True, True, True);
    end;
  end;
end;

{ ── Página final personalizada ───────────────────────────────────────────── }

function UpdateReadyMemo(
  Space, NewLine, MemoUserInfoInfo, MemoDirInfo,
  MemoTypeInfo, MemoComponentsInfo, MemoGroupInfo,
  MemoTasksInfo: String): String;
begin
  Result :=
    MemoDirInfo + NewLine + NewLine +
    MemoTasksInfo + NewLine + NewLine +
    'PATH del usuario:' + NewLine +
    Space + ExpandConstant('{app}') + NewLine;
end;

[CustomMessages]
spanish.FinishedHeadingLabel=PVM instalado correctamente
spanish.FinishedLabel=PVM ha sido instalado en tu sistema.%n%nPara usar Python a través de PVM, ejecuta primero:%n%n    pvm use 3.12.4%n%nLuego abre una nueva terminal y comprueba:%n%n    python -V

english.FinishedHeadingLabel=PVM installed successfully
english.FinishedLabel=PVM has been installed on your system.%n%nTo use Python through PVM, first run:%n%n    pvm use 3.12.4%n%nThen open a new terminal and verify:%n%n    python -V
