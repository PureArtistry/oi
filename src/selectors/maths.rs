use crate::selectors::{default_output, get_vec};

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let x = get_vec(data, "span.qv3Wpe");
    let mut y = x[0].to_string();
    y.remove(y.len() - 1);
    y.remove(y.len() - 1);
    y.remove(0);

    match tty {
        true => default_output(&y, w),
        false => println!("{}", y)
    }
}
