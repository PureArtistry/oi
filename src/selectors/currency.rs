use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_BORDERS_ONLY, Attribute, Cell, CellAlignment,
    Color, ContentArrangement, Row, Table
};

use crate::selectors::get_vec;

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let x = get_vec(data, "span.DFlfde.SwHCTb");
    let y = get_vec(data, "span.MWvIVe");
    match tty {
        true => {
            let mut t = Table::new();
            t.load_preset(UTF8_BORDERS_ONLY);
            t.apply_modifier(UTF8_ROUND_CORNERS);
            t.set_content_arrangement(ContentArrangement::Dynamic);
            if w > 100 {
                t.set_table_width(100);
            }
            t.set_header(Row::from(vec![Cell::new(x[0])
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
                .fg(Color::Green)]));
            t.add_row(Row::from(vec![Cell::new(y[0])
                .set_alignment(CellAlignment::Center)
                .fg(Color::DarkGrey)]));
            println!("{}", t)
        }
        false => println!("{} {}", x[0], y[0])
    }
}
