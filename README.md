# EasyCipher

An easy-to-use CLI-based cipher suite.

## Usage

```
EasyCipher 0.1.0

An easy-to-use CLI-based cipher suite.

USAGE:
    easy_cipher.exe [OPTIONS] <MODE> <FILE>

ARGS:
    <MODE>    Sets a mode (e: encryption | d: decryption)
    <FILE>    Sets an input file path

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -o, --output <OUTPUT DIR>    Sets an output directory
    -s, --secret <SECRET>        Sets a secret
```

- In Encryption Mode, "secret" is a seed
- In Decryption Mode, "secret" is a decryption key

## Commands

### Build Debug

```shell
cargo build
```

### Build Release

```shell
cargo build --release
```