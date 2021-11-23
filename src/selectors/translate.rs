use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Attribute, Cell, CellAlignment, Color, ContentArrangement, Row, Table
};
use scraper::Selector;

use crate::selectors::get_vec;

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let source = get_vec(data, "span.source-language");
    let target = get_vec(data, "span.target-language");
    let mut answers = vec![];
    for x in data.select(&Selector::parse("span.Y2IQFc").unwrap()) {
        answers.push(x.text().next().unwrap_or(""))
    }
    match tty {
        true => {
            let mut t = Table::new();
            t.load_preset(UTF8_FULL);
            t.apply_modifier(UTF8_ROUND_CORNERS);
            t.apply_modifier(UTF8_SOLID_INNER_BORDERS);
            t.set_content_arrangement(ContentArrangement::Dynamic);
            if w > 100 {
                t.set_table_width(100);
            }
            t.set_header(Row::from(vec![
                Cell::new(source[0])
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::DarkGrey),
                Cell::new(target[0])
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::DarkGrey),
            ]));
            t.add_row(Row::from(vec![
                Cell::new(answers[0])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold),
                Cell::new(answers[2])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Green),
            ]));
            println!("{}", t)
        }
        false => {
            println!(
                "source: {}\n{}\n\ntarget: {}\n{}",
                source[0], answers[0], target[0], answers[2]
            )
        }
    }
}
