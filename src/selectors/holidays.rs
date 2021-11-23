use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Attribute, Cell, CellAlignment, Color, ContentArrangement, Row, Table
};
use crossterm::style::Stylize;
use scraper::Selector;

use crate::selectors::get_vec;

pub fn tty(data: &scraper::Html, w: usize) {
    let mut name: Vec<_> = vec![];
    for x in data.select(&Selector::parse("td.LakcPb").unwrap()) {
        let y = x.text().collect::<Vec<_>>().join("");
        name.push(y)
    }
    let mut date: Vec<_> = vec![];
    for x in data.select(&Selector::parse("td.fs5Qwd").unwrap()) {
        let y = x.text().collect::<Vec<_>>().join("");
        date.push(y)
    }
    let n = name.len();
    assert_eq!(n, date.len());

    if n == 0 {
        println!(
            "\n{} There are no valid holidays associated with this query.\n{}\n",
            "error:".bold().red(),
            "(this is a known bug, sorry about that!)".dark_grey()
        );
    }
    else {
        let title = get_vec(data, "div.N6Sb2c.i29hTd").join(" ");
        println!("{}", title.bold().cyan());

        let mut t = Table::new();
        t.load_preset(UTF8_FULL);
        t.apply_modifier(UTF8_ROUND_CORNERS);
        t.apply_modifier(UTF8_SOLID_INNER_BORDERS);
        t.set_content_arrangement(ContentArrangement::Dynamic);
        if w > 100 {
            t.set_table_width(100);
        }
        t.set_header(Row::from(vec![
            Cell::new("name")
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
                .fg(Color::DarkGrey),
            Cell::new("date")
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
                .fg(Color::DarkGrey),
        ]));
        for i in 0..n {
            t.add_row(Row::from(vec![
                Cell::new(&name[i])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Green),
                Cell::new(&date[i])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold),
            ]));
        }
        println!("{}", t)
    }
}

pub fn raw(data: &scraper::Html) {
    let mut name: Vec<_> = vec![];
    for x in data.select(&Selector::parse("td.LakcPb").unwrap()) {
        let y = x.text().collect::<Vec<_>>().join("");
        name.push(y)
    }
    let mut date: Vec<_> = vec![];
    for x in data.select(&Selector::parse("td.fs5Qwd").unwrap()) {
        let y = x.text().collect::<Vec<_>>().join("");
        date.push(y)
    }
    let n = name.len();
    assert_eq!(n, date.len());

    if n == 0 {
        eprintln!(
            "error: There are no valid holidays associated with this query.\n(this is a known \
             bug, sorry about that!)"
        )
    }
    else {
        let title = get_vec(data, "div.N6Sb2c.i29hTd").join(" ");
        println!("{}\n", title);
        for i in 0..n {
            println!("{} - {}", name[i], date[i])
        }
    }
}
