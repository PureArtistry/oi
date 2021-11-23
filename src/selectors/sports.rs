use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::{UTF8_BORDERS_ONLY, UTF8_FULL},
    Attribute, Cell, CellAlignment, Color, ContentArrangement, Row, Table
};

use crate::selectors::get_vec;

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let heading = get_vec(data, "div.imso_mh__stts-l.imso-ani.imso_mh__stts-l-cont");
    let scores = get_vec(data, "div.imso_mh__scr-sep");
    let team1 = get_vec(
        data,
        "div.imso_mh__first-tn-ed.imso_mh__tnal-cont.imso-tnol"
    );
    let team2 = get_vec(
        data,
        "div.imso_mh__second-tn-ed.imso_mh__tnal-cont.imso-tnol"
    );

    match heading.len() {
        3 => match tty {
            true => {
                let mut t = Table::new();
                t.load_preset(UTF8_BORDERS_ONLY);
                t.apply_modifier(UTF8_ROUND_CORNERS);
                t.set_content_arrangement(ContentArrangement::Dynamic);
                if w > 100 {
                    t.set_table_width(100);
                }
                t.set_header(Row::from(vec![
                    Cell::new(heading[0]).set_alignment(CellAlignment::Center),
                    Cell::new(heading[1])
                        .set_alignment(CellAlignment::Center)
                        .fg(Color::DarkGrey),
                    Cell::new(heading[2])
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold)
                        .fg(Color::Magenta),
                ]));
                t.add_row(Row::from(vec![
                    Cell::new(team1[0])
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold),
                    Cell::new(scores[0])
                        .set_alignment(CellAlignment::Center)
                        .fg(Color::DarkGrey),
                    Cell::new(team2[0])
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold),
                ]));
                println!("{}", t)
            }
            false => println!(
                "{}\n{} {} {}",
                heading.join(""),
                team1[0],
                scores[0],
                team2[0]
            )
        },
        4 => match tty {
            true => {
                let mut t = Table::new();
                t.load_preset(UTF8_FULL);
                t.apply_modifier(UTF8_ROUND_CORNERS);
                t.apply_modifier(UTF8_SOLID_INNER_BORDERS);
                t.set_content_arrangement(ContentArrangement::Dynamic);
                if w > 100 {
                    t.set_table_width(100);
                }
                let mut y = vec![];
                for i in 1..4 {
                    y.push(heading[i])
                }
                t.set_header(Row::from(vec![
                    Cell::new(y.join(""))
                        .set_alignment(CellAlignment::Center)
                        .fg(Color::DarkGrey),
                    Cell::new(heading[0])
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold)
                        .fg(Color::Green),
                ]));
                let a = scores[0].parse::<u32>().unwrap();
                let b = scores[2].parse::<u32>().unwrap();
                if a == b {
                    t.add_row(Row::from(vec![
                        Cell::new(team1.join("\n"))
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                        Cell::new(scores[0])
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                    ]));
                    t.add_row(Row::from(vec![
                        Cell::new(team2.join("\n"))
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                        Cell::new(scores[2])
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                    ]));
                }
                else if a > b {
                    t.add_row(Row::from(vec![
                        Cell::new(team1.join("\n"))
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Yellow),
                        Cell::new(scores[0])
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Yellow),
                    ]));
                    t.add_row(Row::from(vec![
                        Cell::new(team2.join("\n"))
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                        Cell::new(scores[2])
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                    ]));
                }
                else {
                    t.add_row(Row::from(vec![
                        Cell::new(team1.join("\n"))
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                        Cell::new(scores[0])
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold),
                    ]));
                    t.add_row(Row::from(vec![
                        Cell::new(team2.join("\n"))
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Yellow),
                        Cell::new(scores[2])
                            .set_alignment(CellAlignment::Center)
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Yellow),
                    ]));
                }
                println!("{}", t)
            }
            false => println!(
                "{}{}{} | {}\n{} {}\n{} {}",
                heading[1],
                heading[2],
                heading[3],
                heading[0],
                team1.join(" "),
                scores[0],
                team2.join(" "),
                scores[2]
            )
        },
        _ => panic!("sports: heading.len() != 3||4; result can not be formatted")
    }
}
