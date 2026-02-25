use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let res = run();
    match res {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            // Ignore
            if e.kind() == io::ErrorKind::BrokenPipe {
                return ExitCode::SUCCESS;
            }

            eprintln!("error: {e}");
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
        hexdump_file(file)?;
    }

    Ok(())
}

fn hexdump_file(path: impl AsRef<Path>) -> io::Result<()> {
    let file = File::open(&path)?;

    let metadata = file.metadata()?;

    let mut remaining = metadata.len();

    let mut offset = 0;
    let mut buf = vec![0u8; 16 * 1024];

    let mut stdout = io::stdout();

    let display_path = path.as_ref().display();

    stdout.write_all(format!("{:-<width$} {display_path}\n", "", width = 77).as_bytes())?;

    while remaining > 0 {
        let n = file.read_at(&mut buf, offset)?;
        if n == 0 {
            break;
        }

        hexdump(&mut stdout, offset, &buf[..n])?;

        offset += n as u64;
        remaining -= n as u64;
    }

    stdout.write_all(
        format!(
            "\n{}: {} bytes, {:.4} KiB {:.4} MiB\n",
            display_path,
            offset,
            offset as f64 / 1024.0,
            offset as f64 / 1024.0 / 1024.0
        )
        .as_bytes(),
    )?;

    Ok(())
}

fn hexdump_stdin() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut buf = vec![0u8; 16 * 1024];
    let mut offset = 0;

    loop {
        let n = stdin.read(&mut buf)?;
        if n == 0 {
            break;
        }

        hexdump(&mut stdout, offset, &buf[..n])?;
        offset += n as u64;
    }

    Ok(())
}

fn hexdump(out: &mut impl Write, mut offset: u64, buf: &[u8]) -> io::Result<()> {
    for chunk in buf.chunks(16) {
        let mut line = format!("{offset:#012x}:  ");
        let mut ascii = String::new();

        for (i, byte) in chunk.iter().enumerate() {
            if i == 8 {
                line.push(' ');
            }

            line.push_str(&format!("{:02x} ", byte));

            let ch = match byte {
                // Null
                // 0x00 => todo!(),
                // Space
                // 0x20 => todo!(),
                // Punctuation
                // 0x21..=0x2f | 0x3a..=0x40 | 0x5b..=0x60 | 0x7b..=0x7e => todo!(),
                // Digits
                // 0x30..=0x39 => todo!(),
                // Letters
                // 0x41..=0x5a | 0x61..=0x7a => todo!(),
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

        out.write_all(format!("{line} {ascii}\n").as_bytes())?;
        offset += 16;
    }

    Ok(())
}
