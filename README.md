# emojify

emojify is a cli tool that converts text to emojified text

[![Build status](https://github.com/attilarepka/emojify/actions/workflows/tests.yml/badge.svg)](https://github.com/attilarepka/emojify/actions)

## Features

- converts input text to emojified text
- adjustable space width
- adjustable character spacing
- optional background character

## Installation

**[Archives of precompiled binaries for emojify are available for 
macOS and Linux.](https://github.com/attilarepka/emojify/releases)**

Linux binaries are static executables.

If you're a **Debian** user (or a user of a Debian derivative like **Ubuntu**),
then emojify can be installed using a binary `.deb` file provided in each
[emojify release](https://github.com/attilarepka/emojify/releases).

```
$ curl -LO https://github.com/attilarepka/emojify/releases/download/0.1.3/emojify_0.1.3_amd64.deb
$ sudo dpkg -i emojify_0.1.3_amd64.deb
```

### Building

emojify is written in Rust, so you'll need [Rust installation](https://www.rust-lang.org/) in order to compile it.
emojify compiles with Rust 1.70.0 (stable) or newer. In general, it tracks
the latest stable release of the Rust compiler.

```shell
$ git clone https://github.com/attilarepka/emojify.git
$ cd emojify
$ cargo build --release
```

## Usage

emojify provides a command-line interface with the following options:

```shell
Usage: emojify [OPTIONS] --space-width <SPACE_WIDTH>

Options:
  -i, --input <INPUT>
          Text to convert into an emoji representation
  -s, --space-width <SPACE_WIDTH>
          Width of spaces between emoji characters
  -f, --foreground-emoji <FOREGROUND_EMOJI>
          Emoji used for the foreground (text)
  -b, --background-emoji <BACKGROUND_EMOJI>
          Emoji used for the background (empty spaces)
  -c, --character-spacing <CHARACTER_SPACING>
          Extra spacing between characters (in units of space_width)
  -p, --print-output
          Print the generated emoji output to the console
  -h, --help
          Print help
  -V, --version
          Print version
```

## Contributing

Contributions are welcome! Open a GitHub issue or pull request.

## License

This project is licensed under the [MIT license](LICENSE)
