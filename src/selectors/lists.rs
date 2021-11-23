use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Attribute, Cell, CellAlignment, ContentArrangement, Row, Table
};
use scraper::Selector;

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let mut y: Vec<&str> = vec![];
    for x in data.select(&Selector::parse("a.ct5Ked.klitem-tr.PZPZlf").unwrap()) {
        y.push(x.value().attr("title").unwrap())
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
            for x in &y {
                t.add_row(Row::from(vec![Cell::new(x)
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)]));
            }
            println!("{}", t)
        }
        false => {
            println!("{}", y.join("\n"))
        }
    }
}
