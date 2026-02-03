@echo off
cls
setlocal enabledelayedexpansion

rem === prepare assets ===
del /f /q ..\assets\gfx\gba\compiled\*.*
rem C:\devkitPro\tools\bin\grit.exe ..\assets\gfx\gba\background\bg_parallax_forest.png -gB4 -gt -m -mp1 -pn16 -o ..\assets\gfx\gba\compiled\bg_parallax_forest
rem C:\devkitPro\tools\bin\grit.exe ..\assets\gfx\gba\background\bg_parallax_forest.png -gB4 -gt -m -p -pe16 -ftb -fh! -o ..\assets\gfx\gba\compiled\bg_parallax_forest
rem C:\devkitPro\tools\bin\grit.exe ..\assets\gfx\gba\background\bg_parallax_forest_256.png -gB8 -gt -m -p -pe256 -ftb -o ..\assets\gfx\gba\compiled\bg_parallax_forest
C:\devkitPro\tools\bin\grit.exe ..\assets\gfx\gba\background\bg_parallax_forest_256.png -gB8 -gt -m -p -pe256 -pn256 -ftb -o ..\assets\gfx\gba\compiled\bg_parallax_forest

rem echo === build ===
del target\thumbv4t-none-eabi\debug\jumpy
cargo +nightly build -Z build-std=core,alloc --no-default-features --features gba --target thumbv4t-none-eabi
rem arm-none-eabi-readelf -h target\thumbv4t-none-eabi\debug\jumpy
rem arm-none-eabi-readelf -l target\thumbv4t-none-eabi\debug\jumpy

rem === verify elf exists ===
if not exist target\thumbv4t-none-eabi\debug\jumpy (
	echo missing elf: target\thumbv4t-none-eabi\debug\jumpy
	exit /b 1
)

rem dir target\thumbv4t-none-eabi\debug\jumpy

where arm-none-eabi-objcopy
if errorlevel 1 (
	echo arm-none-eabi-objcopy not found in PATH
	exit /b 1
)

echo.
echo === elf -> gba ===
del /f /q target\thumbv4t-none-eabi\debug\jumpy.gba 2>nul

rem arm-none-eabi-objcopy -O binary target\thumbv4t-none-eabi\debug\jumpy target\thumbv4t-none-eabi\debug\jumpy.gba

arm-none-eabi-objcopy --strip-all target\thumbv4t-none-eabi\debug\jumpy target\thumbv4t-none-eabi\debug\jumpy_stripped.elf
arm-none-eabi-objcopy -O binary target\thumbv4t-none-eabi\debug\jumpy_stripped.elf target\thumbv4t-none-eabi\debug\jumpy.gba
rem dir target\thumbv4t-none-eabi\debug\jumpy.gba



rem echo === output size ===
rem dir target\thumbv4t-none-eabi\debug\jumpy.gba

mgba-qt target\thumbv4t-none-eabi\debug\jumpy.gba
