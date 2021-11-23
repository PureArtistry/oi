use crate::selectors::{default_output, get_vec};

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let x = get_vec(data, "div.HwtpBd.gsrt.PZPZlf.kTOYnf");
    match tty {
        true => default_output(x[0], w),
        false => println!("{}", x[0])
    }
}
