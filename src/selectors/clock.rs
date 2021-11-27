use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_BORDERS_ONLY, Attribute, Cell, CellAlignment,
    Color, ContentArrangement, Row, Table
};

use crate::selectors::get_vec;

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let x = get_vec(data, "div.vk_gy.vk_sh.card-section.sL6Rbf");
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
            let mut t = Table::new();
            t.load_preset(UTF8_BORDERS_ONLY);
            t.apply_modifier(UTF8_ROUND_CORNERS);
            t.set_content_arrangement(ContentArrangement::Dynamic);
            if w >= 100 {
                t.set_table_width(100);
            }
            t.set_header(Row::from(vec![Cell::new(x[1])
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
                .fg(Color::Green)]));
            t.add_row(Row::from(vec![
                Cell::new(l1.join("")).set_alignment(CellAlignment::Center)
            ]));
            t.add_row(Row::from(vec![Cell::new(l2[0])
                .set_alignment(CellAlignment::Center)
                .fg(Color::DarkGrey)]));
            println!("{}", t)
        }
        false => println!("{}\n{}\n{}", x[1], l1.join(""), l2[0])
    }
}
