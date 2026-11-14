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

fn pci_id(card: u8, name: &str) -> String {
    let mut b = [0u8; 16];
    read(&format!("/sys/class/drm/card{card}/device/{name}"), &mut b)
        .trim()
        .trim_start_matches("0x")
        .to_lowercase()
}

fn gpu() -> String {
    let Some((vendor, device)) = (0..4u8)
        .map(|c| (pci_id(c, "vendor"), pci_id(c, "device")))
        .find(|(v, d)| !v.is_empty() && !d.is_empty())
    else {
        return "unknown".into();
    };

    let db = std::fs::read_to_string("/usr/share/hwdata/pci.ids")
        .or_else(|_| std::fs::read_to_string("/usr/share/misc/pci.ids"))
        .unwrap_or_default();

    let mut vendor_name = "";
    let mut device_name = "";
    let mut in_vendor = false;

    for l in db.lines() {
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        if let Some(rest) = l.strip_prefix('\t') {
            if in_vendor && !rest.starts_with('\t') {
                if let Some(name) = rest.strip_prefix(device.as_str()) {
                    device_name = name.trim();
                    break;
                }
            }
        } else if in_vendor {
            break;
        } else if let Some(name) = l.strip_prefix(vendor.as_str()) {
            vendor_name = name.trim();
            in_vendor = true;
        }
    }

    if device_name.is_empty() {
        return format!("{vendor}:{device}");
    }

    let model = match (device_name.find('['), device_name.rfind(']')) {
        (Some(a), Some(b)) if a < b => &device_name[a + 1..b],
        _ => device_name,
    };
    let brand = vendor_name.split_whitespace().next().unwrap_or("");
    format!("{brand} {model}").trim().to_string()
}

fn main() {
    let (mut b1, mut b2, mut b4) = ([0u8; 1024], [0u8; 128], [0u8; 4096]);

    let osr = read("/etc/os-release", &mut b1);
    let os = field(osr, "NAME=").trim_matches('"');

    let kernel = read("/proc/sys/kernel/osrelease", &mut b2).trim_end();

    let cpuinfo = read("/proc/cpuinfo", &mut b4);
    let cpu = field(cpuinfo, "model name")
        .trim_start_matches(|c| c == ' ' || c == '\t' || c == ':')
        .trim();

    let gpu = gpu();

    let info: [(&str, &str); 4] = [("os", os), ("kernel", kernel), ("cpu", cpu), ("gpu", &gpu)];

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
