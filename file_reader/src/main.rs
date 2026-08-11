use std::{env, fs::File, io::{BufRead, BufReader}};


fn main() {
    println!("File Reader");

    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <file_path> --lines --search <keyword>");
    }

    let file_path = &args[1];
    
    let show_lines = &args.contains(&"--lines".to_string());

    let keyword = if let Some(pos) = args.iter().position(|x| x == "--search") {
        args.get(pos + 1)
    } else {
        None
    };
    
    if let Some(reader) = read_file(&file_path) {
        // start enumeration
        for (i, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(li) => li,
                Err(err) => {
                    eprintln!("Failed to read lines: {}", err);
                    continue;
                }
            };

            let matched = keyword.map_or(true, |k| line.contains(k));

            if matched {
                if *show_lines {
                    println!("{}: {}", i + 1, line);
                } else {
                    println!("{}", line);
                }
            }
        }
    }
}


fn read_file(file_path: &str) -> Option<BufReader<File>> {
    let file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return None;
        }
    };

    let reader = BufReader::new(file);

    Some(reader)
}
