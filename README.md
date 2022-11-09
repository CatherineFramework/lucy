# Lucy
Lucy is a simple command line interface built on top of the Mercy Rust crate. Lucy takes the entirety of Mercy and converts it into an easy to use CLI for quick security assessments and demonstrates the custom crate.

### Usage
```bash
cargo build
```

```bash
./lucy -m <METHOD> -p <PROTOCOL> -i <STRING>
```

You can also run the help command:
```
./lucy -h
```

### Examples
```bash
./lucy -m decode -p base64 -i <EncodedString>
./lucy -m encode -p base64 -i <PlaintextString>
./lucy -m hash -p md5 -i <PlaintextString>
```