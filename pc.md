# pc setup

this doc covers what you need installed to build and run the pc version of jumpy on windows/mac/linux.

## quick checklist

- rust toolchain installed
- sdl2 runtime + dev headers
- sdl2_image, sdl2_ttf, sdl2_mixer
- audio codec libs for sdl2_mixer (ogg/vorbis at minimum)
- build tools for native deps (compiler + make/cmake depending on platform)

## rust

install rust using rustup:

- windows/mac/linux: install via rustup (recommended)

verify:

- `rustc --version`
- `cargo --version`

## platform setup

### windows

#### option a: msys2 (recommended)

1) install msys2  
2) open “msys2 mingw64” shell  
3) install dependencies:

- `pacman -Syu`
- restart shell if msys2 tells you to
- then:

- `pacman -S --needed mingw-w64-x86_64-toolchain`
- `pacman -S --needed mingw-w64-x86_64-SDL2`
- `pacman -S --needed mingw-w64-x86_64-SDL2_image`
- `pacman -S --needed mingw-w64-x86_64-SDL2_ttf`
- `pacman -S --needed mingw-w64-x86_64-SDL2_mixer`

4) make sure your build environment can find the mingw64 binaries and dlls  
typically this means your PATH includes:

- `C:\msys64\mingw64\bin`

verify from the same shell:

- `sdl2-config --version` (if available)
- or confirm `SDL2.dll` exists in `mingw64\bin`

#### option b: vcpkg

this works too, but is more fiddly because you need to make sure cargo finds the vcpkg libs consistently.

high level steps:

- install visual studio build tools (c++ workload)
- install vcpkg
- `vcpkg install sdl2 sdl2-image sdl2-ttf sdl2-mixer`
- set env vars so `sdl2-sys` can find vcpkg (depends on how your rust sdl crate is configured)

env vars:
SDL2_INCLUDE_DIR = C:\libs\SDL2-2.30.11\include
SDL2_LIB_DIR = C:\libs\SDL2-2.30.11\lib\x64

VCPKG_DEFAULT_TRIPLET = x64-windows
VCPKG_ROOT = C:\dev\vcpkg
VCPKGRS_DYNAMIC = 1

### macos

install via homebrew:

- `brew install sdl2 sdl2_image sdl2_ttf sdl2_mixer`

note:
- sdl2_mixer pulls in codec deps, but if ogg playback fails, make sure `libogg`/`libvorbis` are installed (brew normally handles it).

### linux (debian/ubuntu)

install dev packages:

- `sudo apt-get update`
- `sudo apt-get install -y build-essential pkg-config`
- `sudo apt-get install -y libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev libsdl2-mixer-dev`

if your distro splits codec plugins, also install:

- `libogg-dev libvorbis-dev`

### linux (fedora)

- `sudo dnf install -y @development-tools pkgconf-pkg-config`
- `sudo dnf install -y SDL2-devel SDL2_image-devel SDL2_ttf-devel SDL2_mixer-devel`

## building and running

from repo root:

- debug:
- `cargo run --features pc`

- release:
- `cargo run --release --features pc`

if the project uses a different feature name or default features change, update the commands above.

## common problems

### missing dll / shared library at runtime

symptom:
- windows: “SDL2.dll was not found”
- linux: “error while loading shared libraries: libSDL2…”

fix:
- windows: ensure the folder containing `SDL2.dll` is in PATH, or copy required dlls next to the exe
- linux: install runtime libs (non `-dev` packages) or fix library path
- mac: brew install usually resolves

### sdl2_mixer loads but ogg music won’t play

fix:
- confirm ogg/vorbis libs are installed (see platform sections)
- verify your audio files are valid ogg/wav
- for debugging, print the exact mixer error string when load/play fails

### build fails finding sdl2 headers / pkg-config

fix:
- linux: install `pkg-config` (or `pkgconf`) and the `-dev` packages listed above
- mac: make sure homebrew is in PATH
- windows (msys2): build from the mingw64 shell or ensure your environment points at mingw64

## repo conventions

- pc uses the `pc` cargo feature
- assets are loaded from the project’s assets root (see `crate::assets`)
- supported audio formats for pc: wav + ogg
- prefer adding new native deps via the platform package managers above, not by committing binaries into the repo
