use crossterm::style::Stylize;
use scraper::Selector;

// this looks like bait for updoots on r/badcode

pub fn tty(data: &scraper::Html, length_max: usize) {
    let define = data
        .select(&Selector::parse("div.VpH2eb.vmod").unwrap())
        .next()
        .unwrap();

    let word = define
        .select(&Selector::parse("div.c8d6zd.ya2TWb.DgZBFd").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<Vec<&str>>();
    let pronounce = define
        .select(&Selector::parse("div.S23sjd").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<Vec<&str>>();
    println!(
        "{}\t{} {}\n",
        word[0].bold(),
        "pronounced:".dark_grey(),
        pronounce[1].magenta().bold()
    );

    for x in define.select(&Selector::parse(r#"div[jsname="r5Nvmf"]"#).unwrap()) {
        let mut dim = true;
        let mut thesaurus = false;
        let mut thes_vec = vec![];
        let mut sim_op = 2;
        let mut example = false;
        let mut example_vec = vec![];
        let mut blank_line = true;

        let classes = x
            .select(&Selector::parse("div.lW8rQd").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>();
        for y in classes.iter().skip(1) {
            match *y {
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
                    true => print!("{}", y.dark_grey()),
                    false => print!("{}", y.bold().magenta())
                }
            }
        }
        println!("\n");

        let definitions = x
            .select(&Selector::parse("ol.eQJLDd").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>();
        for y in &definitions {
            let num = match y.parse::<u8>() {
                Ok(n) => {
                    if thesaurus {
                        thesaurus = false;
                        if !thes_vec.is_empty() {
                            println!(
                                "{}\n",
                                format_string(length_max, thes_vec.clone(), sim_op, true)
                            );
                            thes_vec.clear()
                        }
                    }
                    print!("{}", y.dark_grey());
                    n
                }
                Err(_) => 0
            };

            if num == 0 {
                match *y {
                    ". " => print!("{}", y.dark_grey()),

                    " h " => match thesaurus {
                        true => {
                            if !thes_vec.is_empty() {
                                println!(
                                    "{}\n",
                                    format_string(length_max, thes_vec.clone(), sim_op, true)
                                );
                                thes_vec.clear()
                            }
                        }
                        false => thesaurus = true
                    },

                    "Similar:" => {
                        print!("{} ", "Similar:".green().bold());
                        sim_op = 0
                    }

                    "Opposite:" => {
                        print!("{} ", "Opposite:".red().bold());
                        sim_op = 1
                    }

                    _ if y.ends_with('.') => {
                        if thesaurus {
                            if !thes_vec.is_empty() {
                                println!(
                                    "{}\n",
                                    format_string(length_max, thes_vec.clone(), sim_op, true)
                                );
                                thes_vec.clear();
                                blank_line = true;
                            }
                            thesaurus = false
                        }
                        match blank_line {
                            true => blank_line = false,
                            false => {
                                println!();
                                blank_line = true;
                            }
                        }
                        match y.len() > length_max {
                            true => println!(
                                "{}",
                                format_string(
                                    length_max,
                                    y.split(' ').collect::<Vec<&str>>(),
                                    0,
                                    false
                                )
                                .bold()
                            ),
                            false => println!("{}", y.bold())
                        }
                    }

                    _ if y.starts_with('"') && y.ends_with('"') => {
                        match (y.len() + 4) > length_max {
                            true => println!(
                                "    {}\n",
                                format_string(
                                    length_max,
                                    y.split(' ').collect::<Vec<&str>>(),
                                    4,
                                    false
                                )
                                .yellow()
                                .bold()
                            ),
                            false => println!("    {}\n", y.yellow().bold())
                        }
                        blank_line = true
                    }

                    _ if y.starts_with('"') => {
                        example = true;
                        example_vec.push(y.trim());
                    }

                    _ if y.ends_with('"') => {
                        example = false;
                        example_vec.push(y.trim());
                        println!(
                            "    {}\n",
                            format_string(length_max, example_vec.clone(), 4, false)
                                .yellow()
                                .bold()
                        );
                        example_vec.clear();
                    }

                    _ if example => example_vec.push(y.trim()),
                    _ if thesaurus => thes_vec.push(y),

                    _ => {} // TODO: there are some edge cases that could be handled here
                }
            }
        }

        if thesaurus && !thes_vec.is_empty() {
            println!(
                "{}\n",
                format_string(length_max, thes_vec.clone(), sim_op, true)
            );
            thes_vec.clear();
        }
    }
}

pub fn raw(data: &scraper::Html) {
    let define = data
        .select(&Selector::parse("div.VpH2eb.vmod").unwrap())
        .next()
        .unwrap();

    let word = define
        .select(&Selector::parse("div.c8d6zd.ya2TWb.DgZBFd").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<Vec<&str>>();
    let pronounce = define
        .select(&Selector::parse("div.S23sjd").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<Vec<&str>>();
    println!("{}\tpronounced: {}\n", word[0], pronounce[1]);

    for x in define.select(&Selector::parse(r#"div[jsname="r5Nvmf"]"#).unwrap()) {
        let mut thesaurus = false;
        let mut thes_vec: Vec<&str> = vec![];
        let mut example = false;
        let mut example_vec = vec![];
        let mut blank_line = true;

        let classes = x
            .select(&Selector::parse("div.lW8rQd").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>();
        for y in classes.iter().skip(1) {
            match *y {
                "; " => println!(),
                "British" => println!("[British]"),
                "informal" => println!("[informal]"),
                _ => print!("{}", y)
            }
        }
        println!("\n");

        let definitions = x
            .select(&Selector::parse("ol.eQJLDd").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>();
        for y in &definitions {
            let num = match y.parse::<u8>() {
                Ok(n) => {
                    if thesaurus {
                        thesaurus = false;
                        if !thes_vec.is_empty() {
                            println!("{}\n", thes_vec.join(", "));
                            thes_vec.clear()
                        }
                    }
                    print!("{}", y);
                    n
                }
                Err(_) => 0
            };

            if num == 0 {
                match *y {
                    ". " => print!("{}", y),

                    " h " => match thesaurus {
                        true => {
                            if !thes_vec.is_empty() {
                                println!("{}\n", thes_vec.join(", "));
                                thes_vec.clear()
                            }
                        }
                        false => thesaurus = true
                    },

                    "Similar:" | "Opposite:" => {
                        print!("{} ", y);
                    }

                    _ if y.ends_with('.') => {
                        if thesaurus {
                            if !thes_vec.is_empty() {
                                println!("{}\n", thes_vec.join(", "));
                                thes_vec.clear();
                                blank_line = true;
                            }
                            thesaurus = false
                        }
                        match blank_line {
                            true => blank_line = false,
                            false => {
                                println!();
                                blank_line = true;
                            }
                        }
                        println!("{}", y)
                    }

                    _ if y.starts_with('"') && y.ends_with('"') => {
                        println!("{}\n", y);
                        blank_line = true
                    }

                    _ if y.starts_with('"') => {
                        example = true;
                        example_vec.push(y.trim());
                    }

                    _ if y.ends_with('"') => {
                        example = false;
                        example_vec.push(y.trim());
                        println!("{}\n", example_vec.join(" "));
                        example_vec.clear();
                    }

                    _ if example => example_vec.push(y.trim()),
                    _ if thesaurus => thes_vec.push(y),

                    _ => {} // TODO: there are some edge cases that could be handled here
                }
            }
        }

        if thesaurus && !thes_vec.is_empty() {
            println!("{}\n", thes_vec.join(", "));
            thes_vec.clear();
        }
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
