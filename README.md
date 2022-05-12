## About

Cli utility to encrypt or decrypt files using merkle-hellman knapsack cryptosystem

## Usage

### Help

To get help type

```sh
./knapsack --help
```

or

```sh
./knapsack [command] --help
```

### Key generation

Generate private and public keys and save them in respective files

```sh
./knapsack keys
```

### Encryption

```sh
./knapsack encrypt --keyfile <path to public key> [file]
```

If no file is specified, it reads from stdin

### Decryption

```sh
./knapsack decrypt --keyfile <path to private key> [file]
```

If no file is specified, it reads from stdin

### Example

You can find usage example in [run.sh](./run.sh)
