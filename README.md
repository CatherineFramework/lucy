<h1 align="center">
    <img src="https://raw.githubusercontent.com/CatherineFramework/mercy/main/assets/mercy_icon.png" width="40%" />
</h1>

Mercy CLI is a simple command line interface built on top of the [Mercy](https://github.com/CatherineFramework/mercy) Rust crate. Mercy CLI takes the entirety of Mercy and converts it into an easy-to-use CLI for quick security assessments and demonstrates the crate's functionality.

The Mercy crate is freely available to everyone, you can learn more about it on [crates.io](https://crates.io/crates/mercy).

### Usage

You can go over to GitHub releases to download the latest verson of Mercy CLI. Or you can install the CLI via Cargo:

```bash
cargo install mercy-cli
```

Once the executable has been downloaded, you can run the CLI tool using the following syntax:
```bash
./mercy-cli -m <METHOD> -p <PROTOCOL> -i <STRING/FILE>
```

You can also run the help command if you need a refresher on the available arguments:
```bash
./mercy-cli -h
```

The available options are listed below:
```
-i, --input     Encoded/Plaintext string for decoding/encoding (ex: IaMStr1Ng) + location of the file for hex_dump

-m, --method    Chosen method for data manipulation (ex: decode)

-p, --protocol  Chosen protocol for data manipulation (ex: base64)

-e, --extended  View every available option within the Mercy CLI
```

### Examples

Here are some quick examples of use cases:

If you need to decode a string using the base64 protocol.
```bash
./mercy-cli -m decode -p base64 -i <EncodedString>
```

Print host system information, such as hostname, cpu cores, etc.
```bash
./mercy-cli -m sys -p system_info -i all
```

Take a plaintext string and encode it using MD5.
```bash
./mercy-cli -m hash -p md5 -i <PlaintextString>
```

Print the internal IP address of your host system.
```bash
./mercy-cli -m ip -p internal_ip
```

Quickly check if a domain is malicious.
```
./mercy-cli -m mal -p status -i "azazelm3dj3d.com"
```

If you're stuck, you can use this option to learn every command at your disposal from [Mercy](https://github.com/CatherineFramework/mercy):
```bash
./mercy-cli -e
```