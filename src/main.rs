use clap::{crate_authors, crate_description, crate_version, App, Arg};
use crossterm::style::{Colorize, Styler};
use crossterm::tty::IsTty;
use scraper::{Html, Selector};
use std::env;
use std::io::stdout;
use std::process::exit;

mod selectors;
mod stuff;

fn main() {
    let mut tty = stdout().is_tty();
    let main_array = selectors::details();
    let mut selectors = vec![];
    for i in 0..main_array.len() {
        selectors.push(main_array[i].0)
    }
    let args = App::new("oi!")
        .author(crate_authors!())
        .about("please use --help for more detailed information")
        .long_about(crate_description!())
        .version(crate_version!())
        .help_template("{bin} - {about}\n\nUSAGE:\n    {usage}\n\n{all-args}\n\nversion {version} by {author}\nplease report any bugs to https://gitub.com/PureArtistry/oi/issues")
        .arg(Arg::new("all")
            .short('a')
            .long("all")
            .display_order(1)
            .about("Prints all of the answers found"))
        .arg(Arg::new("urls")
            .short('u')
            .long("urls")
            .display_order(2)
            .about("Also print a list of the top urls associated with your query")
            .long_about(
"Also print a list of the top urls associated with your query
these are typically only shown when an answer can't be found"))
        .arg(Arg::new("quiet")
            .short('q')
            .long("quiet")
            .display_order(3)
            .about("Only print the answer (if applicable) and error messages")
            .long_about(
"Only print the answer (if applicable) and error messages
silences corrections, unrequested urls and selector information"))
        .arg(Arg::new("raw")
            .short('r')
            .long("raw")
            .display_order(4)
            .about("Raw output (use --help for details)")
            .long_about(
"Raw output - no colours, terminal attributes and messages
this is only required if you don't want to use colours etc in your terminal
if you are piping the output somewhere else this flag is passed automatically"))
        .arg(Arg::new("save")
            .short('s')
            .long("save")
            .display_order(5)
            .conflicts_with("cache")
            .about("Saves the raw HTML for this query (linux only atm, sorry!)")
            .long_about(
"Saves the raw HTML for this query to the following path:
$HOME/.cache/oi/[date]-[query].html
As you might be able to infer from the path, this flag is only for linux right now
I plan to update this to also use the correct path on Windows and macOS soon"))
        .arg(Arg::new("cache")
            .short('c')
            .long("cache")
            .display_order(6)
            .conflicts_with("language")
            .about("Use the most recent cached HTML"))
        .arg(Arg::new("list")
            .short('L')
            .long("list")
            .exclusive(true)
            .display_order(7)
            .about("Prints a table of all the valid answer selectors")
            .long_about(
"Prints a table of all the valid answer selectors
includes descriptions and examples (for use with the -p --pick option)"))
        .arg(Arg::new("language")
            .short('l')
            .long("lang")
            .takes_value(true)
            .display_order(8)
            .conflicts_with("cache")
            .about("Specify the language to use (eg: en_GB)")
            .long_about(
"Specify the language to use (eg: en_GB)
oi uses your system language by default
if that can't be resolved then it will default to en_US"))
        .arg(Arg::new("selectors")
            .short('p')
            .long("pick")
            .takes_value(true)
            .multiple(true)
            .value_terminator("--")
            .possible_values(&selectors)
            .hide_possible_values(true)
            .display_order(9)
            .about("Target specific answers, use -- to stop parsing arguments")
            .long_about(
"Target specific answers, use -- to stop parsing arguments
eg: oi -p simple_values basic_answers -- my search query"))
        .arg(Arg::new("query")
            .conflicts_with("cache")
            .conflicts_with("list")
            .about("Whaddya wanna know?")
            .required_unless_present("cache")
            .required_unless_present("list")
            .multiple(true))
        .get_matches();

    if args.is_present("list") {
        selectors::print_list(main_array)
    }

    if args.is_present("raw") {
        tty = false;
    }

    let quiet = match tty {
        true => args.is_present("quiet"),
        false => true,
    };

    let use_cache = args.is_present("cache");

    let query: Vec<&str> = match use_cache {
        true => vec![],
        false => args.values_of("query").unwrap().collect(),
    };
    if query.len() == 1 {
        if query[0] == "-" {
            println!(
                "{} The following required arguments were not provided:
    {}

USAGE:
    oi <query>...

For more information try {}",
                "error:".dark_red().bold(),
                "<query>...".dark_green(),
                "--help".dark_green()
            );
            exit(1)
        }
    }

    let os_lang = env::var("LANG").unwrap_or("en_US".to_string());
    let lang_split: Vec<&str> = os_lang.split('.').collect();
    let lang = args.value_of("language").unwrap_or(lang_split[0]);

    let html = match use_cache {
        true => stuff::cached_html(&tty),
        false => match stuff::fetch(query.join(" ").to_string(), lang) {
            Ok(x) => x,
            Err(_) => {
                match tty {
                    true => eprintln!(
                        "{} No response from google, sorry!",
                        "error:".dark_red().bold()
                    ),
                    false => eprintln!("error: No response from google, sorry!"),
                };
                exit(1)
            }
        },
    };

    if args.is_present("save") {
        stuff::save_html(&query, &html, &tty)
    }

    if args.is_present("selectors") {
        selectors = args.values_of("selectors").unwrap().collect()
    }

    if quiet == false {
        selectors.push("corrections")
    }

    let data = Html::parse_document(&html);
    let mut answers = vec![];
    for i in 0..selectors.len() {
        if data
            .select(&Selector::parse(selectors::name_to_id(selectors[i])).unwrap())
            .next()
            .is_some()
        {
            answers.push(selectors[i])
        }
    }

    match answers.len() {
        0 => no_result(tty, &data, quiet),
        1 => {
            if &answers[0] == &"corrections" {
                no_result(tty, &data, quiet)
            }
        }
        _ => {}
    }

    if &answers[(answers.len() - 1)] == &"corrections" {
        corrections(&data);
        answers.pop();
    }

    let total = answers.clone();
    match args.is_present("all") {
        true => selectors::print_answer(&data, answers, &tty, &quiet, total),
        false => {
            if answers.len() > 1 {
                answers = selectors::filter(answers)
            }
            selectors::print_answer(&data, answers, &tty, &quiet, total)
        }
    }

    if args.is_present("urls") {
        print_urls(&data)
    }
}

fn no_result(tty: bool, data: &scraper::Html, quiet: bool) {
    match tty {
        true => match quiet {
            true => println!("{} Sorry about that!", "No result:".dark_red().bold()),
            false => {
                println!(
                    "{} Perhaps one of these links might help?",
                    "No result:".bold().dark_red()
                );
                print_urls(data)
            }
        },
        false => eprintln!("No result!"),
    }
    exit(1)
}

fn corrections(data: &scraper::Html) {
    let x = data
        .select(&Selector::parse(selectors::name_to_id("corrections")).unwrap())
        .next()
        .unwrap();
    let y1 = x.inner_html();
    let y: Vec<&str> = y1.split(" ").collect();
    let j = y.len();
    let z1: Vec<&str> = x.text().collect();
    let z2 = z1.join("");
    let z: Vec<&str> = z2.split(" ").collect();

    if j != z.len() {
        panic!()
    }

    print!("{}", "I'll assume you meant this: ".grey());
    for i in 0..j {
        if y[i] == z[i] {
            print!("{} ", z[i])
        } else {
            print!("{} ", z[i].bold().dark_cyan())
        }
    }
    print!("\n")
}

fn print_urls(data: &scraper::Html) {
    let mut titles: Vec<&str> = vec![];
    let mut urls: Vec<&str> = vec![];
    for x in data.select(&Selector::parse("h3.LC20lb.DKV0Md").unwrap()) {
        titles.push(x.text().next().unwrap())
    }
    for x in data.select(&Selector::parse("div.yuRUbf").unwrap()) {
        urls.push(
            x.first_child()
                .unwrap()
                .value()
                .as_element()
                .unwrap()
                .attr("href")
                .unwrap(),
        )
    }
    let j = titles.len();
    if urls.len() != j {
        panic!()
    } else if j == 0 {
        println!("{}", "jk, there are no links!".grey());
        exit(0)
    }
    for i in 0..j {
        print!("\n");
        println!("{}", titles[i].bold().dark_blue());
        println!("{}", urls[i]);
    }
    exit(0)
}
