use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_name = "STRING", multiple_values = true)]
    string: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("{}", cli.string.join(" "));
}
