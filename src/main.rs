use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let res = run();
    match res {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return hexdump_stdin();
    }

    for file in args.iter().skip(1) {
        match fs::metadata(file) {
            Ok(metadata) if !metadata.is_file() => eprintln!("{file}: not a regular file"),
            Ok(_) => hexdump_file(file)?,
            Err(err) => eprintln!("{file}: {err}"),
        }
    }

    Ok(())
}

fn hexdump_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let file = File::open(&path)?;
    let metadata = file.metadata()?;
    let mut remaining = metadata.len();
    let mut offset = 0;
    let mut buf = vec![0u8; 16 * 1024];

    let display_path = path.as_ref().display();

    println!("{:-<width$} {display_path}", "", width = 77);

    while remaining > 0 {
        let n = file.read_at(&mut buf, offset)?;
        if n == 0 {
            break;
        }

        hexdump(&buf[..n], offset)?;
        offset += n as u64;
        remaining -= n as u64;
    }

    println!(
        "\n{}: {} bytes, {:.4} KiB {:.4} MiB",
        display_path,
        offset,
        offset as f64 / 1024.0,
        offset as f64 / 1024.0 / 1024.0
    );

    Ok(())
}

fn hexdump_stdin() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buf = vec![0u8; 16 * 1024];
    let mut offset = 0;

    loop {
        let n = stdin.read(&mut buf)?;
        if n == 0 {
            break;
        }

        hexdump(&buf[..n], offset)?;
        offset += n as u64;
    }

    Ok(())
}

fn hexdump(buf: &[u8], mut offset: u64) -> io::Result<()> {
    for chunk in buf.chunks(16) {
        let mut line = format!("{offset:08x}:  ");
        let mut ascii = String::new();

        for (i, byte) in chunk.iter().enumerate() {
            if i == 8 {
                line.push(' ');
            }

            line.push_str(&format!("{:02x} ", byte));

            let ch = match byte {
                0x20..=0x7e => *byte as char,
                _ => '.',
            };
            ascii.push(ch);
        }

        let padding = 16 - chunk.len();
        if padding > 0 {
            let pad_spaces = padding * 3 + if chunk.len() <= 8 { 1 } else { 0 };
            line.push_str(&" ".repeat(pad_spaces));
        }

        println!("{line} {ascii}");
        offset += 16;
    }

    Ok(())
}
