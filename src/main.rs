/*
    Owner: Hifumi1337 (https://github.com/hifumi1337)
    Project: Lucy
    License: BSD 2-Clause
*/

use mercy;
use clap::Parser;

use prettytable::{
    Cell,
    Row,
    Table
};

#[derive(Parser, Debug)]
#[command(name = "Lucy")]
#[command(version)] // Reads from Cargo.toml
#[command(about = "Lucy is a simple command line interface built on top of the Mercy Rust crate", long_about = None)]
struct Args {

    /// Encoded/Plaintext string for decoding/encoding (ex: IaMStr1Ng) + location of the file for hex_dump
    #[arg(short, long)]
    #[clap(default_value = "")]
    input: String,

    /// Chosen method for data manipulation (ex: decode)
    #[arg(short, long)]
    #[clap(default_value = "")]
    method: String,

    /// Chosen protocol for data manipulation (ex: base64)
    #[arg(short, long)]
    #[clap(default_value = "")]
    protocol: String,

    /// View every available option within the Lucy CLI
    #[arg(short, long)]
    extended: bool
}

fn pretty_output(input: &str, output: &str, left_col: &str, right_col: &str) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new(left_col),
        Cell::new(right_col)
    ]));

    table.add_row(Row::new(vec![
        Cell::new(input),
        Cell::new(output)
    ]));

    table.printstd();
}

fn main() {
    let args = Args::parse();

    if args.extended {
        println!("\n=== Lucy ===");
        pretty_output("encode\ndecode\nhash\nhex\nsys\nip", "base64, rot13\nbase64, rot13\nmd5, sha2_256\nhex_dump\nsystem_info\ninternal_ip", "Method(s)", "Protocol(s)");

        println!("\n=== Lucy Extended ===");
        pretty_output("system_info", "hostname\ncpu_cores\ncpu_speed\nos_release\nproc\nall", "Protocol(s)", "Input(s)");

        println!("\n=== Examples ===");
        println!("Print general information for the local system");
        println!("./lucy -m sys -p system_info -i all\n");

        println!("Decode an encoded string using base64");
        println!("./lucy -m decode -p base64 -i dW1pa28gbGFicyBpcyB0aGUgYmVzdCB3YWlmdQ==\n");
    } else {
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
    
            "hex" => {
                println!("{}", mercy::mercy_hex(&args.protocol, &args.input));
            },
    
            "sys" => {
                println!("{}", mercy::mercy_extra(&args.protocol, &args.input));
            },
    
            "ip" => {
                println!("{}", mercy::mercy_extra(&args.protocol, &args.input));
            },
    
            _ => {
                println!("Unable to parse provided arguments");
            }
        }
    }
}