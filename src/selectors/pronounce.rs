use crate::selectors::{default_output, get_vec};

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let x = get_vec(data, "div.TQ7enb");
    match tty {
        true => default_output(&x.join(""), w),
        false => println!("{}", x.join(""))
    }
}
