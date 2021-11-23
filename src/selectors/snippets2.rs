use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Attribute, Cell, CellAlignment, Color, ContentArrangement, Row, Table
};
use scraper::Selector;

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let mut title = vec![];
    let title_check = data.select(&Selector::parse("div.co8aDb").unwrap()).next();
    if let Some(x) = title_check {
        title.push(x.text().collect::<Vec<&str>>())
    }

    let mut list = vec![];
    for x in data.select(&Selector::parse("li.TrT0Xe").unwrap()) {
        list.push(x.text().collect::<Vec<&str>>())
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
            if !title.is_empty() {
                t.set_header(Row::from(vec![Cell::new(title[0].join(""))
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Green)]));
            }
            for x in &list {
                t.add_row(Row::from(vec![Cell::new(x.join(""))
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)]));
            }
            println!("{}", t);
        }
        false => {
            if !title.is_empty() {
                println!("{}\n", title[0].join(""));
            }
            for (i, x) in list.iter().enumerate() {
                println!("[{}] {}", (i + 1), x.join(""));
            }
        }
    }
}
