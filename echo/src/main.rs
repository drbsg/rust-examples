extern crate getopts;

use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("n", "", "Do not print the trailing newline.");
    opts.optflag("h", "help", "Display this usage information.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }
    let suppress_newline = matches.opt_present("n");

    print!("{}", matches.free.join(" "));
    if !suppress_newline { print!("\n") }
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] text...", program);
    print!("{}", opts.usage(&brief));
}
