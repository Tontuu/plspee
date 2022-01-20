use std:: {
    fs::File,
    io::{ prelude::*, BufReader },
    env::args,
};

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

    
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);

    let lines:Vec<String> = 
        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

    let mut bad_text = String::new();
    let word_to = "penis";
    let mut i:usize = 0;

    loop {
       for word in lines[i].split_whitespace() {
            let word_from = word;

            ['`','#','(', '{','[','<','!', '\'']
                .into_iter()
                .for_each(|arg| {
                    if word.contains(arg) {
                        bad_text.push(arg); 
                    }
                });

            bad_text.push_str(word.replace(&word_from, word_to).as_str());

            ['.', ',', ';', ')', '}',']','>','`', '!', '\'']
                .into_iter()
                .for_each(|arg| {
                    if word.contains(arg) {
                        bad_text.push(arg);
                    }
                });
            bad_text.push(' ');
        }
       bad_text.push('\n');

       if i == lines.len()-1 { break; }
       i+=1;
    }
 
    println!("\nFinal String: \n{}", bad_text.trim());
}
