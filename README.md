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

### Examples

- Encrypt `path_to_file`

```shell
easy_cipher(.exe) e path_to_file -s="secret seed"
```

- Decrypt `path_to_file`

```shell
easy_cipher(.exe) d path_to_file -s="decryption key"
```

- Decrypt `path_to_file` with `path_to_key`

```shell
easy_cipher(.exe) d path_to_file -s="$(cat path_to_key)"
```

## Dev

### Build Debug

```shell
cargo build
```

### Build Release

```shell
cargo build --release
```