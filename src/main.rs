use std::{io::stdout, process::exit};

use crossterm::{style::Stylize, terminal::size, tty::IsTty};
use scraper::{Html, Selector};
use whoami::lang;

mod cli;
mod io_functions;
mod selectors;

fn main() {
    let main_array = selectors::details();
    let mut selector_list = vec![];
    for x in &main_array {
        selector_list.push(x.0)
    }
    let args = cli::build(&selector_list).get_matches();

    if args.is_present("list") {
        selectors::print_list(main_array)
        // application will exit(0) here!
    }

    if args.is_present("clean") {
        match io_functions::clean_cache() {
            Ok(r) => {
                println!(
                    "{} The directory {} and it's contents have been removed!",
                    "success:".green().bold(),
                    r.blue()
                );
                exit(0)
            }
            Err(e) => {
                eprintln!("{} {}", "error:".red().bold(), e);
                exit(1)
            }
        }
    }

    let use_cache = args.is_present("cache");
    let query: Vec<&str> = match use_cache {
        true => vec![],
        false => args.values_of("query").unwrap().collect()
    };

    if query.len() == 1 && query[0] == "-" {
        println!(
            "{} The following required arguments were not provided:\n    {}\n\nUSAGE:\n    oi \
             <query>...\n\nFor more information try {}",
            "error:".red().bold(),
            "<query>...".green(),
            "--help".green()
        );
        exit(1)
    }

    let mut tty = stdout().is_tty();
    let tty_size = size().unwrap_or((0, 0));
    let w: usize = match tty_size.0 {
        0 if tty => panic!("main: can't determine terminal size"),
        0 => 0,
        1..=100 => tty_size.0.into(),
        _ => 100
    };

    if args.is_present("raw") {
        tty = false;
    }

    let quiet = match tty {
        true => args.is_present("quiet"),
        false => true
    };

    let html = match use_cache {
        true => match io_functions::cached_html() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{} {}", "error:".red().bold(), e);
                exit(1)
            }
        },
        false => {
            let lang = match args.is_present("language") {
                true => args.value_of("language").unwrap().to_string(),
                false => lang().next().unwrap_or_else(|| "en-US".to_string())
            };
            match io_functions::fetch(query.join(" "), lang) {
                Ok(r) => r,
                Err(_) => {
                    eprintln!("{} No response from google, sorry!", "error:".red().bold());
                    exit(1)
                }
            }
        }
    };

    if args.is_present("save") {
        match io_functions::save_html(&query, &html) {
            Ok(r) => match tty {
                true => println!(
                    "{}\n    {}\n",
                    "HTML for the query has been saved to the following path:".dark_grey(),
                    r.blue()
                ),
                false => {}
            },
            Err(e) => eprintln!("{} {}\n", "error:".red().bold(), e)
        }
    }

    let mut selectors = match args.is_present("selectors") {
        true => args.values_of("selectors").unwrap().collect(),
        false => selector_list.clone()
    };
    selectors.push("corrections");

    let data = Html::parse_document(&html);
    let mut answers = vec![];
    for x in &selectors {
        let y = match *x {
            "corrections" => selectors::name_to_id("corrections"),
            _ => {
                let p = selector_list.iter().position(|&r| r == *x).unwrap();
                main_array[p].3
            }
        };
        if data.select(&Selector::parse(y).unwrap()).next().is_some() {
            match *x == "holidays" {
                // can't find a unique id for holidays selector, this is required to prevent false positives
                true if data
                    .select(&Selector::parse("div.wDYxhc").unwrap())
                    .nth(1)
                    .unwrap()
                    .value()
                    .attr("data-attrid")
                    .unwrap()
                    == "kc:/public_events:holidays_for_date" =>
                {
                    answers.push(*x)
                }
                true => {}
                false => answers.push(*x)
            }
        }
    }

    let total = answers.len();
    match total {
        0 => {
            no_result(tty, w, data, quiet, false);
            exit(1)
        }
        1 if answers[0] == "corrections" => {
            no_result(tty, w, data, quiet, true);
            exit(1)
        }
        _ => {}
    }

    let mut corrected = false;
    if answers[(total - 1)] == "corrections" {
        corrected = true;
        if !quiet {
            corrections(&data)
        }
        answers.pop();
    }

    let matches = answers.clone();
    if !args.is_present("all") && total > 1 {
        let r_query = match corrected {
            true => data
                .select(&Selector::parse(selectors::name_to_id("corrections")).unwrap())
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .join(""),
            false => match use_cache {
                true => {
                    let x = data
                        .select(&Selector::parse("title").unwrap())
                        .next()
                        .unwrap()
                        .text()
                        .collect::<Vec<&str>>()
                        .join("");
                    let mut y = x.split(' ').collect::<Vec<&str>>();
                    for _ in 0..3 {
                        y.pop();
                    }
                    y.join(" ")
                }
                false => query.join(" ")
            }
        };
        answers = selectors::filter(answers, r_query)
    }
    selectors::print_answer(&data, answers, &tty, w, &quiet, matches);

    if args.is_present("urls") {
        print_urls(w, data)
    }
}

