use std::fs::File;
use std::io::Read;

use grep_lite::*;

fn main() {

    let opts = get_opts();

    let mut f = File::open(&opts.filename)
        .expect("cannot open file text.txt");

    let mut text = String::new();
    f.read_to_string(&mut text)
        .expect("cannot read file text.txt");

    Grep::run(&text, &opts);
}