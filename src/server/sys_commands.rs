use std::{process::Command, net::TcpStream, fs::{OpenOptions, File}, os::windows::prelude::FileExt, io::{Write, Read}};
const COMMAND_SEPARATOR: &str = "&&";
const BUF_LENGTH: usize = 1400;
///Returns dir content as vector of filenames inside it.
fn dir(path: String) -> Option<Vec<String>>{
    let mut dir_vec: Vec<String> = Vec::new();
    let mut command: String = String::from("cd ");
    command.push_str(&path);
    command.push_str(COMMAND_SEPARATOR);
    command.push_str("dir /b");
    let output = Command::new("cmd")
    .args(&["/C", &command])
    .output()
    .expect("Failed to execute command");

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        let mut start: usize = 0;
        let mut end: usize = 0;
        for &i in result.as_bytes(){
            println!("{}", i);
            if i == 13 {
                dir_vec.push((&result[start..end]).to_string());
                start = end+2;
            }
            end+=1;
        }
        return Some(dir_vec);
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}", error);
        return None
    }
}
///General purpose fn to send a file at path to stream.
///Opens the file in standard read-only mode.
///Absolute path must be provided
pub fn send_file(mut stream: &TcpStream, path: String){
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
            stream.write(&buffer[.. filesize_left as usize ]).unwrap_or_default();
            println!("Bytes left: {}", filesize_left);
            println!("Data transfered in 100%");
            stream.shutdown(std::net::Shutdown::Both).unwrap_or_default();
            return; }

        else {
            file_toSend.seek_read(&mut buffer[..], filesize-filesize_left).unwrap_or_default();
            filesize_left-=BUF_LENGTH as u64;
            stream.write(&buffer).unwrap_or_default(); 
            println!("Bytes left: {}", filesize_left);
            println!("Data transfered in {:.2}%", ( (filesize - filesize_left)*100/filesize ) as f64 ); }
    }
}
/// General purpose funtion to receive data from stream and save it in a file.
/// File descriptor so write options get be set.
/// Undefined behaviour unless write/write_new is set to true;
pub fn receive_file(mut stream: &TcpStream, mut file: File) {
    let mut buffer  = [0; 1400];
    loop {
        let bytes_read = match stream.read(&mut buffer)
        {
            Ok(0) => return,
            Ok(n) => file.write(&buffer[..n]),
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                return;
            }
        };
    }
}