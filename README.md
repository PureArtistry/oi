# oi
trivia on the command line

full docs coming soon, use ```oi --help``` for full usage information

### dependencies

on all platforms the only thing you need installed is cargo.

+ Windows: https://win.rustup.rs/
+ Linux/macOS: ```curl https://sh.rustup.rs -sSf | sh``` (or use your distro's package manager)

this is only required for building the binary, if you have no more use for cargo you can remove it afterwards

### build instructions:

clone repo

cd into repo root

run ```cargo build --release```

then for convenience copy ```./target/release/oi``` to somewhere in your $PATH
(on linux this would be something like ```/usr/local/bin```, not sure what location to use for other systems)

I'll be providing pre compiled binaries for Linux, Windows & macOS in the releases section soon

### TODO:

+ full docs
+ shell completion scripts
+ user customisable colours (using an environment variable)
+ general code improvements (and probable bug fixes)
