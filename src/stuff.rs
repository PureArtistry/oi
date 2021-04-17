use chrono::prelude::Local;
use crossterm::style::{Colorize, Styler};
use glob::glob;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::{env, fs};

pub fn fetch(query: String, lang: &str) -> Result<String, ureq::Error> {
    let r = ureq::get("https://google.com/search")
        .set(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:53.0) Gecko/20100101 Firefox/53.0",
        )
        .query("q", &query)
        .query("hl", lang)
        .call()?
        .into_string()
        .unwrap();
    Ok(r)
}

pub fn cached_html(tty: &bool) -> String {
    let cache_path = vec![
        get_cache_path(tty),
        "/oi".to_string(),
        "/*.html".to_string(),
    ]
    .join("")
    .to_string();
    let mut files: Vec<String> = vec![];
    for x in glob(&cache_path).unwrap() {
        match x {
            Ok(x) => files.push(x.display().to_string()),
            Err(_) => panic!(),
        }
    }
    if files.len() == 0 {
        match tty {
            true => eprintln!(
                "{} Can't find any cached results, sorry!",
                "error:".dark_red().bold()
            ),
            false => eprintln!("error: Can't find any cached results, sorry!"),
        }
        exit(1)
    }
    files.reverse();
    let html = match fs::read_to_string(&files[0]) {
        Ok(x) => x,
        Err(_) => {
            match tty {
                true => eprintln!(
                    "{} Can't read from {}\nnot sure what happened, sorry!",
                    "error:".dark_red().bold(),
                    files[0].to_string().dark_green()
                ),
                false => eprintln!(
                    "error: Can't read from {}\nnot sure what appened, sorry!",
                    files[0]
                ),
            }
            exit(1)
        }
    };
    html
}

//TODO: probably ought to make this compatable with both windows and mac too
pub fn save_html(query: &Vec<&str>, html: &String, tty: &bool) {
    let cache_path = get_cache_path(&tty);
    let file_date = Local::now().format("%s").to_string();
    let file_query = query.join("_").to_string();
    let mut x: Vec<&str> = vec![&cache_path, "/oi"];
    if !Path::new(&x.join("").to_string()).is_dir() {
        if !fs::create_dir(&x.join("").to_string()).is_ok() {
            match tty {
                true => eprintln!(
                    "{} Can't create directory {}\nNot sure what went wrong, sorry!",
                    "error:".dark_red().bold(),
                    &x.join("").to_string().dark_green()
                ),
                false => eprintln!(
                    "error: Can't create directory {}\nNot sure what went wrong, sorry!",
                    &x.join("").to_string()
                ),
            };
            exit(1)
        };
    };
    x.push("/");
    x.push(&file_date);
    x.push("-");
    x.push(&file_query);
    x.push(".html");
    match write_file(&x.join("").to_string(), &html) {
        Ok(_) => match tty {
            true => println!(
                "{}\n    {}\n",
                "HTML for the query has been saved to the following path:".grey(),
                &x.join("").to_string().dark_blue()
            ),
            false => println!(
                "HTML for the query has been saved to the following path\n    {}\n",
                &x.join("").to_string()
            ),
        },
        Err(_) => {
            match tty {
                true => eprintln!(
                    "{} Can't create file {}\nNot sure what went wrong, sorry!",
                    "error:".dark_red().bold(),
                    &x.join("").to_string().dark_green()
                ),
                false => eprintln!(
                    "error: Can't create file {}\nNot sure what went wrong, sorry!",
                    &x.join("").to_string()
                ),
            };
            exit(1)
        }
    }
}

fn get_cache_path(tty: &bool) -> String {
    let cache_path = match env::var("XDG_CACHE_HOME") {
        Ok(x) => x.to_string(),
        Err(_) => {
            let home_path = match env::var("HOME") {
                Ok(x) => x.to_string(),
                Err(_) => {
                    match tty {
                        true => eprintln!(
                            "{} Can't find your HOME directory, sorry!",
                            "error:".dark_red().bold()
                        ),
                        false => eprintln!("error: Can't find your HOME directory, sorry!"),
                    }
                    exit(1)
                }
            };
            let x = [home_path, "/.cache".to_string()];
            x.join("").to_string()
        }
    };
    cache_path
}

fn write_file(path: &String, html: &String) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(path)?;
    file.write_all(html.as_bytes())?;
    Ok(())
}
