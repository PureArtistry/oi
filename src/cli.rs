use clap::{App, Arg};

pub fn build<'a>(selectors: &'a [&str]) -> App<'a> {
    App::new("oi!")
        .author("David Sherriff <d.sherriff.81@gmail.com>")
        .about("please use --help for more detailed information")
        .long_about("trivia on the command line.")
        .version("0.2.0")
        .help_template("{bin}\n{about}\n\n{usage-heading}\n    {usage}\n\n{all-args}\n\nversion {version} by {author}\nplease report any bugs to https://gitub.com/PureArtistry/oi/issues")

        .arg(Arg::new("all")
            .short('a')
            .long("all")
            .display_order(1)
            .help("Prints all of the answers found")
        )

        .arg(Arg::new("urls")
            .short('u')
            .long("urls")
            .display_order(2)
            .help("Also print a list of the top urls associated with your query")
            .long_help(
                "Also print a list of the top urls associated with your query\n\
                these are typically only shown when an answer can't be found"
            )
        )

        .arg(Arg::new("quiet")
            .short('q')
            .long("quiet")
            .display_order(3)
            .help("Only print the answer (if applicable) and error messages")
            .long_help(
                "Only print the answer (if applicable) and error messages\n\
                silences corrections, unrequested urls and selector information"
            )
        )

        .arg(Arg::new("raw")
            .short('r')
            .long("raw")
            .display_order(4)
            .help("Raw output (use --help for details)")
            .long_help(
                "Raw output - no colours, terminal attributes and messages\n\
                this is only required if you don't want to use colours etc in your terminal\n\
                if you are piping the output somewhere else this flag is passed automatically"
            )
        )

        .arg(Arg::new("save")
            .short('s')
            .long("save")
            .display_order(5)
            .conflicts_with("cache")
            .help("Saves the raw HTML for this query")
            .long_help(
                "Saves the raw HTML for this query to the following path:\n\
                (BSD/Linux) $HOME/.cache/oi/[date]-[query].html\n\
                (MacOS)     $HOME/Library/Application Support/oi/[date]-[query].html\n\
                (Windows)   %LOCALAPPDATA%\\oi\\[date]-[query].html"
            )
        )

        .arg(Arg::new("cache")
            .short('c')
            .long("cache")
            .display_order(6)
            .conflicts_with("language")
            .help("Use the most recent cached HTML")
        )

        .arg(Arg::new("clean")
            .long("clean")
            .exclusive(true)
            .display_order(7)
            .help("Remove all previously saved results")
        )

        .arg(Arg::new("list")
            .short('L')
            .long("list")
            .exclusive(true)
            .display_order(8)
            .help("Prints a table of all the valid answer selectors")
            .long_help(
                "Prints a table of all the valid answer selectors\n\
                includes descriptions and examples (for use with the -p --pick option)"
            )
        )

        .arg(Arg::new("language")
            .short('l')
            .long("lang")
            .takes_value(true)
            .display_order(9)
            .conflicts_with("cache")
            .help("Specify the language to use (eg: en-GB)")
            .long_help(
                "Specify the language to use (eg: en-GB)\n\
                oi uses your system language by default\n\
                if that can't be resolved then it will default to en-US"
            )
            .value_hint(clap::ValueHint::Other)
        )

        .arg(Arg::new("selectors")
            .short('p')
            .long("pick")
            .takes_value(true)
            .multiple_values(true)
            .value_terminator("--")
            .possible_values(selectors)
            .hide_possible_values(true)
            .display_order(10)
            .help("Target specific answers, use -- to stop parsing arguments")
            .long_help(
                "Target specific answers, use -- to stop parsing arguments\n\
                eg: oi -p basic1 basic2 summary -- my search query"
            )
        )

        .arg(Arg::new("query")
            .conflicts_with_all(&["cache", "clean", "list"])
            .help("Whaddya wanna know?")
            .required_unless_present_any(&["cache", "clean", "list"])
            .multiple_values(true)
            .value_hint(clap::ValueHint::Other)
        )
}
