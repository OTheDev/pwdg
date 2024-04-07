[![](https://github.com/OTheDev/pwdg/actions/workflows/ci.yml/badge.svg)](https://github.com/OTheDev/pwdg/actions/workflows/ci.yml)
[![](https://github.com/OTheDev/pwdg/actions/workflows/static_analysis.yml/badge.svg)](https://github.com/OTheDev/pwdg/actions/workflows/static_analysis.yml)

# pwdg

`pwdg` is a rudimentary command-line tool and Rust library for generating
secure, random passwords.

# Installation

Install `pwdg` using `cargo`:

```shell
cargo install pwdg
```

# Usage

## Command Line Interface

Generate a password with default settings:

```shell
pwdg
```

Generate a password with at least 1 uppercase letter, 1 lowercase letter, 1
digit, and 1 special character:

```shell
pwdg -s
```

Generate a 12-character password with at least 2 uppercase letters, 2 lowercase
letters, 2 digits, and 2 special characters:

```shell
pwdg --length 12 --min-upper 2 --min-lower 2 --min-digit 2 --min-special 2
```

Generate a password with default settings, but excluding the characters `A`,
`B`, `C`, `D`, and `E` from the overall character set used for password
generation:

```shell
pwdg --exclude=ABCDE
```

### Command Line Options

```console
$ pwdg --help
A rudimentary command-line tool and Rust library for generating secure, random passwords.

Usage: pwdg [OPTIONS]

Options:
  -l, --length <LENGTH>            Sets the length of the password. Must be at least 8 [default: 8]
      --min-upper <MIN_UPPER>      Minimum number of uppercase characters (A to Z) [default: 0]
      --min-lower <MIN_LOWER>      Minimum number of lowercase characters (a to z) [default: 0]
      --min-digit <MIN_DIGIT>      Minimum number of digit characters (0 to 9) [default: 0]
      --min-special <MIN_SPECIAL>  Minimum number of special characters.
                                   Special characters: !@#$%^&*()_+-={}[]|:;"'<>,.?/~\` [default: 0]
  -e, --exclude <EXCLUDE>          Characters to exclude from the overall character set used for password generation
  -s, --strong                     Generates a password with at least 1 uppercase letter, 1 lowercase letter, 1 digit, and 1 special character. This option overrides --min-upper, --min-lower, --min-digit, and --min-special if they are also set
  -h, --help                       Print help
  -V, --version                    Print version
```

## Characters

Passwords may be comprised of **uppercase** (`A` to `Z`), **lowercase** (`a` to
`z`), **digit** (`0` to `9`), or **special** characters.

The set of special characters:
```plaintext
! @ # $ % ^ & * ( ) _ + - = { } [ ] | : ; " ' < > , . ? / ~ \ `
```

# License

`pwdg` is licensed under the [Apache License, Version 2.0](
    https://github.com/OTheDev/pwdg/blob/main/LICENSE).

# Source

Clone the [repository](https://github.com/OTheDev/pwdg):

```shell
git clone git@github.com:OTheDev/pwdg.git
cd pwdg
```

## Test

```shell
cargo test  # Optionally, --release
```

## Install

```shell
cargo install --path .
```
