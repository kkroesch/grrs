use std::path::PathBuf;
use clap::Parser;
use std::fs;


#[derive(Parser, Debug)]
#[command(name = "Beispielprogramm")]
#[command(version = "1.0")]
#[command(author = "Dein Name <deinemail@example.com>")]
#[command(about = "Ein Programm mit Flags und Argumenten")]
struct Cli {
    #[arg(help="Regexp pattern for searching in file.")]
    pattern: String,
    #[arg(help="The file to be processed.")]
    path: PathBuf,
    #[arg(short, long, help="Prints line numbers.")]
    numbering: bool
}


fn find_matches(content: &str, pattern: &str, numbering: bool) {
    for (num, line) in content.lines().enumerate() {
        if line.contains(pattern) && numbering {
            println!("{}: {}", num, line);
        }
        if line.contains(pattern) && !numbering {
            println!("{}", line);
        }
    }
}


fn main() {
    let args = Cli::parse();
    let content = fs::read_to_string(&args.path).expect("could not read file");
    find_matches(&content, &args.pattern, args.numbering);
}
