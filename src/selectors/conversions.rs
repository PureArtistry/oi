use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Attribute, Cell, CellAlignment, Color, ContentArrangement, Row, Table
};
use crossterm::style::Stylize;
use scraper::Selector;

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let formula = data
        .select(&Selector::parse("div.bjhkR").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>();

    let mut units = vec![];
    for x in data.select(&Selector::parse("select.OR9QXc.LNn04b.gsrt").unwrap()) {
        let u1 = x.inner_html();
        let u2 = u1.split("</option>").collect::<Vec<_>>();
        for y in &u2 {
            if y.contains("<option selected=\"1\">") {
                let z = y.split('>').collect::<Vec<_>>();
                units.push(z[1].to_string())
            }
        }
    }

    let mut values = vec![];
    for x in data.select(&Selector::parse("input.vXQmIe.gsrt").unwrap()) {
        values.push(x.value().attr("value").unwrap())
    }

    match tty {
        true => {
            let mut t = Table::new();

            t.load_preset(UTF8_FULL);
            t.apply_modifier(UTF8_ROUND_CORNERS);
            t.apply_modifier(UTF8_SOLID_INNER_BORDERS);
            t.set_content_arrangement(ContentArrangement::Dynamic);

            if w >= 100 {
                t.set_table_width(100);
            }

            t.set_header(Row::from(vec![
                Cell::new("Unit")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::DarkGrey),
                Cell::new("Value")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::DarkGrey),
            ]));

            for i in 0..2 {
                t.add_row(Row::from(vec![
                    Cell::new(&units[i])
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold),
                    Cell::new(values[i])
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold)
                        .fg(Color::Green),
                ]));
            }

            println!(
                "{}\n{} {}",
                t,
                "Formula:".bold().yellow(),
                formula.join("").bold()
            )
        }

        false => println!(
            "{}: {}\n{}: {}\n\nFormula: {}",
            units[0],
            values[0],
            units[1],
            values[1],
            formula.join("")
        )
    }
}
