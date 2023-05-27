use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::env::temp_dir;
use std::fmt::Error;
#[allow(unused_imports)]
use std::io::{BufRead, BufReader, Read, Write};
#[allow(unused_imports)]
use std::net::{SocketAddrV4, TcpListener, SocketAddr, ToSocketAddrs, Ipv4Addr, TcpStream};
#[allow(unused_imports)]
use std::env;
use std::ops::{Deref, DerefMut};
#[allow(unused_imports)]
use std::thread::JoinHandle;
#[allow(unused_imports)]
use std::fs::{self, read, File};
use std::vec;
use std::process::{Command, Output};
use std::fs::{OpenOptions, };
use std::io::{Seek, SeekFrom,};

use std::io;

use std::os::windows::prelude::FileExt;
// use self::logged_user::{LoggedUser, Privileges};

use std::fs::{remove_file};
const BUF_LENGTH: usize = 1400;

// mod logged_user;
pub struct FtpServer {
    // addr: SocketAddrV4,
    // ip: String,
    // socket: String,
    handler: TcpListener,
    read_only: bool,
    // max_users: u16,
    // current_users: u16,    
    // max_timeout: u32,
    // use_timeout: bool,
    // users: Vec<logged_user::LoggedUser>,
    user: Option<TcpStream>,
    cwd: RefCell<String>,
    // user_entries: String,
    root_dir: String,
    }
// pub enum UserTruncate {
//     AllowExcessive,
//     TruncateExcessive
// }

// enum AllowExcessive { AllowExcessive, TruncateExcessive(TruncateMethod), TruncateWithDelay() }
// // enum TruncateMethod { TruncateNewest, TruncateOldest}
// enum Action {
//     Download,
//     Upload,
//     Dir,
//     Mkdir,
//     Rmdir,
//     Touch, 
//     Rm,
//     None
// }
// struct ServerCommand {
//     args: String, 
//     command_type: Action
// }

pub struct RecvdCommand  {
    command: String,
    argument: String,
    output: Option<String>,
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
impl FtpServer {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Self {

        // let args: Vec<String> = env::args().collect();

        // self.socket = args.pop().unwrap();
        // self.ip = args.pop().unwrap();
        // self.handler = TcpListener::bind(&args[1]).unwrap();
        // self.read_only = true;
        // self.current_users = 0;
        // self.max_users = 10;
        // self.max_user_timeout = 6000;
        Self {
            // handler: TcpListener::bind(&args[1]).unwrap(),
            handler: TcpListener::bind(addr).unwrap(),
            read_only: true,
            // max_users: 10,
            // current_users: 0,
            // max_timeout: 6000,
            // use_timeout: false,
            // users: Vec::new(),
            user: None,
            cwd: RefCell::new(String::from("C:/Users/Ryzen/Desktop/Test")),
            // user_entries: String::from("C://"),
            root_dir: String::from("C:/"),
        }
    }
    // fn set_read_only(&mut self, is_read_only: bool) { self.read_only = is_read_only; }
    // fn set_max_user(&mut self, max_user: u16, option: UserTruncate) {
    //     self.max_users = max_user;
    //     match option {
    //         UserTruncate::AllowExcessive => (),
    //         UserTruncate::TruncateExcessive => (),
    //     }
    // }
    // fn set_max_timeout(&mut self, max_timeout: u32, option: UserTruncate) {
    //     self.max_timeout = max_timeout;
    //     match option {
    //         UserTruncate::AllowExcessive => (),
    //         UserTruncate::TruncateExcessive => (),
    //     }
    // }
    // fn set_credentials(&mut self, path: String) { self.user_entries = path; }

    // fn set_root_dir(&mut self, path: String) { self.root_dir = path; }

    ///Starts server's listening
    pub fn start_listening(&mut self) {
        for connection in self.handler.incoming() {
            match connection {
                Ok(connection) => {
                    self.user = Some(connection);
                    eprintln!("Polaczono");
                    self.handle_client();
                }
                Err(_) => eprintln!("Unnsuccessful connetion captured"), 
            } 
        } 
    }
/// Performs user defined actions to the connected TCPStream
    pub fn handle_client(&self) {
        use crate::server::capture_command;
        loop {
            self.handle_command( capture_command(self.user.as_ref().unwrap() ) )
        }
    }
    pub fn handle_command(&self, command: RecvdCommand) {
        use crate::server::send_file;
        let mut path = self.cwd.borrow().clone();
        match command.command.as_str() {
            "cd" => {
                match command.argument.as_str() {
                    ".." => {
                        let mut pos: usize = 0;
                        for (i, c) in command.argument.chars().enumerate() {  
                            if c == '/' {
                                pos = i;
                            }
                        }
                        (*self.cwd.borrow_mut()).truncate(pos);
                    }
                    "/." => {
                        *self.cwd.borrow_mut() = self.root_dir.clone();
                    }
                    _  =>{
                        if command.argument == 0 {
                        
                        }
                    }
                }
                
            }
            "dir" => {
                
            }
            "mkdir" => {
    
            }
            "rmdir" => {
    
            }
            "echo." => {
    
            }
            "del" => {
    
            }
            "download" => {
                path.push_str("/");
                path.push_str(&command.argument);
                send_file(self.user.as_ref().unwrap(), path);
            }
            "upload" => {
                let mut file_desc = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)
                .unwrap();
                receive_file(self.user.as_ref().unwrap(), file_desc);
            }
            _ => {

                //     OpenOptions::new()
//     .write(true)
//     .create_new(true)
//     .open(&filepath)
//     .unwrap();
// // .open(DIR.to_owned()+"//test.txt")
// // .open(filepath)
// // .unwrap();
    
            }
        }
    }

