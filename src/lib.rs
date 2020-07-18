use std::fs::File;
use std::io::Read;
use std::path;

use clap::Clap;
use colored::Colorize;

#[derive(Clap)]
pub struct Opts {
    pub filename: path::PathBuf,
    pub search: String,
    #[clap(short, long, default_value = "0")]
    pub context: usize,
}

pub fn get_opts() -> Opts {
    Opts::parse()
}

pub fn get_file_content(filename: &path::PathBuf) -> String {
    let mut f = File::open(filename)
        .expect("cannot open file text.txt");

    let mut text = String::new();
    f.read_to_string(&mut text)
        .expect("cannot read file text.txt");
    text
}

pub struct Grep {
    search: String,
    content: String,
    context: usize,
    matched_lines: Vec<usize>,
    all_lines: Vec<usize>,
}

impl Grep {
    pub fn new(content: &str, opts: &Opts) -> Grep {
        Grep {
            search: opts.search.to_string(),
            content: content.to_string(),
            context: opts.context,
            matched_lines: vec![],
            all_lines: vec![]
        }
    }

    pub fn run(content: &str, opts: &Opts) {
        let mut grep = Self::new(content, opts);
        grep.find_lines();
        grep.print_lines();
    }

    fn find_lines(&mut self) {
        for (i, line) in self.content.clone().lines().enumerate() {
            if !line.contains(&self.search) {
                continue
            }
            self.matched_lines.push(i);
            self.all_lines.push(i);

            self.find_context_lines(i);
        }
    }

    fn find_context_lines(&mut self, line_no: usize) {
        if self.context == 0 {
            return
        }

        for j in 1..self.context+1 {
            if j <= line_no {
                self.all_lines.push(line_no-j);
            }
            self.all_lines.push(line_no+j);
        }
    }

    fn print_lines(&self){
        for (i, line) in self.content.lines().enumerate() {
            if !self.all_lines.contains(&i) {
                continue;
            }
            if self.matched_lines.contains(&i) {
                let line = line.replace(&self.search, &self.search.red().to_string());
                let line = format!("{}| {}", i+1, line);
                println!("  {}", line.bold());
            } else {
                println!("* {}| {}", i+1, line);
            }
        }
    }
}