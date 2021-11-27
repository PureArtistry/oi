use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Attribute, Cell, CellAlignment, Color, ContentArrangement, Row, Table
};

use crate::selectors::get_vec;

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let temps = get_vec(data, "div.vk_bk.TylWce");
    let mut c = vec![];
    let mut f = vec![];
    match temps[0].parse::<i8>().unwrap() < temps[1].parse::<i8>().unwrap() {
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
    let temps_display = [c.join(""), "|".to_string(), f.join("")];

    let misc = get_vec(data, "div.wtsRwe");
    let details = get_vec(data, "div.VQF4g");

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
            t.add_row(Row::from(vec![
                Cell::new("Description:")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::DarkGrey),
                Cell::new(details[2])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Green),
            ]));
            t.add_row(Row::from(vec![
                Cell::new("Temperature:")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::DarkGrey),
                Cell::new(temps_display.join(" "))
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Yellow),
            ]));
            t.add_row(Row::from(vec![
                Cell::new("Precipitation:")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::DarkGrey),
                Cell::new(misc[1])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold),
            ]));
            t.add_row(Row::from(vec![
                Cell::new("Humidity:")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::DarkGrey),
                Cell::new(misc[3])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold),
            ]));
            t.add_row(Row::from(vec![
                Cell::new("Wind:")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::DarkGrey),
                Cell::new(misc[5])
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold),
            ]));
            t.add_row(Row::from(vec![
                Cell::new("Location:")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::DarkGrey),
                Cell::new(details[0]).set_alignment(CellAlignment::Center),
            ]));
            t.add_row(Row::from(vec![
                Cell::new("Time:")
                    .set_alignment(CellAlignment::Center)
                    .fg(Color::DarkGrey),
                Cell::new(details[1]).set_alignment(CellAlignment::Center),
            ]));
            println!("{}", t)
        }
        false => {
            println!("Description: {}", details[2]);
            println!("Temperature: {}", temps_display.join(" "));
            println!("Precipitation: {}", misc[1]);
            println!("Humidity: {}", misc[3]);
            println!("Wind: {}", misc[5]);
            println!("Location: {}", details[0]);
            println!("Time: {}", details[1])
        }
    }
}
