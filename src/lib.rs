use std::path;

use clap::Clap;
use colored::Colorize;

#[derive(Clap)]
pub struct Opts {
    pub filename: path::PathBuf,
    pub search: String,
    #[clap(short, long, default_value = "0")]
    pub context: i32,
}

pub fn get_opts() -> Opts {
    Opts::parse()
}

pub struct Grep {
    search: String,
    content: String,
    context: i32,
    matched_lines: Vec<usize>,
    all_lines: Vec<i32>,
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
        grep.find_match_lines();
        grep.find_context_lines();
        grep.print_lines();
    }

    fn find_match_lines(&mut self) {
        for (i, line) in self.content.lines().enumerate() {
            if !line.contains(&self.search) {
                continue
            }
            self.matched_lines.push(i);
            self.all_lines.push(i as i32);
        }
    }

    fn find_context_lines(&mut self) {
        for i in 1..self.context+1 {
            for j in self.matched_lines.iter() {
                let j = *j as i32;
                if i <= j {
                    if !self.all_lines.contains(&(j-i)) {
                        self.all_lines.push(j-i);
                    }
                }

                if j + i < self.content.lines().count() as i32 {
                    if !self.all_lines.contains(&(j + i)) {
                        self.all_lines.push(j+i);
                    }
                }
            }
        }
    }

    fn print_lines(&self){
        for (i, line) in self.content.lines().enumerate() {
            if !self.all_lines.contains(&(i as i32)) {
                continue;
            }
            if self.matched_lines.contains(&i) {
                let line = line.replace(&self.search, &self.search.red().to_string());
                let line = format!("{}| {}", i+1, line);
                println!("{}", line.bold());
            } else {
                println!("{}| {}", i+1, line);
            }
        }
    }
}