    fn send_cmd_result(mut stream: &TcpStream, mut result: String) {
        if result.is_empty() { result.push_str("Done"); }
        stream.write(&result.as_bytes()).unwrap();
    }
} 
    // Creates a "name" dir with a cmd command at user's cwd
//     pub fn mkdir(&self, dirname: String) {
//         let mut command = String::from("cd ");
//         command.push_str(&self.cwd);
//         command.push_str(" && mkdir ");
//         command.push_str(&dirname);
//         let output = Command::new("cmd")
//         .args(&["/C", &command])
//         .output()
//         .expect("Failed to execute command");

//         // if output.status.success() {
//         //     let result = String::from_utf8_lossy(&output.stdout);
//         //     println!("Command output: {}", result);
//         // } else {
//         //     let error = String::from_utf8_lossy(&output.stderr);
//         //     eprintln!("Command failed: {}", error);
//     }
//     ///Removes a "name" dir with a cmd command at user's cwd
//     pub fn rmdir(&self, dirname: String) {
//         let mut command = String::from("cd ");
//         command.push_str(&self.cwd);
//         command.push_str(" && rmdir /s /q ");
//         command.push_str(&dirname);
//         let output = Command::new("cmd")
//         .args(&["/C", &command])
//         .output()
//         .expect("Failed to execute command");
//     }
//     /// Creates a "filename" file at cwd. Returns it's descriptor.
//     pub fn touch(&self, filename: String) -> File {
//         let mut filepath=String::from(&self.cwd);
//         filepath.push_str("/");
//         filepath.push_str(&filename);
//     return 
//     OpenOptions::new()
//     .write(true)
//     .create_new(true)
//     .open(&filepath)
//     .unwrap();
// // .open(DIR.to_owned()+"//test.txt")
// // .open(filepath)
// // .unwrap();
//     }
//     /// Removes a "filename" file. 
//     pub fn rm(&self, filename: String) {
//         let mut command = String::from("cd ");
//         command.push_str(&self.cwd);
//         command.push_str(" && del ");
//         command.push_str(&filename);
//         let output = Command::new("cmd")
//         .args(&["/C", &command])
//         .output()
//         .expect("Failed to execute command");
//     }

// pub fn start_listening(&mut self) {
    //     for connection in self.handler.incoming() {
    //         match connection {
    //             Ok(connection) => {
    //                 if self.current_users + 1 > self.max_users { connection.shutdown(std::net::Shutdown::Both); }
    //                 else { 
    //                     match self.authenticate_user(connection) {
    //                         Ok(user) => self.users.push(user),
    //                         Err(_) => eprintln!("Unnsuccessful login captured"), 
    //                     } 
    //                 }
    //             }
    //             Err(_) => eprintln!("Unnsuccessful connetion captured"), 
    //         } 
    //     } 
    // }

//     fn authenticate_user(&self, mut connection: TcpStream) -> Result<LoggedUser, ()>
//      {
//         let mut temp: String = String::new();
//         connection.read_to_string(&mut temp);
//         let temp_vec: Vec<&str> = temp.trim().split(" ").collect();

//         let user_entries_file = File::open(&self.user_entries).unwrap();
//         let buffer = BufReader::new(user_entries_file);
//         for line in buffer.lines() {
//             match line {
//                 Ok(line) => {
// //1 -> login, 2 -> password, 3 -> usertype, 4 -> login, 5 -> login, 6 -> login, 7 -> login, 8 -> login, 
//                     let user_entry: Vec<&str>= line.trim().split(" ").collect();
//                     let mut user_type: logged_user::UserType;
//                     if temp_vec[0] == user_entry[0] && temp_vec[1] == user_entry[1] {
//                         let mut user_type: logged_user::UserType = {
//                             match user_entry[2] {
//                                 "0" => logged_user::UserType::Admin,
//                                 "1" => logged_user::UserType::Moderator,
//                                 "2" => logged_user::UserType::Default,
//                                  _  => logged_user::UserType::Default,
//                             }};  
//                         return Ok( LoggedUser::new( 
//                                         user_type, 
//                                         temp_vec[0].to_owned(), 
//                                         temp_vec[1].to_owned(), 
//                                         connection,
//                                         1000 , 
//                                         Privileges::new(
//                                             true, 
//                                             true, 
//                                             true, 
//                                             true
//                         ))); }}
//                     Err(_) => (),
//                     }}
//         return Err(());

// }
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

/// Captures a packet with client's cmd command string.
pub fn capture_command(mut stream: &TcpStream) -> RecvdCommand{
    let mut buffer: [u8; 1400]  = [0; 1400];
    let mut parsed: String = String::new();
    match stream.read(&mut buffer) {
        Ok(n) => {  
            parsed = match std::str::from_utf8(&buffer[..n]) {
                Ok(string) => string.to_string(),
                Err(_) => return RecvdCommand::new("0".to_string(), "0".to_string(), None),
            };
        }
        Err(e) => {
            eprintln!("Failed to read from socket: {}", e);
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

    // if &parsed[..2] == "cd" {
    //     let mut temp = String::new();
    //     temp.push_str(&parsed);
    //     temp.push_str(" && cd");
    //     let output= Command::new("cmd")
    //     .args(&["/C", &temp])
    //     .output()
    //     .expect("Failed to execute command");
    //     *self.cwd.borrow_mut() = String::from_utf8_lossy(&output.stdout).to_string();
    //     return self.cwd.borrow().clone();

    // }
    // let mut command = String::from("cd ");
    // command.push_str(self.cwd.borrow().as_str());
    // command.push_str(" && ");
    // command.push_str(&parsed);
    // let output = Command::new("cmd")
    // .args(&["/C", &command])
    // .output()
    // .expect("Failed to execute command");
    // return String::from_utf8_lossy(&output.stdout).to_string();
}
