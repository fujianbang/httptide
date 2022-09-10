use clap::Parser;

#[derive(clap::ValueEnum, Debug, Clone)]
enum Method {
    GET,
    POST
}

/// HTTP-Tide for Terminal —— modern, user-friendly command-line HTTP client, inspired by [httpie]
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[clap(short, long, value_enum)]
    method: Method,
}

fn main() {
    let args = Cli::parse();

    for _ in 0..3 {
        println!("Hello {:?}", args.method)
    }
}
