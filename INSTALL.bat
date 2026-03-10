@echo off
setlocal

:: Change directory to WICS/WICS/ from WICS/
cd /d "%~dp0WICS"

:: Check if Node.js is already installed (if node -v works)
set NODE_VERSION =
for /f "tokens=*" %%i in ('node -v 2^>nul') do set NODE_VERSION=%%i

:: Check if NODE_VERSION was rly set
if defined NODE_VERSION (
    echo ✅ Node.js is installed. Version: %NODE_VERSION%
) else (
    echo ❗Node.js required by WICS is not installed or not in PATH.
    set /p input="Do you want to install it? [y]/[n]: "

    if /i "%input%"=="y" (
        echo Attempting to install via winget...
        call winget install -e --id OpenJS.NodeJS.LTS
    ) else (
        echo Installation skipped. WICS cannot run without Node.js. If you want to proceed with it's installation, please install Node.js manually and run INSTALL.bat again afterwards.
        pause
        exit /b 1
    )
)

:: Check if npm is installed and working
call npm -v >nul 2>&1
if %errorlevel% EQU 0 (
    echo ✅ npm installed and working.
) else (
    echo npm is missing! This usually means Node.js installation is broken.
    echo Attempting to repair Node.js...
    call winget install --upgrade --silent OpenJS.NodeJS.LTS

    :: Final check
    call npm -v >nul 2>&1
    if %errorlevel% NEQ 0 (
        echo ❌ npm repair failed. Please reinstall Node.js manually.
        pause
        exit /b 1
    )
)

:: Install dependencies
echo 📦 Installing dependencies...
call npm install
if %errorlevel% NEQ 0 (
    echo ❗Dependencies installation failed!
    pause
    exit /b 1
)

:: Build project
echo 🏗️ Building project...
call npm run build
if %errorlevel% NEQ 0 (
    echo ❗Building failed!
    pause
    exit /b 1
)

:: Link
echo ⛓️‍💥 Trying to link...
call npm link
if %errorlevel% NEQ 0 (
    echo ❗Global linking failed! This was likely caues by lack of privileges.
    echo This won't cause WICS not to work, but you'll need to run it manually.
    echo To do this: open your terminal and type in:
    echo     node "%%~dp0/WICS/dist/index.js"
)

echo ✅ WICS setup completed succesfully!