use mercy;
use clap::Parser;

// TODO: Add hex_dump functionality from Mercy

#[derive(Parser, Debug)]
#[command(name = "Lucy")]
#[command(version, author)] // Reads from Cargo.toml
#[command(about = "Lucy is a simple command line interface built on top of the Mercy Rust crate", long_about = None)]
struct Args {

    /// Encoded/Plaintext string for decoding/encoding (ex: IaMStr1Ng)
    #[arg(short, long)]
    input: String,

    /// Chosen method for data manipulation (ex: decode)
    #[arg(short, long)]
    method: String,

    /// Chosen protocol for data manipulation (ex: base64)
    #[arg(short, long)]
    protocol: String
}

fn main() {
    let args = Args::parse();

    match args.method.as_str() {

        "decode" => {
            println!("{}",mercy::mercy_decode(&args.protocol, &args.input));
        },

        "encode" => {
            println!("{}", mercy::mercy_encode(&args.protocol, &args.input));
        },

        "hash" => {
            println!("{}", mercy::mercy_hash(&args.protocol, &args.input));
        },

        "sys" => {
            println!("{}", mercy::mercy_extra(&args.protocol, "all"));
            println!("{}", mercy::mercy_extra("internal_ip", ""));
        },

        _ => {
            println!("Unable to parse provided arguments. Please check inputs");
        }
    }
}