use crate::selectors::{default_output, get_vec};

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let mut lyrics = get_vec(data, "div.i4J0ge");
    for _ in 0..4 {
        lyrics.pop();
    }
    let duplicate_lines = get_vec(data, "div.ujudUb.WRZytc.OULBYb");
    let dups_len = duplicate_lines.len();
    let mut lyrics_index = lyrics
        .iter()
        .position(|&r| r == duplicate_lines[(dups_len - 1)])
        .unwrap();
    for _ in 0..dups_len {
        lyrics.remove(lyrics_index);
        lyrics_index -= 1
    }
    match tty {
        true => default_output(&lyrics.join("\n"), w),
        false => println!("{}", lyrics.join("\n"))
    }
}