fn no_result(tty: bool, w: usize, data: scraper::Html, quiet: bool, corrected: bool) {
    match tty {
        true => match quiet {
            true => println!("{} Sorry about that!", "No result:".red().bold()),
            false => {
                if corrected {
                    corrections(&data)
                }
                println!(
                    "{} Perhaps one of these links might help?",
                    "No result:".bold().red()
                );
                print_urls(w, data)
            }
        },
        false => eprintln!("No result!")
    }
}

fn corrections(data: &scraper::Html) {
    let x = data
        .select(&Selector::parse(selectors::name_to_id("corrections")).unwrap())
        .next()
        .unwrap();
    let foo = x.inner_html();
    let bar = x.text().collect::<Vec<&str>>().join("");

    let html = foo.split(' ').collect::<Vec<&str>>();
    let text = bar.split(' ').collect::<Vec<&str>>();
    let total = html.len();

    assert_eq!(total, text.len());

    print!("{}", "I'll assume you meant this: ".dark_grey());
    for i in 0..total {
        match html[i] == text[i] {
            true => print!("{} ", text[i]),
            false => print!("{} ", text[i].bold().cyan())
        }
    }
    println!()
}

fn print_urls(w: usize, data: scraper::Html) {
    match data
        .select(&Selector::parse(selectors::name_to_id("url_block")).unwrap())
        .next()
        .is_some()
    {
        true => {
            for x in data.select(&Selector::parse(selectors::name_to_id("url_block")).unwrap()) {
                let title = x
                    .select(&Selector::parse(selectors::name_to_id("title")).unwrap())
                    .next()
                    .unwrap()
                    .text()
                    .collect::<Vec<&str>>();

                let url = x
                    .select(&Selector::parse(selectors::name_to_id("url")).unwrap())
                    .next()
                    .unwrap()
                    .first_child()
                    .unwrap()
                    .value()
                    .as_element()
                    .unwrap()
                    .attr("href")
                    .unwrap();

                let desc_check = x
                    .select(&Selector::parse(selectors::name_to_id("desc")).unwrap())
                    .next();
                let description: String = match desc_check.is_some() {
                    true => {
                        let y = desc_check.unwrap().text().collect::<Vec<&str>>();
                        y.join("")
                    }
                    false => "No description available, sorry!".to_string()
                };

                println!(
                    "\n{}\n{}\n{}",
                    title.join("").bold().blue(),
                    url,
                    format_desc(w, description).dark_grey()
                )
            }
        }
        false => println!("{}", "jk, there are no links!".dark_grey())
    }
    exit(0)
}

fn format_desc(length_max: usize, desc: String) -> String {
    if desc.len() < length_max {
        return desc
    }
    let mut length = 0;
    let mut desc_build = vec![];
    let mut r: Vec<String> = vec![];

    let desc_words: Vec<&str> = desc.split(' ').collect();
    for x in &desc_words {
        match (x.len() + length) >= length_max {
            true => {
                r.push(desc_build.join(" "));
                desc_build.clear();
                length = x.len() + 1
            }
            false => length += x.len() + 1
        }
        desc_build.push(*x)
    }
    r.push(desc_build.join(" "));
    r.join("\n")
}
