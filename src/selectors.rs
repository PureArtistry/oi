use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::{UTF8_BORDERS_ONLY, UTF8_FULL},
    Attribute, Cell, CellAlignment, Color, ContentArrangement, Row, Table
};
use crossterm::style::Stylize;
use scraper::Selector;

mod basic1;
mod basic2;
mod clock;
mod conversions;
mod currency;
mod define;
mod holidays;
mod lists;
mod lyrics;
mod maths;
mod pronounce;
mod snippets1;
mod snippets2;
mod sports;
mod summary;
mod translate;
mod weather;

pub fn details() -> [(&'static str, &'static str, &'static str, &'static str); 17] {
    // selector name - single "word" (no whitespace), preferably descriptive
    // result description
    // example query
    // a (hopefully) unique selector id used to check for answers
    [
        (
            "basic1",
            "Google's top line answer for simple dates, values etc",
            "what is the density of silver",
            "div.HwtpBd.gsrt.PZPZlf.kTOYnf"
        ),
        (
            "basic2",
            "Similar to basic1 but uses different IDs",
            "shortest day of the year",
            "div.zCubwf"
        ),
        (
            "clock",
            "World clock",
            "time in new york",
            "div.vk_gy.vk_sh.card-section.sL6Rbf"
        ),
        (
            "conversion",
            "Unit conversions",
            "395mm in inch",
            "div.CR33Se"
        ),
        (
            "currency",
            "Currency conversion",
            "'£100 in us dollars'",
            "span.DFlfde.SwHCTb"
        ),
        (
            "define",
            "Dictionary and thesaurus - English only currently\n(also, the formatting can \
             occasionally mess up - working on it, sorry!)",
            "define free",
            "div.VpH2eb.vmod"
        ),
        (
            "holidays",
            "Public holiday list",
            "holidays uk",
            "div.kp-blk.EyBRub.XzTjhb.fm06If"
        ),
        (
            "lists",
            "Film/TV cast lists, album tracklists, artist's [*]ography",
            "snatch cast",
            "a.ct5Ked.klitem-tr.PZPZlf"
        ),
        (
            "lyrics",
            "Song lyrics",
            "paint it black lyrics",
            "div.ujudUb.WRZytc.OULBYb"
        ),
        (
            "maths",
            "Math equations",
            "'log_2(3) * pi^e'",
            "span.qv3Wpe"
        ),
        (
            "pronounce",
            "Learn to pronounce",
            "shibboleth pronunciation",
            "div.TQ7enb"
        ),
        (
            "snippets1",
            "Featured snippets",
            "who is robert paulson",
            "span.hgKElc"
        ),
        (
            "snippets2",
            "These are typically short lists\n(of quotes for example)",
            "linus torvalds quotes",
            "div.di3YZe"
        ),
        (
            "sports",
            "Sports fixtures and results",
            "result of last el clasico",
            "div.imso_mh__tm-scr"
        ),
        (
            "summary",
            "Short info summary\n(shown in right pane on website)",
            "outer edges",
            "div.kno-rdesc"
        ),
        (
            "translate",
            "Language translation",
            "'Vais para cascais? em ingles'",
            "div.tw-src-ltr"
        ),
        (
            "weather",
            "Weather forecast information",
            "weather fiji",
            "div.UQt4rd"
        )
    ]
}

pub fn print_list(details: [(&'static str, &'static str, &'static str, &'static str); 17]) {
    let mut t = Table::new();
    t.load_preset(UTF8_FULL);
    t.apply_modifier(UTF8_ROUND_CORNERS);
    t.apply_modifier(UTF8_SOLID_INNER_BORDERS);
    t.set_content_arrangement(ContentArrangement::Dynamic);
    t.set_header(Row::from(vec![
        Cell::new("selector")
            .set_alignment(CellAlignment::Center)
            .add_attribute(Attribute::Bold)
            .fg(Color::DarkGrey),
        Cell::new("description")
            .set_alignment(CellAlignment::Center)
            .add_attribute(Attribute::Bold)
            .fg(Color::DarkGrey),
        Cell::new("example")
            .set_alignment(CellAlignment::Center)
            .add_attribute(Attribute::Bold)
            .fg(Color::DarkGrey),
    ]));
    for x in &details {
        t.add_row(Row::from(vec![
            Cell::new(x.0)
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
                .fg(Color::Cyan),
            Cell::new(x.1).set_alignment(CellAlignment::Center),
            Cell::new(x.2)
                .set_alignment(CellAlignment::Center)
                .fg(Color::Green),
        ]));
    }
    println!("{}", t);
    std::process::exit(0)
}

