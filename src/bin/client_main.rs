use package::package::*;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let mut stream = TcpStream::connect("127.0.0.1:80")?;
    let mut buf = [0; 512];
    let t = stream.read(&mut buf)?;
    let pack = Package::new(&buf);
    println!("{}", pack.to_string());
    Ok(())
}
