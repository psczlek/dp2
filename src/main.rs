use colored::Colorize;
use resolve_path::PathResolveExt;
use std::cmp;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::os::unix::fs::FileExt;
use std::os::unix::fs::MetadataExt;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().collect();
    let biname = args.remove(0);
    if args.is_empty() {
        println!(
            "{} {} [file(s)]...",
            "usage:".green().bold(),
            biname.cyan().bold(),
        );
        return ExitCode::SUCCESS;
    }

    let mut dumpq = VecDeque::new();
    while !args.is_empty() {
        let arg = args.remove(0);
        match fs::metadata(arg.resolve()) {
            Ok(m) => {
                if m.is_dir() {
                    println!("{}: '{}' - is a directory", "note".yellow(), arg);
                    continue;
                }
                dumpq.push_back(arg);
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => {
                    println!("{}: '{}' - no such file", "note".yellow(), arg);
                    continue;
                }
                ErrorKind::PermissionDenied => {
                    println!("{}: '{}' - permission denied", "note".yellow(), arg);
                    continue;
                }
                _ => panic!(),
            },
        }
    }

    if dumpq.is_empty() {
        eprintln!(
            "{}\n{}: supplied files have been discarded due to the preceding conditions - nothing to dump",
            "^^^^^".red(),
            "error".red(),
        );
        return ExitCode::FAILURE;
    }

    while !dumpq.is_empty() {
        let path = dumpq.pop_front().unwrap();

        println!(
            "{delim:->width$} {path}\n",
            delim = '-',
            width = 79 - path.len(),
        );
        hexdump(&path).unwrap();
    }

    ExitCode::SUCCESS
}

fn hexdump(path: &str) -> io::Result<()> {
    let fh = File::open(path.resolve())?;
    let mut fsize = fh.metadata()?.size();
    let mut foff = 0;
    let mut boff = 0;

    while fsize > 0 {
        let bufsize = cmp::min(fsize, 1536) as usize;
        let mut buf = vec![0u8; bufsize];
        let bytes_read = fh.read_at(&mut buf, foff)?;
        let mut carry = 0;

        for chunk in buf.chunks(16) {
            let mut printables = String::new();
            let mut bytes_printed = 0;

            print!("{boff:012x}: ");

            if chunk.iter().all(|byte| *byte == 0x00) {
                println!("00...");
                boff += 16;
                continue;
            }

            for byte in chunk {
                if *byte >= 0x21 && *byte <= 0x7e {
                    printables.push(*byte as char);
                } else {
                    printables.push('.');
                }

                if bytes_printed == 8 {
                    print!(" ");
                }

                print!("{byte:02x} ");
                bytes_printed += 1;
                carry = 16 - bytes_printed;
            }

            if carry > 0 {
                carry = carry * 3;
                if carry % 8 == 0 {
                    carry += 1;
                }

                print!("{:>width$}", "", width = carry);
            }

            println!(" {printables}");
            boff += 16;
        }

        fsize -= bytes_read as u64;
        foff += bytes_read as u64;
    }

    println!(
        "\n{}: {} bytes, {:.4} KiB {:.4} MiB\n",
        path,
        foff,
        (foff as f64 / 1024.0),
        (foff as f64 / 1024.0 / 1024.0)
    );

    Ok(())
}
