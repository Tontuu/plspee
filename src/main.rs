use std:: {
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    env::args,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("
Unknown way to use the computer.
Usage:

    plspee <file>.

have a good day!
");
        return;
    }

    let path = &args[1];

    let lines = lines_from_file(path);
    let mut bad_text = String::new();
    let mut i:usize = 0;

    loop {
        for word in lines[i].split_whitespace() {
            if word.contains(").") {
                bad_text.push_str("pee-pee). ");
            } else if word.contains("),") {
                bad_text.push_str("schlong), ");
            } else if word.contains("}.") {
                bad_text.push_str("schlong}. ");
            } else if word.contains("},") {
                bad_text.push_str("schlong}, ");
            } else if word.contains('.') {
                bad_text.push_str("dick. ");
            } else if word.contains(',') {
                bad_text.push_str("cock, ");
            } else if  word.contains('?') {
                bad_text.push_str("cock? ");
            } else if word.contains('!') {
                bad_text.push_str("dick! ");
            } else if word.contains("()") {
                bad_text.push_str("penis() ");
            } else if word.contains(';') {
                bad_text.push_str("dingus;");
            } else if word.contains(':') {
                bad_text.push_str("slim jim;");
            } else if word.contains('{') {
                bad_text.push_str("{schlong ");
            } else if word.contains('}') {
                bad_text.push_str("schlong} ");
            } else if word.contains('(') {
                bad_text.push_str("(pee-pee ");
            } else if word.contains(')') {
                bad_text.push_str("pee-pee) ");
            } else if word.contains('(') && word.contains(')') {
                bad_text.push_str("(pee-pee)");
            } else if word.contains('(') && word.contains("),") {
                bad_text.push_str("(pee-pee),");
            } else if word.contains('(') && word.contains(").") {
                bad_text.push_str("(pee-pee).");
            } else if word.contains('{') && word.contains('}') {
                bad_text.push_str("{schlong}");
            } else if word.contains('{') && word.contains("},") {
                bad_text.push_str("{schlong},");
            } else if word.contains('{') && word.contains("}.") {
                bad_text.push_str("{schlong}.");
            } else {
                bad_text.push_str("penis ");
            }
        }
        bad_text.push('\n');

        if i == lines.len()-1 { break; }
        i+=1;
    }
    println!("{}", bad_text.trim());
}
