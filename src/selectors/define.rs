use crossterm::style::Stylize;
use scraper::Selector;

use crate::selectors::get_vec;

// this looks like bait for updoots on r/badcode

pub fn tty(data: &scraper::Html, length_max: usize) {
    let word = get_vec(data, "div.c8d6zd.ya2TWb.DgZBFd");
    let pronounce = get_vec(data, "div.S23sjd");
    println!(
        "{}\t{} {}\n",
        word[0].bold(),
        "pronounced:".dark_grey(),
        pronounce[1].magenta().bold()
    );

    let mut classes = vec![];
    for x in data.select(&Selector::parse("div.lW8rQd").unwrap()) {
        classes.push(x.text().collect::<Vec<&str>>())
    }

    let mut definitions = vec![];
    for x in data.select(&Selector::parse("ol.eQJLDd").unwrap()) {
        definitions.push(x.text().collect::<Vec<&str>>())
    }

    let total = classes.len();
    assert_eq!(total, definitions.len());

    for i in 0..total {
        classes[i].remove(0);

        let mut dim = true;
        let mut example = false;
        let mut thesaurus = false;
        let mut thes_vec: Vec<&str> = vec![];
        let mut sim_op = 0;
        let mut sad_noises_intensify = false;

        for x in classes[i].iter() {
            match *x {
                ": " => {
                    print!("{}", ": ".dark_grey());
                    dim = false
                }
                "; " => {
                    println!();
                    dim = true
                }
                "British" => println!("{}", "[British]".dark_grey()),
                "informal" => println!("{}", "[informal]".dark_grey()),
                _ => match dim {
                    true => print!("{}", x.dark_grey()),
                    false => print!("{}", x.bold().magenta())
                }
            }
        }
        println!("\n");

        for x in definitions[i].iter() {
            let num = match x.parse::<u8>() {
                Ok(n) => {
                    example = false;
                    if thesaurus {
                        thesaurus = false;
                        if !thes_vec.is_empty() {
                            print!(
                                "{}",
                                format_string(length_max, thes_vec.clone(), sim_op, true)
                            );
                            thes_vec.clear()
                        }
                    }
                    if n > 1 {
                        println!("\n")
                    }
                    print!("{}", x.dark_grey());
                    n
                }
                Err(_) => 0
            };

            if num == 0 {
                let chop = match example {
                    true => (x.len() + 4) > length_max,
                    false => x.len() > length_max
                };

                match *x {
                    " h " => {
                        match thesaurus {
                            true => {
                                if !thes_vec.is_empty() {
                                    print!(
                                        "{}",
                                        format_string(length_max, thes_vec.clone(), sim_op, true)
                                    );
                                    thes_vec.clear()
                                }
                            }
                            false => {
                                thesaurus = true;
                                example = false;
                            }
                        }
                        println!("\n");
                    }

                    ". " => {
                        print!("{}", x.dark_grey());
                        sad_noises_intensify = true
                    }

                    "Similar:" => {
                        print!("{} ", "Similar:".green().bold());
                        sim_op = 0
                    }

                    "Opposite:" => {
                        print!("{} ", "Opposite:".red().bold());
                        sim_op = 1
                    }

                    _ if thesaurus => thes_vec.push(x),

                    _ if sad_noises_intensify => {
                        match x.as_bytes().iter().any(u8::is_ascii_whitespace) {
                            true => {
                                match chop {
                                    true => print!(
                                        "\n{}",
                                        format_string(
                                            length_max,
                                            x.split(' ').collect::<Vec<&str>>(),
                                            0,
                                            false
                                        )
                                        .bold()
                                    ),
                                    false => print!("\n{}", x.bold())
                                }
                                if x.ends_with('.') {
                                    example = true;
                                    print!("\n    ")
                                }
                            }
                            false => println!("{}", ["[", x, "]"].join("").dark_grey())
                        }
                        sad_noises_intensify = false
                    }

                    _ => match example {
                        true if chop => print!(
                            "{}",
                            format_string(
                                length_max,
                                x.split(' ').collect::<Vec<&str>>(),
                                4,
                                false
                            )
                            .yellow()
                            .bold()
                        ),
                        true => print!("{}", x.yellow().bold()),
                        false => {
                            match chop {
                                true => print!(
                                    "\n{}",
                                    format_string(
                                        length_max,
                                        x.split(' ').collect::<Vec<&str>>(),
                                        0,
                                        false
                                    )
                                    .bold()
                                ),
                                false => print!("{}", x.bold())
                            }
                            if x.ends_with('.') {
                                example = true;
                                print!("\n    ")
                            }
                        }
                    }
                }
            }
        }

        if thesaurus && !thes_vec.is_empty() {
            print!(
                "{}",
                format_string(length_max, thes_vec.clone(), sim_op, true)
            );
            thes_vec.clear()
        }
        println!("\n")
    }
}

