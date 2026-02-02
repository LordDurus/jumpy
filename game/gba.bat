@echo off
cls
setlocal enabledelayedexpansion

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
