```
      ▪  ▄▄ 
▪     ██ ██▌
 ▄█▀▄ ▐█·▐█·
▐█▌.▐▌▐█▌.▀ 
 ▀█▄▀▪▀▀▀ ▀  - trivia on the command line
```
#### Documentation is my...

![define_ah](https://user-images.githubusercontent.com/53883649/146692265-ec042204-7cbd-4918-89fa-10d47d1bf621.jpg)

use ```oi --help``` for full usage information

## dependencies:

on all platforms the only thing you need installed is cargo.

+ Windows: https://win.rustup.rs/
+ Linux/macOS: ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` (or use your distro's package manager)

this is only required for building the binary, if you have no more use for cargo you can remove it afterwards

## build instructions:

clone repo

cd into repo root

#### Linux
an install script has been provided, just run ```./install.sh```

#### macOS
run ```cargo build --release --frozen```

then ```sudo cp ./target/release/oi /usr/local/bin```

a zsh completion script is located at the following path ```./etc/completions/_oi``` but I currently have no idea where to put it (sorry!)

#### Windows
run ```cargo build --release --frozen```

then make a new folder in a location of your choosing

```copy .\target\release\oi.exe [letter]:\path\to\your\folder```

following [this](https://medium.com/@kevinmarkvi/how-to-add-executables-to-your-path-in-windows-5ffa4ce61a53) guide you can add your new folder to you environment PATH

a PowerShell completion script is located at the following path ```.\etc\completions\_oi.ps1``` but I currently have no idea where to put it (sorry!)

##### I'll be providing pre compiled binaries for Linux, Windows & macOS in the releases section soon

### TODO:

- [x] shell completion scripts
- [ ] add a proper release with binaries
- [ ] license?
- [ ] improve docs
- [ ] user customisable colours (using an environment variable)
- [ ] general code improvements (and probable bug fixes)
