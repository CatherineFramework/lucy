//! # Mercy CLI
//!
//! Mercy is a public Rust crate created to assist with building cybersecurity frameworks and assessment tools. This CLI was built on top of the Mercy Rust crate to showcase it's functionality in an easy to ingest format.
//! 

/*
    Project: Mercy CLI (https://github.com/mercy-cli)
    Author: azazelm3dj3d (https://github.com/azazelm3dj3d)
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
#[command(name = "Mercy CLI")]
#[command(version)] // Reads from Cargo.toml
#[command(about = "Mercy CLI is a simple command line interface built on top of the Mercy Rust crate.", long_about = None)]
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

    /// View every available option within the Mercy CLI
    #[arg(short, long)]
    extended: bool
}

// Creates a pretty output for the CLI
fn pretty_output(method: &str, protocol: &str, left_col: &str, right_col: &str) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new(left_col),
        Cell::new(right_col)
    ]));

    table.add_row(Row::new(vec![
        Cell::new(method),
        Cell::new(protocol)
    ]));

    table.printstd();
}

fn main() {
    let args = Args::parse();

    // Extended help section for new users
    if args.extended {
        println!("\n=== Mercy CLI ===");
        pretty_output("encode\ndecode\nhash\nhex\nsys\nip\nmal\nd\nwho", "base64, rot13\nbase64, rot13\nmd5, sha2_256\nhex_dump\nsystem_info\ninternal_ip\nstatus\ndefang\nwhois", "Method(s)", "Protocol(s)");

        println!("\n=== Mercy CLI Extended ===");
        pretty_output("system_info", "hostname\ncpu_cores\ncpu_speed\nos_release\nproc\nall", "Protocol(s)", "Input(s)");

        // Example scenarios
        println!("\n=== Examples ===");
        println!("Print general information for the local system");
        println!("./mercy-cli -m sys -p system_info -i all\n");

        println!("Decode an encoded string using base64");
        println!("./mercy-cli -m decode -p base64 -i bWVyY3kgaXMgcmVhbGx5IGNvb2w=\n");

        println!("Check if a domain is malicious or not");
        println!("./mercy-cli -m mal -p status -i 'azazelm3dj3d.com'\n");
    } else {
        match args.method.as_str() {

            // Available arguments from the Mercy crate
            "decode" => println!("{}",mercy::mercy_decode(&args.protocol, &args.input)),
            "encode" => println!("{}", mercy::mercy_encode(&args.protocol, &args.input)),
            "hash"   => println!("{}", mercy::mercy_hash(&args.protocol, &args.input)),
            "hex"    => println!("{}", mercy::mercy_hex(&args.protocol, &args.input)),
            "sys"    => println!("{}", mercy::mercy_extra(&args.protocol, &args.input)),
            "ip"     => println!("{}", mercy::mercy_extra(&args.protocol, &args.input)),
            "d"      => println!("{}", mercy::mercy_extra(&args.protocol, &args.input)),
            "who"    => println!("{}", mercy::mercy_extra(&args.protocol, &args.input)),
            "mal"    => println!("{}", mercy::mercy_malicious(&args.protocol, &args.input)),
            _        => println!("Unable to parse provided arguments")
        }
    }
}