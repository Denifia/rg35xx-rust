# rg35xx-rust

Example of a simple rust app developed on windows and cross compiled to run on a RG35XX.

## Prerequsites

- [Install WSL v2](https://learn.microsoft.com/en-us/windows/wsl/install)
- [Install Docker Desktop](https://docs.docker.com/desktop/install/windows-install/)
- [Turn on Docker Desktop WSL 2](https://docs.docker.com/desktop/windows/wsl/#turn-on-docker-desktop-wsl-2)
- [Enable Docker support in WSL 2](https://docs.docker.com/desktop/windows/wsl/#enabling-docker-support-in-wsl-2-distros)
- [Download adb](https://developer.android.com/tools/releases/platform-tools)
- In your WSL Terminal
  - Install build-essential
    ```
    sudo apt install build-essential
    ```
  - [Install Rust](https://www.rust-lang.org/tools/install) (I used the default install)
  - [Install cross](https://github.com/cross-rs/cross)
- Clone this repo
- Optional 
  - [VS Code](https://code.visualstudio.com/download)

## Develop

In windows, modify the files in this-repo (I use VS Code)

## Build

In your WSL Terminal

```bash
# Navigate to where you cloned this repo
# e.g. "cd /mnt/c/Users/your-name/rg35xx-rust"

# Build the Rust app
cross build --target armv7-unknown-linux-musleabihf
```

## Deploy

In your Windows Terminal

```powershell
# Navigate to where you cloned this repo
# e.g. "cd c:\Users\your-name\rg35xx-rust"

# Navigate to the built app
cd target\armv7-unknown-linux-musleabihf\debug

# Copy it to the device
path/to/adb.exe push --sync .\hellorust "/mnt/mmc/hellorust"
```

## Run

In your Windows Terminal
```powershell
# Open a shell on your device
path/to/adb.exe shell

# Run the app
/mnt/mmc/hellorust
```

## Acknowledgements

- Easy to digest blog post [Reproducible cross-compilation for Rust (with Docker)](https://kerkour.com/rust-reproducible-cross-compilation-with-docker) got me most of the way
- [Hello World](https://doc.rust-lang.org/rust-by-example/hello.html) example in the [official Rust docs](https://www.rust-lang.org/learn)
- Learned a lot from the [Cross compiling for arm or aarch64 on Debian or Ubuntu](https://www.youtube.com/watch?v=SoGsKI_nrYU) video on YouTube and associated [blog post](https://jensd.be/1126/linux/cross-compiling-for-arm-or-aarch64-on-debian-or-ubuntu)
- [Drone](https://www.drone.io/) has a [tutorial for another approach](https://github.com/drone/tutorials/blob/master/content/rust/docker/rust-docker-arm64.md)
