use crate::selectors::{default_output, get_vec};

pub fn main(data: &scraper::Html, tty: &bool, w: usize) {
    let mut x = get_vec(data, "div.i4J0ge");
    for _ in 0..4 {
        x.pop();
    }
    let y = get_vec(data, "div.ujudUb.WRZytc.OULBYb");
    let j = y.len();
    let mut i = x.iter().position(|&r| r == y[(j - 1)]).unwrap();
    for _ in 0..j {
        x.remove(i);
        i -= 1
    }
    match tty {
        true => default_output(&x.join("\n"), w),
        false => println!("{}", x.join("\n"))
    }
}
