use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_name = "STRING", multiple_values = true)]
    string: Vec<String>,

    #[clap(
        short = 'n',
        help = "Do not output the trailing newline",
        takes_value = false
    )]
    omit_newline: bool,
}

fn main() {
    let cli = Cli::parse();

    print!(
        "{}{}",
        cli.string.join(" "),
        if cli.omit_newline { "" } else { "\n" }
    );
}
