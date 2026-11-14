// COPYRIGHT (C) akettunen
use std::{fs::File, io::{Read, Write}};

const O: &str = "\x1b[1m\x1b[38;2;255;165;0m";
const R: &str = "\x1b[0m";

const LOGO: [&str; 4] = [
    r"    |\__/|    ",
    r"   /     \    ",
    r"  /_.~ ~,_\   ",
    r"     \@/      ",
];

/// Read up to buf.len() bytes of a file; tolerate truncation mid-UTF-8.
fn read<'a>(path: &str, buf: &'a mut [u8]) -> &'a str {
    let n = File::open(path).and_then(|mut f| f.read(buf)).unwrap_or(0);
    match std::str::from_utf8(&buf[..n]) {
        Ok(s) => s,
        Err(e) => std::str::from_utf8(&buf[..e.valid_up_to()]).unwrap_or(""),
    }
}

fn field<'a>(s: &'a str, key: &str) -> &'a str {
    s.lines().find_map(|l| l.strip_prefix(key)).unwrap_or("")
}

fn main() {
    let (mut b1, mut b2, mut b3, mut b4) = ([0u8; 1024], [0u8; 128], [0u8; 128], [0u8; 4096]);

    let osr = read("/etc/os-release", &mut b1);
    let os = field(osr, "NAME=").trim_matches('"');

    let kernel = read("/proc/sys/kernel/osrelease", &mut b2).trim_end();

    let mem = read("/proc/meminfo", &mut b3);
    let ram_mb = field(mem, "MemTotal:")
        .trim()
        .trim_end_matches("kB")
        .trim()
        .parse::<u64>()
        .unwrap_or(0)
        / 1024;

    let cpuinfo = read("/proc/cpuinfo", &mut b4);
    let cpu = field(cpuinfo, "model name")
        .trim_start_matches(|c| c == ' ' || c == '\t' || c == ':')
        .trim();

    let mut ram = String::with_capacity(16);
    use std::fmt::Write as _;
    let _ = write!(ram, "{ram_mb} MB");

    let info: [(&str, &str); 4] = [("os", os), ("kernel", kernel), ("ram", &ram), ("cpu", cpu)];

    let mut out = String::with_capacity(512);
    for i in 0..LOGO.len().max(info.len()) {
        out.push_str(O);
        out.push_str(LOGO.get(i).copied().unwrap_or("              "));
        out.push_str(R);
        if let Some((k, v)) = info.get(i) {
            out.push_str(O);
            out.push_str(k);
            out.push_str(R);
            out.push_str(": ");
            out.push_str(v);
        }
        out.push('\n');
    }

    let _ = std::io::stdout().write_all(out.as_bytes());
}