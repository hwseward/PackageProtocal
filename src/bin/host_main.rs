use package::package::*;
use std::io::prelude::*;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    
    let pack = Package::new_msg(String::from("Test"));
    let listen = TcpListener::bind("127.0.0.1:80")?;
    loop { 
        for stream in listen.incoming() {
        
            println!("Test");
            println!("{:?}", pack.clone().to_vec());
            stream.unwrap().write(&pack.clone().to_vc())?;
            
        }
    }
}
