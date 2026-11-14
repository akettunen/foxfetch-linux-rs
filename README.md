# foxfetch-linux-rs

A fast system information tool for Linux, written in Rust.

## Features

- [Fast, minimal output]
- [Distro / kernel / uptime / shell / DE / CPU / GPU / memory info]
- [Custom ASCII art or config support, if any]

## Installation

### From source

```sh
git clone https://github.com/akettunen/foxfetch-linux-rs.git
cd foxfetch-linux-rs
cargo build --release
sudo install -Dm755 target/release/foxfetch /usr/local/bin/foxfetch
```

> Requires a recent stable Rust toolchain ([rustup.rs](https://rustup.rs)).

## Usage

```sh
foxfetch
```

[List flags/options here, if any.]

## Configuration

[Describe the config file location and format, or remove this section.]

## Contributing

Issues and pull requests are welcome.

## License

[License name], see [LICENSE](LICENSE).

## Extra
+ Claude made the system fetch fast & wrote this README cuz im lazy :D
