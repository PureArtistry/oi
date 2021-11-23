use crate::selectors::{default_output, get_vec};

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let x = get_vec(data, "div.kno-rdesc");
    match tty {
        true => default_output(x[1], w),
        false => println!("{}", x[1])
    }
}
