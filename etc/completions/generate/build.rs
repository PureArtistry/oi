use std::{env, io::Error};

use clap_generate::{
    generate_to,
    generators::{Bash, Fish, PowerShell, Zsh}
};

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let selectors = [
        "basic1",
        "basic2",
        "clock",
        "conversions",
        "currency",
        "define",
        "holidays",
        "lists",
        "lyrics",
        "maths",
        "pronounce",
        "snippets1",
        "snippets2",
        "sports",
        "summary",
        "translate",
        "weather"
    ];
    let mut app = build(&selectors);
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir
    };

    let bash = generate_to(Bash, &mut app, "oi", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", bash);

    let fish = generate_to(Fish, &mut app, "oi", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", fish);

    let powershell = generate_to(PowerShell, &mut app, "oi", &outdir)?;
    println!(
        "cargo:warning=completion file is generated: {:?}",
        powershell
    );

    let zsh = generate_to(Zsh, &mut app, "oi", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", zsh);

    Ok(())
}
