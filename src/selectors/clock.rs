use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_BORDERS_ONLY, Attribute, Cell, CellAlignment,
    Color, ContentArrangement, Row, Table
};

use crate::selectors::get_vec;

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let source = get_vec(data, "div.vk_gy.vk_sh.card-section.sL6Rbf");

    let mut date_vec: Vec<&str> = vec![];
    let mut day = source[3].to_string();
    day.remove(0);
    date_vec.push(&day);
    date_vec.push(source[4]);

    let mut timezone = source[6].to_string();
    timezone.remove(timezone.len() - 1);
    date_vec.push(&timezone);

    let mut location = source[9].to_string();
    location.remove(0);
    location.remove(location.len() - 1);
    location.remove(location.len() - 1);

    match tty {
        true => {
            let mut t = Table::new();
            t.load_preset(UTF8_BORDERS_ONLY);
            t.apply_modifier(UTF8_ROUND_CORNERS);
            t.set_content_arrangement(ContentArrangement::Dynamic);
            if w >= 100 {
                t.set_table_width(100);
            }
            t.set_header(Row::from(vec![Cell::new(source[1])
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
                .fg(Color::Green)]));
            t.add_row(Row::from(vec![
                Cell::new(date_vec.join("")).set_alignment(CellAlignment::Center)
            ]));
            t.add_row(Row::from(vec![Cell::new(location)
                .set_alignment(CellAlignment::Center)
                .fg(Color::DarkGrey)]));
            println!("{}", t)
        }
        false => println!("{}\n{}\n{}", source[1], date_vec.join(""), location)
    }
}
