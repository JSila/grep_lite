use grep_lite::*;

fn main() {

    let opts = get_opts();

    let text = get_file_content(&opts.filename);

    Grep::run(&text, &opts);
}