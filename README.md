<h1 align="center">
    <img src="assets/lucy_logo.png" />
    <br />
    Lucy
</h1>

Lucy is a simple command line interface built on top of the [Mercy](https://github.com/CatherineFramework/mercy) Rust crate. Lucy takes the entirety of Mercy and converts it into an easy to use CLI for quick security assessments and demonstrates the crate's abilities.

### Usage

While Lucy is in an alpha state, the source will need to be built. You can easily build the source using the cargo package manager like so:

```bash
cargo build
```

Once the executable has been compiled, you can run the CLI tool using the following syntax:
```bash
./lucy -m <METHOD> -p <PROTOCOL> -i <STRING/FILE>
```

You can also run the help command if you need a refresher on the available arguments:
```bash
./lucy -h
```

We've collected the available commands and listed them below:
```
-i, --input     Encoded/Plaintext string for decoding/encoding (ex: IaMStr1Ng) + location of the file for hex_dump

-m, --method    Chosen method for data manipulation (ex: decode)

-p, --protocol  Chosen protocol for data manipulation (ex: base64)

-e, --extended  View every available option within the Lucy CLI
```

### Examples

Here are some quick examples for the `decode`, `sys`, `hash`, and `ip` arguments:

```bash
./lucy -m decode -p base64 -i <EncodedString>
```

```bash
./lucy -m sys -p system_info -i all
```

```bash
./lucy -m hash -p md5 -i <PlaintextString>
```

```bash
./lucy -m ip -p internal_ip
```

If you're still stuck, you can use this option to learn every command at your disposal:
```bash
./lucy -e
```