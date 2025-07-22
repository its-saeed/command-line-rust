use clap::{Arg, ArgAction};

fn main() {
    let matches = clap::Command::new("echor")
        .version("0.1.0")
        .author("Saeed <s4eed@outlook.com>")
        .about("Rust echo")
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print new line")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("omit_delimiter")
                .short('s')
                .help("Do not separate texts")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .action(ArgAction::Append)
                .required(true),
        )
        .get_matches();

    let omit_delimiter = matches.get_flag("omit_delimiter");
    let delimiter = if omit_delimiter { "" } else { " " };
    let omit_newline = matches.get_flag("omit_newline");
    let texts = matches
        .get_many::<String>("text")
        .expect("At least one text should be inserted")
        .map(|s| s.as_str())
        .collect::<Vec<_>>();

    if omit_newline {
        print!("{}", texts.join(delimiter));
    } else {
        println!("{}", texts.join(delimiter));
    }
}
