@echo off
setlocal

echo 🧹 Starting WICS uninstallation...

:: Unlinking global command
echo Removing global symlink...
call npm unlink -g

echo ⚠️ This script has no way of deleting files of WICS, it did it's job now it's time for you!
echo To proceed with uninstallation delete the directory (folder) containing this script.
pause