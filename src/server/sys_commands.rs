use std::{process::Command, net::TcpStream, fs::{OpenOptions, File}, 
os::windows::prelude::FileExt, io::{Write, Read}};

pub const COMMAND_SEPARATOR: &str = "&&";
pub const BUF_LENGTH: usize = 1400;
pub struct RecvdCommand  {
    pub command: String,
    pub argument: String,
    pub output: Option<String>,
}

impl RecvdCommand {
    pub fn new(command:  String, argument: String, output: Option<String>) -> Self {
        return  Self{
            command,
            argument, 
            output
        }
    }
} 

///Returns dir content as vector of filenames inside it.
pub fn dir(path: String) -> Option<Vec<String>>{
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
///Performs specified by arg actions to current working directory in cmd's cd command manner.
/// 
/// Args: 
/// 
/// ".." => return to previous dir, cannot exceed past root dir . 
/// 
/// "/." => goes back to specified roor dir . 
/// 
///   _  => tries moving to specified dir . 
/// 
/// "0 " => invalid arg . 
/// 
/// Returns some(path) to the folder, none if errors occured;
pub fn cd(mut stream: &TcpStream, root_dir: &String, cwd: String, arg: Option<String>) -> Option<String>{
    let mut command = String::new();
    match arg {
        None =>  {
            stream.write(cwd.as_bytes());
            return None;
        },
        Some(ref n) => match n.as_str() {
            ".." => {
                let mut pos: usize = 0;
                let mut last_pos: usize = 0;
                for c in cwd.chars(){
                    if c == '/' {
                        last_pos = pos;
                    }
                    pos+=1;
                }
                let mut new = cwd[..last_pos].to_string();
                if new.len() < root_dir.len() { return Some( root_dir.clone() ); }
                return Some(new);
            }
            "/." => {
                return Some(root_dir.clone());
            }
            "0" => {
                return None;
            }
            _ => {
                command.push_str("cd ");
                command.push_str(cwd.as_str());
                command.push('\\');
                command.push_str(arg.unwrap().clone().as_str());
                command.push_str(COMMAND_SEPARATOR);
                command.push_str(" cd");
                let output = Command::new("cmd")
                .args(&["/C", &command])
                .output()
                .expect("Failed to execute command");    
                if output.status.success() {
                    let mut result = String::from_utf8_lossy(&output.stdout).to_string();
                    result.pop();
                    result.pop();
                    return Some( result );
                }
                else { return None; }
            }
        }
    }
}
///General purpose fn to send a file at path to stream.
/// 
///Opens the file in standard read-only mode.
/// 
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
    #[cfg(feature = "client")]
    println!("Filesize: {}", filesize);

    loop {
        if filesize_left < BUF_LENGTH as u64 {
            if filesize_left == 0 { return; } 
            file_toSend.seek_read(&mut buffer[.. filesize_left as usize ], filesize-filesize_left).unwrap_or_default();
            stream.write(&buffer[.. filesize_left as usize ]).unwrap_or_default();
            println!("Bytes left: {}", filesize_left);
            println!("Data transfered in 100%");
            return; 
        } else {
            file_toSend.seek_read(&mut buffer[..], filesize-filesize_left).unwrap_or_default();
            filesize_left-=BUF_LENGTH as u64;
            stream.write(&buffer).unwrap_or_default();
            println!("Bytes left: {}", filesize_left);
            println!("Data transfered in {:.2}%", ( (filesize - filesize_left)*100/filesize ) as f64 ); 
        }
    }
}
/// General purpose funtion to receive data from stream and save it in a file.
/// 
/// File descriptor so write options get be set.
/// 
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
/// Captures a packet with client's cmd command string. Split it into command and arguments.
/// 
/// command, arg, None if succesful
/// 
/// command, "0", None if there was no argument
/// 
/// "0", "0", None for any errors of parsing
pub fn capture_command(mut stream: &TcpStream) -> RecvdCommand {
    let mut buffer: [u8; 1400]  = [0; 1400];
    let mut parsed: String = String::new();
    match stream.read(&mut buffer) {
        Ok(n) => {  
            parsed = match std::str::from_utf8(&buffer[..n]) {
                Ok(string) => string.to_string(),
                //Error at parsing
                Err(_) => return RecvdCommand::new("0".to_string(), "0".to_string(), None),
            };
        }
        Err(e) => {
            eprintln!("Failed to read from socket: {}", e);
            //Error at reading
            return RecvdCommand::new("0".to_string(), "0".to_string(), None);
        }
    };
    //cutting EOF sign
    parsed.pop();
    parsed.pop();
    
    // Assuming received command is correct
    for (i, c) in parsed.chars().enumerate() {
        if c == ' ' {
            return RecvdCommand::new((&parsed[..i]).to_string(), (&parsed[i+1..]).to_string(), None  );
        }
    }
    // Else return command/_ with 0 argument. Possible breaches of security.
    return RecvdCommand::new(parsed, "0".to_string(), None); 
}