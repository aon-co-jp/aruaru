# aruaru-readme-auto-rs full bug check
# ASCII-only PowerShell source to avoid mojibake on Windows PowerShell 5.1.
$ErrorActionPreference = "Stop"

# Always run from project root even when launched from the scripts directory.
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptDir "..")
Set-Location $ProjectRoot


function Step($name) {
  Write-Host ""
  Write-Host "== $name =="
}


function Invoke-Native($FilePath, [string[]]$Arguments) {
  & $FilePath @Arguments
  if ($LASTEXITCODE -ne 0) {
    throw "$FilePath failed with exit code $LASTEXITCODE"
  }
}

function Stop-App($proc) {
  if ($null -ne $proc -and -not $proc.HasExited) {
    Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
  }
}

Step "aruaru-readme-auto-rs full bug check"

Invoke-Native cargo @("fmt", "--all")
Invoke-Native cargo @("fmt", "--all", "--", "--check")
Invoke-Native cargo @("check")
Invoke-Native cargo @("test")
Invoke-Native cargo @("test", "development_menu")
Invoke-Native cargo @("test", "programming_language_info")
Invoke-Native cargo @("test", "language_update_job")
Invoke-Native cargo @("test", "language_top100")
Invoke-Native cargo @("test", "business_top100")
Invoke-Native cargo @("test", "car_top100")
Invoke-Native cargo @("test", "insurance_top100")
Invoke-Native cargo @("test", "opencuda_035_reference")
Invoke-Native cargo @("test", "opencuda_core_source_review")
Invoke-Native cargo @("test", "opencuda_examples_tools_review")
Invoke-Native cargo @("clippy", "--all-targets", "--", "-D", "warnings")

Step "banned feature check"
$hits = Select-String -Path .\src\*.rs,.\Cargo.toml -Pattern "tauri","REST API","rest api" -CaseSensitive:$false
if ($hits) {
  Write-Host "NG: Tauri / REST API related text found in implementation files."
  $hits
  exit 1
}

Step "generation test"
Remove-Item -Recurse -Force .\tmp-web -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force .\tmp-web | Out-Null
Set-Content .\tmp-web\README.md "# TEST`n`n- item 1`n- item 2" -Encoding UTF8

$arg = @(
  "run",
  "--",
  "--root", ".\tmp-web",
  "--listen", "127.0.0.1:7878",
  "--output", "both",
  "--extra-outputs", "php,python,ts,json",
  "--scan-interval-secs", "1",
  "--stable-wait-millis", "500"
)

$proc = $null
try {
  $stdoutLog = Join-Path $ProjectRoot "tmp-web\cargo-run.stdout.log"
  $stderrLog = Join-Path $ProjectRoot "tmp-web\cargo-run.stderr.log"
  $proc = Start-Process cargo -ArgumentList $arg -PassThru -WindowStyle Hidden -RedirectStandardOutput $stdoutLog -RedirectStandardError $stderrLog
  Start-Sleep -Seconds 8

  if ($proc.HasExited) {
    Write-Host "cargo run exited during generation test."
    if (Test-Path $stdoutLog) { Get-Content $stdoutLog -Raw }
    if (Test-Path $stderrLog) { Get-Content $stderrLog -Raw }
    throw "cargo run exited before README generation"
  }

  if (!(Test-Path .\tmp-web\README.rs)) {
    if (Test-Path $stdoutLog) { Get-Content $stdoutLog -Raw }
    if (Test-Path $stderrLog) { Get-Content $stderrLog -Raw }
    throw "README.rs was not generated"
  }
  if (!(Test-Path .\tmp-web\README.html)) { throw "README.html was not generated" }
  if (!(Test-Path .\tmp-web\README.php)) { throw "README.php was not generated" }
  if (!(Test-Path .\tmp-web\README.py)) { throw "README.py was not generated" }
  if (!(Test-Path .\tmp-web\README.ts)) { throw "README.ts was not generated" }
  if (!(Test-Path .\tmp-web\README.json)) { throw "README.json was not generated" }

  # Japanese text generated from UTF-8 bytes so this script stays ASCII-only.
  $jpTitle = [System.Text.Encoding]::UTF8.GetString([byte[]](0xE6,0x97,0xA5,0xE6,0x9C,0xAC,0xE8,0xAA,0x9E,0xE3,0x83,0x86,0xE3,0x82,0xB9,0xE3,0x83,0x88))
  $jpBody  = [System.Text.Encoding]::UTF8.GetString([byte[]](0xE3,0x81,0x93,0xE3,0x82,0x93,0xE3,0x81,0xAB,0xE3,0x81,0xA1,0xE3,0x81,0xAF,0xE3,0x80,0x82))

  Set-Content .\tmp-web\README.md ("# " + $jpTitle + "`n`n" + $jpBody + " English conversation and programming learning.") -Encoding UTF8
  Start-Sleep -Seconds 8
  $html = Get-Content .\tmp-web\README.html -Raw -Encoding UTF8
  if (-not $html.Contains($jpTitle)) { throw "UTF-8 Japanese title was not found in README.html" }

  Set-Content .\tmp-web\README.md "# XSS TEST`n`n<script>alert(1)</script>" -Encoding UTF8
  Start-Sleep -Seconds 8
  $html = Get-Content .\tmp-web\README.html -Raw -Encoding UTF8
  if ($html -match "<script>alert") { throw "Unsafe script remained in README.html" }
}
finally {
  Stop-App $proc
}

Step "path check"
Test-Path .\tmp-web\README.rs
Test-Path .\tmp-web\README.html
Test-Path .\tmp-web\README.php
Test-Path .\tmp-web\README.py
Test-Path .\tmp-web\README.ts
Test-Path .\tmp-web\README.json

Write-Host ""
Write-Host "OK: full bug check passed"