pub fn raw(data: &scraper::Html) {
    let word = get_vec(data, "div.c8d6zd.ya2TWb.DgZBFd");
    let pronounce = get_vec(data, "div.S23sjd");
    println!("{}\tpronounced: {}\n", word[0], pronounce[1]);

    let mut classes = vec![];
    for x in data.select(&Selector::parse("div.lW8rQd").unwrap()) {
        classes.push(x.text().collect::<Vec<&str>>())
    }

    let mut definitions = vec![];
    for x in data.select(&Selector::parse("ol.eQJLDd").unwrap()) {
        definitions.push(x.text().collect::<Vec<&str>>())
    }

    let total = classes.len();
    assert_eq!(total, definitions.len());

    for i in 0..total {
        classes[i].remove(0);

        let mut thesaurus = false;
        let mut thes_vec: Vec<&str> = vec![];
        let mut sad_noises_intensify = false;

        for x in classes[i].iter() {
            match *x {
                "; " => println!(),
                "British" => println!("[British]"),
                "informal" => println!("[informal]"),
                _ => print!("{}", x)
            }
        }
        println!("\n");

        for x in definitions[i].iter() {
            let num = match x.parse::<u8>() {
                Ok(n) => {
                    if thesaurus {
                        thesaurus = false;
                        if !thes_vec.is_empty() {
                            print!("{}", thes_vec.join(", "));
                            thes_vec.clear()
                        }
                    }
                    if n > 1 {
                        println!("\n")
                    }
                    print!("{}", x);
                    n
                }
                Err(_) => 0
            };

            if num == 0 {
                match *x {
                    " h " => {
                        match thesaurus {
                            true => {
                                if !thes_vec.is_empty() {
                                    print!("{}", thes_vec.join(", "));
                                    thes_vec.clear()
                                }
                            }
                            false => thesaurus = true
                        }
                        println!("\n");
                    }

                    ". " => {
                        print!("{}", x);
                        sad_noises_intensify = true
                    }

                    "Similar:" | "Opposite:" => print!("{} ", x),

                    _ if thesaurus => thes_vec.push(x),

                    _ if sad_noises_intensify => {
                        match x.as_bytes().iter().any(u8::is_ascii_whitespace) {
                            true => {
                                print!("\n{}", x);
                                if x.ends_with('.') {
                                    println!()
                                }
                            }
                            false => println!("[{}]", x)
                        }
                        sad_noises_intensify = false
                    }

                    _ => {
                        print!("{}", x);
                        if x.ends_with('.') {
                            println!()
                        }
                    }
                }
            }
        }

        if thesaurus && !thes_vec.is_empty() {
            print!("{}", thes_vec.join(", "));
            thes_vec.clear()
        }
        println!("\n")
    }
}

fn format_string(length_max: usize, s: Vec<&str>, padding: usize, thesaurus: bool) -> String {
    let (sep1, sep2, spacing, extra): (&str, &str, usize, usize) = match thesaurus {
        true => (", ", ",\n", 2, 9),
        false => (" ", "\n", 1, 0)
    };
    let mut length = padding + extra;
    let mut build_vec = vec![];
    let mut r: Vec<String> = vec![];

    for x in &s {
        match (x.len() + length + spacing) > length_max {
            true => {
                r.push(build_vec.join(sep1));
                build_vec.clear();
                match padding == 4 {
                    true => {
                        build_vec.push("    ");
                        length = x.len() + spacing + padding
                    }
                    false => length = x.len() + spacing
                }
            }
            false => length += x.len() + spacing
        }
        build_vec.push(*x);
    }

    r.push(build_vec.join(sep1));
    r.join(sep2)
}