pub fn print_answer(
    data: &scraper::Html, answers: Vec<&str>, tty: &bool, w: usize, quiet: &bool, total: Vec<&str>
) {
    let mut i = answers.len();
    for x in answers {
        match x {
            "basic1" => basic1::main(data, tty, w),
            "basic2" => basic2::main(data, tty, w),
            "clock" => clock::main(data, tty, w),
            "conversion" => conversions::main(data, tty, w),
            "currency" => currency::main(data, tty, w),
            "define" => match *tty {
                true => define::tty(data, w),
                false => define::raw(data)
            },
            "holidays" => match *tty {
                true => holidays::tty(data, w),
                false => holidays::raw(data)
            },
            "lists" => lists::main(data, tty, w),
            "lyrics" => lyrics::main(data, tty, w),
            "maths" => maths::main(data, tty, w),
            "pronounce" => pronounce::main(data, tty, w),
            "snippets1" => snippets1::main(data, tty, w),
            "snippets2" => snippets2::main(data, tty, w),
            "sports" => sports::main(data, tty, w),
            "summary" => summary::main(data, tty, w),
            "translate" => translate::main(data, tty, w),
            "weather" => weather::main(data, tty, w),
            _ => panic!("selector passed from answers has no matching function!")
        }

        if !quiet {
            print!("{}", "selector chosen: ".dark_grey());
            for y in &total {
                match y == &x {
                    true => print!("{} ", y.bold().cyan()),
                    false => print!("{} ", y.dark_grey())
                }
            }
            println!()
        }

        i -= 1;
        if i > 0 && !tty {
            println!()
        }
    }
}

pub fn name_to_id(x: &str) -> &'static str {
    let r = match x {
        "corrections" => "a.gL9Hy",
        "url_block" => "div.tF2Cxc",
        "desc" => "div.VwiC3b.yXK7lf.MUxGbd.yDYNvb.lyLwlc.lEBKkf",
        "title" => "h3.LC20lb.MBeuO.DKV0Md",
        "url" => "div.yuRUbf",
        _ => panic!("name_to_id: no matcing id for this selector")
    };
    r
}

pub fn filter(mut answers: Vec<&str>, mut query: String) -> Vec<&str> {
    query = query.to_lowercase();
    let qv = query.split(' ').collect::<Vec<&str>>();
    let n = qv.len() - 1;

    for x in &answers {
        match *x {
            "define" => {
                if qv[0] == "define"
                    || qv[0] == "definition" && qv[1] == "of"
                    || qv[0] == "meaning" && qv[1] == "of"
                    || qv[n] == "definition"
                    || qv[n] == "meaning"
                {
                    answers = vec!["define"];
                    return answers
                }
            }

            "holidays" => {
                if qv[0] == "holidays"
                    || qv[0] == "public" && qv[1] == "holidays"
                    || qv[(n - 1)] == "public" && qv[n] == "holidays"
                    || qv[n] == "holidays"
                {
                    answers = vec!["holidays"];
                    return answers
                }
            }

            "lists" => {
                if qv[0] == "cast" && qv[1] == "of"
                    || qv[0] == "actors" && qv[1] == "in"
                    || qv[n] == "cast"
                    || qv[n] == "actors"
                    || qv[n] == "tracklist"
                    || qv[n] == "songs"
                {
                    answers = vec!["main_lists"];
                    return answers
                }
            }

            "lyrics" => {
                if qv[0] == "lyrics" || qv[n] == "lyrics" {
                    answers = vec!["lyrics"];
                    return answers
                }
            }

            "pronounce" => {
                if qv[0] == "pronounce"
                    || qv[0] == "pronunciation"
                    || qv[0] == "how" && qv[1] == "to" && qv[2] == "pronounce"
                    || qv[0] == "how" && qv[1] == "to" && qv[2] == "say"
                    || qv[n] == "pronunciation"
                {
                    answers = vec!["pronounce"];
                    return answers
                }
            }

            "quotes" => {
                if qv[0] == "quotes" && qv[1] == "by"
                    || qv[0] == "quotes" && qv[1] == "from"
                    || qv[n] == "quotes"
                {
                    answers = vec!["quotes"];
                    return answers
                }
            }

            "translate" => {
                if qv[0] == "translate" || qv[0] == "translation" || qv[n] == "translation" {
                    answers = vec!["translate"];
                    return answers
                }
            }

            "weather" => {
                if qv[0] == "weather" || qv[n] == "weather" {
                    answers = vec!["weather"];
                    return answers
                }
            }

            _ => {}
        }
    }

    for _ in 0..(answers.len() - 1) {
        answers.pop();
    }
    answers
}

// STANDARD SELECTOR FORMATTING FUNCTIONS

fn get_vec<'a>(data: &'a scraper::Html, id: &'a str) -> Vec<&'a str> {
    data.select(&Selector::parse(id).unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<Vec<&str>>()
}

fn default_output(s: &str, w: usize) {
    let mut t = Table::new();
    t.load_preset(UTF8_BORDERS_ONLY);
    t.apply_modifier(UTF8_ROUND_CORNERS);
    t.set_content_arrangement(ContentArrangement::Dynamic);
    if w >= 100 {
        t.set_table_width(100);
    }
    t.add_row(Row::from(vec![Cell::new(s)
        .set_alignment(CellAlignment::Center)
        .add_attribute(Attribute::Bold)]));
    println!("{}", t)
}
