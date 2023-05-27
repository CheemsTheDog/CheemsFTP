use std::net::ToSocketAddrs;
use std::net::TcpStream;
use std::io;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write, Seek, SeekFrom, BufRead};
use std::os::windows::prelude::FileExt;
const BUF_LENGTH: usize = 1400;
pub struct User {
    username: String,
    password: String,
    handle: TcpStream
}

impl User {
    pub fn new<A: ToSocketAddrs>(username: String, password: String, addr: A) -> Self{
        Self { 
            username, 
            password,
            handle: match TcpStream::connect(addr) {
                Ok(handle) => handle,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            }
        }
    }
    pub fn send_command(&mut self) {
        loop {
            let mut buffer  = [0; 1400];
            let mut to_send = String::new();
            io::stdin()
            .read_line(&mut to_send)
            .expect("Failed to read line");
            self.handle.write(&to_send.as_bytes()); 
            //----------------------
            // let mut parsed: String = String::new();
            // match self.handle.read(&mut buffer) {
            //     Ok(n) => {  
            //         parsed = match std::str::from_utf8(&buffer[..n]) {
            //             Ok(string) => string.to_string(),
            //             Err(_) => panic!("Invalid UTF-8 sequence"),
            //         };
            //     }
            //     Err(e) => {
            //         eprintln!("Failed to read from socket: {}", e);
            //     }
            // };
            // println!("{}", &parsed)
        }

    }
    pub fn send(&mut self) {
        let path = String::from("C://Users//Ryzen//Desktop//Projekty//TCP_Klient//DonEskobar.txt");
        let mut file_toSend = OpenOptions::new()
            .read(true)
            .open(path)
            .unwrap();
        let mut buffer:[u8; BUF_LENGTH] = [0;BUF_LENGTH];
        let mut start: u64 = 0;
        let mut end: u64 = 0; 
        let filesize = file_toSend.metadata().unwrap().len();
        let mut filesize_left = file_toSend.metadata().unwrap().len();
        println!("Filesize: {}", filesize);

        loop {
            if filesize_left < BUF_LENGTH as u64 {
                if filesize_left == 0 { return; } 
                // modulo = (BUF_LENGTH-(BUF_LENGTH-filesize_left as usize));
                file_toSend.seek_read(&mut buffer[.. filesize_left as usize ], filesize-filesize_left).unwrap_or_default();
                self.handle.write(&buffer[.. filesize_left as usize ]).unwrap_or_default();
                println!("Bytes left: {}", filesize_left);
                println!("Data transfered in 100%");
                self.handle.shutdown(std::net::Shutdown::Both).unwrap_or_default();
                return; }

            else {
                file_toSend.seek_read(&mut buffer[..], filesize-filesize_left).unwrap_or_default();
                filesize_left-=BUF_LENGTH as u64;
                self.handle.write(&buffer).unwrap_or_default(); 
                println!("Bytes left: {}", filesize_left);
                println!("Data transfered in {:.2}%", ( (filesize - filesize_left)*100/filesize ) as f64 ); }
        }
    }
}
