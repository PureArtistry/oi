use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::{UTF8_BORDERS_ONLY, UTF8_FULL};
use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Row, Table};
use crossterm::style::{Colorize, Styler};
use scraper::Selector;

pub fn details() -> [(&'static str, &'static str, &'static str); 16] {
    [
        (
            "simple_values",
            "Google's top line answer for simple dates, values etc",
            "what is the density of silver",
        ),
        (
            "basic_answers",
            "Similar to simple_values but uses a different div",
            "shortest day of the year",
        ),
        ("define", "Dictionary and thesaurus", "define free"),
        ("cast_list", "Film/TV cast lists", "fight club cast"),
        ("lists", "Simple lists", "how to exit vim"),
        ("snippets", "Featured snippets", "who is robert paulson"),
        ("lyrics", "Song lyrics", "paint it black lyrics"),
        (
            "kno_right",
            "Knowledge graph (info in right pane)",
            "outer edges",
        ),
        ("quotes", "List of quotes", "linus torvalds quotes"),
        ("weather", "Current weather forecast", "weather london uk"),
        ("maths", "Math equations", "log_2(3) * pi^e"),
        ("currency", "Currency conversion", "£100 in us dollars"),
        (
            "translate",
            "Language translation",
            "Vais para cascais? em ingles",
        ),
        (
            "sports",
            "Sports fixtures and results",
            "result of last el clasico",
        ),
        ("pronounce", "Learn to pronounce", "pronounce linux"),
        ("clock", "World clock", "time in new york"),
    ]
}

pub fn print_list(details: [(&'static str, &'static str, &'static str); 16]) {
    let mut list = Table::new();
    list.load_preset(UTF8_FULL);
    list.apply_modifier(UTF8_ROUND_CORNERS);
    list.set_content_arrangement(ContentArrangement::Dynamic);
    list.set_header(Row::from(vec![
        Cell::new("selector")
            .set_alignment(CellAlignment::Center)
            .add_attribute(Attribute::Bold)
            .fg(Color::Cyan),
        Cell::new("description")
            .set_alignment(CellAlignment::Center)
            .add_attribute(Attribute::Bold)
            .fg(Color::Cyan),
        Cell::new("example")
            .set_alignment(CellAlignment::Center)
            .add_attribute(Attribute::Bold)
            .fg(Color::Cyan),
    ]));
    for i in 0..details.len() {
        list.add_row(Row::from(vec![
            Cell::new(details[i].0)
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold),
            Cell::new(details[i].1)
                .set_alignment(CellAlignment::Center)
                .fg(Color::Grey),
            Cell::new(details[i].2)
                .set_alignment(CellAlignment::Center)
                .fg(Color::Green),
        ]));
    }
    println!("{}", list);
    std::process::exit(0)
}

pub fn name_to_id(selector: &str) -> &str {
    match selector {
        "corrections" => "a.gL9Hy",
        "simple_values" => "div.Z0LcW.XcVN5d",
        "basic_answers" => "div.zCubwf",
        "define" => "div.VpH2eb.dZd3De.vmod",
        "cast_list" => "a.ct5Ked.klitem-tr.PZPZlf",
        "lists" => "div.co8aDb.XcVN5d",
        "snippets" => "span.hgKElc",
        "lyrics" => "div.ujudUb.WRZytc.OULBYb",
        "kno_right" => "div.kno-rdesc",
        "quotes" => "div.Qynugf",
        "weather" => "div.UQt4rd",
        "maths" => "span.qv3Wpe",
        "currency" => "span.DFlfde.SwHCTb",
        "translate" => "pre.XcVN5d",
        "sports" => "div.imso_mh__tm-scr",
        "pronounce" => "div.fQ02Rb.eDzgme",
        "clock" => "div.vk_c.vk_gy.vk_sh.card-section.sL6Rbf.R36Kq",
        &_ => panic!(),
    }
}

pub fn print_answer(
    data: &scraper::Html,
    answers: Vec<&str>,
    tty: &bool,
    quiet: &bool,
    total: Vec<&str>,
) {
    let mut i: usize = answers.len();
    for x in &answers {
        match x {
            &"simple_values" => simple_values(data, tty),
            &"basic_answers" => basic_answers(data, tty),
            &"define" => define(data, tty),
            &"maths" => maths(data, tty),
            &"cast_list" => cast_list(data, tty),
            &"quotes" => quotes(data, tty),
            &"lists" => lists(data, tty),
            &"snippets" => snippets(data, tty),
            &"lyrics" => lyrics(data, tty),
            &"weather" => weather(data, tty),
            &"currency" => currency(data, tty),
            &"translate" => translate(data, tty),
            &"kno_right" => kno_right(data, tty),
            &"sports" => sports(data, tty),
            &"pronounce" => pronounce(data, tty),
            &"clock" => clock(data, tty),
            _ => panic!(),
        }

        if quiet == &false {
            print!("{}", "selector chosen: ".grey());
            for y in &total {
                if y == x {
                    print!("{} ", y.bold().dark_cyan())
                } else {
                    print!("{} ", y.grey())
                }
            }
            print!("\n")
        }

        i -= 1;
        if i > 0 && tty == &false {
            print!("\n")
        }
    }
}

pub fn filter(mut answers: Vec<&str>) -> Vec<&str> {
    // this is just a temporary hack until I think of a better way of filtering the answers
    for _ in 0..(answers.len() - 1) {
        answers.pop();
    }
    answers
}

// SELECTOR FORMATTING FUNCTIONS

fn get_vec(data: &scraper::Html, x: String) -> Vec<&str> {
    data.select(&Selector::parse(&x).unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<Vec<&str>>()
}

fn default_output(x: &str) {
    let mut y = Table::new();
    y.load_preset(UTF8_BORDERS_ONLY);
    y.apply_modifier(UTF8_ROUND_CORNERS);
    y.set_content_arrangement(ContentArrangement::Dynamic);
    y.add_row(Row::from(vec![Cell::new(x)
        .set_alignment(CellAlignment::Center)
        .add_attribute(Attribute::Bold)]));
    println!("{}", y)
}

fn simple_values(data: &scraper::Html, tty: &bool) {
    let x = get_vec(data, "div.Z0LcW.XcVN5d".to_string());
    match tty {
        true => default_output(x[0]),
        false => println!("{}", x[0]),
    }
}
fn basic_answers(data: &scraper::Html, tty: &bool) {
    let x = get_vec(data, "div.zCubwf".to_string());
    match tty {
        true => default_output(&x.join("")),
        false => println!("{}", x.join("")),
    }
}
fn define(data: &scraper::Html, tty: &bool) {
    // this one is a massive pain in the arse, I'll think of a way to better format it soon.
    let mut words = vec![];
    for x in data.select(&Selector::parse("div.L1jWkf.U3R6Ke").unwrap()) {
        words.push(x.text().collect::<Vec<_>>())
    }
    let mut defs = vec![];
    for x in data.select(&Selector::parse("ol.eQJLDd").unwrap()) {
        defs.push(x.text().collect::<Vec<_>>())
    }
    let j = words.len();
    if j != defs.len() {
        panic!()
    }
    for i in 0..j {
        match tty {
            true => println!("{}:", words[i][0].grey()),
            false => println!("{}:", words[i][0]),
        }
        match defs[i][0].parse::<u8>() {
            Ok(_) => match tty {
                true => println!("{}", defs[i][2].bold()),
                false => println!("{}", defs[i][2]),
            },
            Err(_) => match tty {
                true => println!("{}", defs[i][0].bold()),
                false => println!("{}", defs[i][0]),
            },
        }
        if i != (j - 1) {
            print!("\n")
        }
    }
}
fn maths(data: &scraper::Html, tty: &bool) {
    let x = get_vec(data, "span.qv3Wpe".to_string());
    let mut y = x[0].to_string();
    y.remove(y.len() - 1);
    y.remove(y.len() - 1);
    y.remove(0);
    match tty {
        true => default_output(&y),
        false => println!("{}", y),
    }
}
fn cast_list(data: &scraper::Html, tty: &bool) {
    let mut y: Vec<&str> = vec![];
    for x in data.select(&Selector::parse("a.ct5Ked.klitem-tr.PZPZlf").unwrap()) {
        y.push(x.value().attr("title").unwrap())
    }
    let j = y.len();
    match tty {
        true => {
            let mut z = Table::new();
            z.load_preset(UTF8_FULL);
            z.apply_modifier(UTF8_ROUND_CORNERS);
            z.set_content_arrangement(ContentArrangement::Dynamic);
            for i in 0..j {
                z.add_row(Row::from(vec![Cell::new(y[i])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)]));
            }
            println!("{}", z)
        }
        false => {
            println!("{}", y.join("\n"))
        }
    }
}
fn quotes(data: &scraper::Html, tty: &bool) {
    let mut y: Vec<&str> = vec![];
    for x in data.select(&Selector::parse("div.Qynugf").unwrap()) {
        y.push(x.text().next().unwrap())
    }
    let j = y.len();
    match tty {
        true => {
            let mut z = Table::new();
            z.load_preset(UTF8_FULL);
            z.apply_modifier(UTF8_ROUND_CORNERS);
            z.set_content_arrangement(ContentArrangement::Dynamic);
            for i in 0..j {
                z.add_row(Row::from(vec![Cell::new(y[i])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)]));
            }
            println!("{}", z)
        }
        false => {
            for i in 0..j {
                println!("{}", y[i]);
                if i != (j - 1) {
                    print!("\n")
                }
            }
        }
    }
}
fn lists(data: &scraper::Html, tty: &bool) {
    let title = get_vec(data, "div.co8aDb.XcVN5d".to_string()).join("");
    let mut answer: Vec<_> = vec![];
    for x in data.select(&Selector::parse("li.TrT0Xe").unwrap()) {
        let y = x.text().collect::<Vec<_>>().join("");
        answer.push(y)
    }
    match tty {
        true => {
            let mut z = Table::new();
            z.load_preset(UTF8_FULL);
            z.apply_modifier(UTF8_ROUND_CORNERS);
            z.set_content_arrangement(ContentArrangement::Dynamic);
            z.set_header(Row::from(vec![Cell::new(title)
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan)]));
            for i in 0..answer.len() {
                z.add_row(Row::from(vec![Cell::new(&answer[i])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)]));
            }
            println!("{}", z)
        }
        false => {
            println!("{}", title);
            for i in 0..answer.len() {
                println!("\n{} -> {}", (i + 1), answer[i])
            }
        }
    }
}
fn snippets(data: &scraper::Html, tty: &bool) {
    let x = get_vec(data, "span.hgKElc".to_string());
    match tty {
        true => default_output(&x.join("")),
        false => println!("{}", x.join("")),
    }
}
fn lyrics(data: &scraper::Html, tty: &bool) {
    let mut x = get_vec(data, "div.i4J0ge".to_string());
    for _ in 0..4 {
        x.pop();
    }
    let y = get_vec(data, "div.ujudUb.WRZytc.OULBYb".to_string());
    let j = y.len();
    let mut i = x.iter().position(|&r| r == y[(j - 1)]).unwrap();
    for _ in 0..j {
        x.remove(i);
        i -= 1
    }
    match tty {
        true => default_output(&x.join("\n")),
        false => println!("{}", x.join("\n")),
    }
}
fn weather(data: &scraper::Html, tty: &bool) {
    let temps = get_vec(data, "div.vk_bk.TylWce".to_string());
    let mut c = vec![];
    let mut f = vec![];
    match temps[0].parse::<i32>().unwrap() < temps[1].parse::<i32>().unwrap() {
        true => {
            c.push(temps[0]);
            f.push(temps[1])
        }
        false => {
            c.push(temps[1]);
            f.push(temps[0])
        }
    }
    c.push("°C");
    f.push("°F");

    let misc = get_vec(data, "div.wtsRwe".to_string());
    let details = get_vec(data, "div.VQF4g".to_string());

    match tty {
        true => {
            let mut x = Table::new();
            x.load_preset(UTF8_FULL);
            x.apply_modifier(UTF8_ROUND_CORNERS);
            x.set_content_arrangement(ContentArrangement::Dynamic);
            x.add_row(Row::from(vec![
                Cell::new(c.join(""))
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Cyan),
                Cell::new(f.join(""))
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Cyan),
            ]));
            x.add_row(Row::from(vec![
                Cell::new("Description:")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Grey),
                Cell::new(details[2])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Green),
            ]));
            x.add_row(Row::from(vec![
                Cell::new("Precipitation:")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Grey),
                Cell::new(misc[1])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold),
            ]));
            x.add_row(Row::from(vec![
                Cell::new("Humidity:")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Grey),
                Cell::new(misc[3])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold),
            ]));
            x.add_row(Row::from(vec![
                Cell::new("Wind:")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Grey),
                Cell::new(misc[5])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold),
            ]));
            x.add_row(Row::from(vec![
                Cell::new("Location:")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Grey),
                Cell::new(details[0]).set_alignment(CellAlignment::Center),
            ]));
            x.add_row(Row::from(vec![
                Cell::new("Time:")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Grey),
                Cell::new(details[1]).set_alignment(CellAlignment::Center),
            ]));
            println!("{}", x)
        }
        false => {
            println!("{} | {} or {}", details[2], c.join(""), f.join(""));
            println!("Precipitation: {}", misc[1]);
            println!("Humidity: {}", misc[3]);
            println!("Wind: {}", misc[5]);
            println!("Location: {}", details[0]);
            println!("Time: {}", details[1])
        }
    }
}
fn currency(data: &scraper::Html, tty: &bool) {
    let x = get_vec(data, "span.DFlfde.SwHCTb".to_string());
    match tty {
        true => default_output(x[0]),
        false => println!("{}", x[0]),
    }
}
fn translate(data: &scraper::Html, tty: &bool) {
    let source = get_vec(data, "span.source-language".to_string());
    let target = get_vec(data, "span.target-language".to_string());
    let mut answers = vec![];
    for x in data.select(&Selector::parse("span.Y2IQFc").unwrap()) {
        answers.push(x.text().next().unwrap_or(""))
    }
    match tty {
        true => {
            let mut x = Table::new();
            x.load_preset(UTF8_FULL);
            x.apply_modifier(UTF8_ROUND_CORNERS);
            x.set_content_arrangement(ContentArrangement::Dynamic);
            x.set_header(Row::from(vec![
                Cell::new(source[0])
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Grey),
                Cell::new(target[0])
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::Grey),
            ]));
            x.add_row(Row::from(vec![
                Cell::new(answers[0])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold),
                Cell::new(answers[2])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Cyan),
            ]));
            println!("{}", x)
        }
        false => {
            println!(
                "source: {}\n{}\n\ntarget: {}\n{}",
                source[0], answers[0], target[0], answers[2]
            )
        }
    }
}
fn kno_right(data: &scraper::Html, tty: &bool) {
    let x = get_vec(data, "div.kno-rdesc".to_string());
    match tty {
        true => default_output(x[1]),
        false => println!("{}", x[1]),
    }
}
fn sports(data: &scraper::Html, tty: &bool) {
    let heading = get_vec(
        data,
        "div.imso_mh__stts-l.imso-ani.imso_mh__stts-l-cont".to_string(),
    );
    let scores = get_vec(data, "div.imso_mh__scr-sep".to_string());
    let team1 = get_vec(
        data,
        "div.imso_mh__first-tn-ed.imso_mh__tnal-cont.imso-tnol".to_string(),
    );
    let team2 = get_vec(
        data,
        "div.imso_mh__second-tn-ed.imso_mh__tnal-cont.imso-tnol".to_string(),
    );

    match heading.len() {
        3 => match tty {
            true => {
                let mut x = Table::new();
                x.load_preset(UTF8_BORDERS_ONLY);
                x.apply_modifier(UTF8_ROUND_CORNERS);
                x.set_content_arrangement(ContentArrangement::Dynamic);
                x.set_header(Row::from(vec![
                    Cell::new(heading[0]).set_alignment(CellAlignment::Center),
                    Cell::new(heading[1])
                        .set_alignment(CellAlignment::Center)
                        .fg(Color::Grey),
                    Cell::new(heading[2])
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold)
                        .fg(Color::Cyan),
                ]));
                x.add_row(Row::from(vec![
                    Cell::new(team1[0])
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold),
                    Cell::new(scores[0])
                        .set_alignment(CellAlignment::Center)
                        .fg(Color::Grey),
                    Cell::new(team2[0])
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold),
                ]));
                println!("{}", x)
            }
            false => println!(
                "{}\n{} {} {}",
                heading.join(""),
                team1[0],
                scores[0],
                team2[0]
            ),
        },
        4 => match tty {
            true => {
                let mut x = Table::new();
                x.load_preset(UTF8_FULL);
                x.apply_modifier(UTF8_ROUND_CORNERS);
                x.set_content_arrangement(ContentArrangement::Dynamic);
                let mut y = vec![];
                for i in 1..4 {
                    y.push(heading[i])
                }
                x.set_header(Row::from(vec![
                    Cell::new(y.join(""))
                        .set_alignment(CellAlignment::Center)
                        .fg(Color::Grey),
                    Cell::new(heading[0])
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold)
                        .fg(Color::Green),
                ]));
                let a = scores[0].parse::<u32>().unwrap();
                let b = scores[2].parse::<u32>().unwrap();
                if a == b {
                    x.add_row(Row::from(vec![
                        Cell::new(team1.join("\n"))
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                        Cell::new(scores[0])
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                    ]));
                    x.add_row(Row::from(vec![
                        Cell::new(team2.join("\n"))
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                        Cell::new(scores[2])
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                    ]));
                } else if a > b {
                    x.add_row(Row::from(vec![
                        Cell::new(team1.join("\n"))
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Cyan),
                        Cell::new(scores[0])
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Cyan),
                    ]));
                    x.add_row(Row::from(vec![
                        Cell::new(team2.join("\n"))
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                        Cell::new(scores[2])
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                    ]));
                } else {
                    x.add_row(Row::from(vec![
                        Cell::new(team1.join("\n"))
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                        Cell::new(scores[0])
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                    ]));
                    x.add_row(Row::from(vec![
                        Cell::new(team2.join("\n"))
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Cyan),
                        Cell::new(scores[2])
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Cyan),
                    ]));
                }
                println!("{}", x)
            }
            false => println!(
                "{}{}{} | {}\n{} {}\n{} {}",
                heading[1],
                heading[2],
                heading[3],
                heading[0],
                team1.join(" "),
                scores[0],
                team2.join(" "),
                scores[2]
            ),
        },
        _ => panic!(),
    }
}
fn pronounce(data: &scraper::Html, tty: &bool) {
    let x = get_vec(data, "div.fQ02Rb.eDzgme".to_string());
    match tty {
        true => default_output(&x.join("")),
        false => println!("{}", x.join("")),
    }
}
fn clock(data: &scraper::Html, tty: &bool) {
    let x = get_vec(
        data,
        "div.vk_c.vk_gy.vk_sh.card-section.sL6Rbf.R36Kq".to_string(),
    );
    let mut l1: Vec<&str> = vec![];
    let mut l2: Vec<&str> = vec![];
    let mut y1 = x[3].to_string();
    y1.remove(0);
    l1.push(&y1);
    l1.push(x[4]);
    let mut y2 = x[6].to_string();
    y2.remove(y2.len() - 1);
    l1.push(&y2);
    let mut y3 = x[9].to_string();
    y3.remove(0);
    y3.remove(y3.len() - 1);
    y3.remove(y3.len() - 1);
    l2.push(&y3);
    match tty {
        true => {
            let mut z = Table::new();
            z.load_preset(UTF8_BORDERS_ONLY);
            z.apply_modifier(UTF8_ROUND_CORNERS);
            z.set_content_arrangement(ContentArrangement::Dynamic);
            z.set_header(Row::from(vec![Cell::new(x[1])
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan)]));
            z.add_row(Row::from(vec![
                Cell::new(l1.join("")).set_alignment(CellAlignment::Center)
            ]));
            z.add_row(Row::from(vec![Cell::new(l2[0])
                .set_alignment(CellAlignment::Center)
                .fg(Color::Grey)]));
            println!("{}", z)
        }
        false => println!("{}\n{}\n{}", x[1], l1.join(""), l2[0]),
    }
}
