# Jawir

Jawir is a powerful command-line utility that simplifies the process of cracking and generating JSON Web Tokens (JWTs). Whether you're assessing token security or generating tokens for development, Jawir streamlines your workflow with ease.

## Features

- **JWT Cracking**: Perform dictionary attacks to crack JWTs with weak signing keys.
- **JWT Generation**: Create new JWTs using a specified secret, with support for both base64-encoded inputs and JSON format.

## Installation

Clone the repository and build the binary:
```sh
git clone https://github.com/dundorma/jawir.git
cd jawir
cargo build --release
```
```sh
./target/release/jawir --help
```

or if you use nix, you can also build the binary using nix-build:
```sh
git clone https://github.com/dundorma/jawir.git
cd jawir
nix-build
```
```sh
./result/bin/jawir --help
```

## Usage

### Cracking a JWT

Use the `crack` command to perform a dictionary attack on a JWT:

```sh
jawir crack --jwt <JWT> --wordlist <wordlist_path> --thread <thread_count>
```

*Default thread count is 1.*

**Example:**

```sh
jawir crack --jwt eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhZG1pbiI6ZmFsc2UsImlhdCI6MTUxNjIzOTAyMiwibmFtZSI6IkpvaG4gRG9lIiwic3ViIjoiMTIzNDU2Nzg5MCJ9._zpXlQezgYzPoc0EgVMd3F8cUtQtKGvxZU94bB_FU7U --wordlist ~/my_list/rockyou.txt --thread 4
```

### Generating a JWT

Jawir supports JWT generation in two formats:

#### 1. Base64-Encoded Header and Payload

```sh
jawir generate --header <b64encoded_header_value> --payload <b64encoded_payload_value> --secret <jwt_secret>
```

#### 2. JSON Input

```sh
jawir generate --json --header '<jwt_header_value_in_json>' --payload '<jwt_payload_value_in_json>' --secret <jwt_secret>
```

**Examples:**

_Base64-Encoded:_

```sh
jawir generate --header eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9 --payload eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOmZhbHNlLCJpYXQiOjE1MTYyMzkwMjJ9 --secret johnston2
```

_JSON Format:_

```sh
jawir generate --header '{"typ":"JWT","alg":"HS256"}' --payload '{"admin":true,"iat":1516239022,"name":"John Doe","sub":"1234567890"}' --secret johnston2
```

## Demo
![jawir demo](./demo.gif) 
