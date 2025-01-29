use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

fn exit_with_message(code: i32, msg: &str) -> ! {
    eprintln!("{}", msg);
    exit(code);
}

fn exit_on_invalid_arguments(args: &[String]) {
    if args.len() != 2 {
        exit_with_message(1, &format!("Usage: {} [filename]", args[0]));
    }
}

fn try_open_file(filename: &str) -> File {
    match File::open(&filename) {
        Ok(f) => f,
        Err(e) => exit_with_message(1, &format!("Failed to open {}: {}", filename, e)),
    }
}

fn read_into_buffer(reader: &mut BufReader<File>, buff: &mut [u8]) -> usize {
    match reader.read(buff) {
        Ok(n) => n,
        Err(e) => exit_with_message(1, &format!("Error reading from file: {}", e))
    }
}

fn print_buff(buff: &[u8]) {
    print!("{}", String::from_utf8_lossy(buff));
}

fn write_file_to_stdout(filename: &str) {
    let file = try_open_file(filename);
    let mut reader = BufReader::new(file);
    let mut buff = [0u8; 4096];

    loop {
        let bytes_read = read_into_buffer(&mut reader, &mut buff);
        if bytes_read == 0 {
            break
        }
        print_buff(&buff[..bytes_read]);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    exit_on_invalid_arguments(&args);
    write_file_to_stdout(args.get(1).unwrap());
}
