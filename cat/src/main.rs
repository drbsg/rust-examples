extern crate getopts;

use getopts::Options;
use std::env;
use std::io;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Display this usage information.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }

    for filename in matches.free {
        if let Err(e) = print_file(&filename) {
            panic!(e.to_string());
        }
    }
}

fn print_file(filename: &str) -> io::Result<()> {
    let file = try!(File::open(filename));
    let mut reader = BufReader::new(file);
    try!(io::copy(&mut reader, &mut io::stdout()));
    Ok(())
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] file...", program);
    print!("{}", opts.usage(&brief));
}
