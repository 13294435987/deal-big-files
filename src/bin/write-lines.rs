use std::{env::args, fs::File, io::Write};

fn main() {
    let args =args().collect::<Vec<_>>();
    let n = args[1].parse::<usize>().unwrap();
    println!("will write {} lines", n);

    let mut file = File::create("./data/a.txt").unwrap();
    for line in Lines::new().take(n) { 
        file.write_all(line.as_bytes()).unwrap();
    }
    print!("done!")
}

struct Lines(usize);

impl Lines {
    fn new() -> Lines {
        Lines(0)
    }
}

impl Iterator for Lines {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let n : usize = 1000000000 + self.0;
        let line: String = format!("{}\n", n);
        self.0 += 1;
        Some(line)
    }
}