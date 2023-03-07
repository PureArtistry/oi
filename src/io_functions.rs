use std::{env, fs, io::Write, path::Path};

use anyhow::{bail, Result};
use chrono::prelude::Local;
use glob::glob;

/// SeParator -  Only here to handle windows properly
const SP: &str = if cfg!(windows) { "\\" } else { "/" };

pub fn fetch(query: String, lang: String) -> Result<String, ureq::Error> {
    let x = ureq::get("https://google.com/search")
        .set(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) QtWebEngine/5.15.2 Chrome/87.0.4280.144 Safari/537.36"
        )
        .query("q", &query)
        .query("hl", &lang)
        .call()?
        .into_string()
        .unwrap();
    Ok(x)
}

pub fn cached_html() -> Result<String> {
    let files = get_file_list()?;
    let html = fs::read_to_string(&files[(files.len() - 1)])?;
    Ok(html)
}

pub fn save_html(query: &[&str], html: &str) -> Result<String> {
    let cache_path = get_cache_path()?;
    let file_date = Local::now().format("%s").to_string();
    let file_query = query.join("_");

    let mut x: Vec<&str> = vec![&cache_path, SP, "oi"];
    if !Path::new(&x.join("")).is_dir() {
        fs::create_dir(&x.join(""))?
    }
    x.push(SP);
    x.push(&file_date);
    x.push("-");
    x.push(&file_query);
    x.push(".html");

    let full_path = x.join("");
    let mut file = fs::File::create(&full_path)?;
    file.write_all(html.as_bytes())?;
    Ok(full_path)
}

pub fn clean_cache() -> Result<String> {
    let target = [&get_cache_path()?, SP, "oi"].join("");
    fs::remove_dir_all(&target)?;
    Ok(target)
}

fn get_cache_path() -> Result<String> {
    let cache_path: String =
        if cfg!(target_os = "freebsd") || cfg!(target_os = "linux") {
            env::var("XDG_CACHE_HOME").unwrap_or(env::var("HOME")? + "/.cache")
        } else if cfg!(target_os = "macos") {
            env::var("HOME")? + "/Library/Application Support"
        } else if cfg!(target_os = "windows") {
            env::var("LOCALAPPDATA")?
        } else {
            bail!("This feature is not supported on your platform, sorry!")
        };
    Ok(cache_path)
}

fn get_file_list() -> Result<Vec<String>> {
    let cache_path = [&get_cache_path()?, SP, "oi", SP, "*.html"].join("");

    let mut files: Vec<String> = vec![];
    for x in glob(&cache_path).unwrap() {
        match x {
            Ok(x) => files.push(x.display().to_string()),
            Err(_) => panic!("get_file_list: glob search failed!?!")
        }
    }

    match files.len() {
        0 => bail!("Can't find any cached results, sorry!"),
        _ => Ok(files)
    }
}
