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
pub mod sys_commands;
pub mod user;
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
    root_dir: String,
    credentials: String,
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
            cwd: RefCell::new(String::new()),
            root_dir: String::from("C:/"),
            credentials: String::new(),

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
    pub fn set_credentials(&mut self, path: String) { self.credentials = path; }

    pub fn set_root_dir(&mut self, path: String) { self.root_dir = path; }

    pub fn set_cwd(&mut self, path: String) {*self.cwd.borrow_mut() = path}
    
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
        loop {
            match self.auth_usr() {
                Ok(_) => {
                    self.handle_command( sys_commands::capture_command(self.user.as_ref().unwrap() ) ); 
                }
                Err(_) => { break; }
            }
        }
    }
    
    pub fn handle_command(&self, command: sys_commands::RecvdCommand) {
        use crate::server::sys_commands::send_file;
        let mut path = self.cwd.borrow().clone();
        match command.command.as_str() {
            "cd" => {
                match sys_commands::cd(self.user.as_ref().unwrap(),&self.root_dir, self.cwd.borrow().clone() , Some(command.argument)) {
                    Some(n) => { *self.cwd.borrow_mut() = n },
                    None => (),
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
                sys_commands::receive_file(
                self.user.as_ref().unwrap(), 
                OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(path)
                        .unwrap()
                );
            }
            _ => (),
        }
    }

    fn send_cmd_result(mut stream: &TcpStream, mut result: String) {
        if result.is_empty() { result.push_str("Done"); }
        stream.write(&result.as_bytes()).unwrap();
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
    pub fn auth_usr(&self) -> Result<(), ()> {
        // let mut buffer = [0;1400];
        // let mut credentials = String::new();
        // //Read login from stream
        // match stream.read(&mut buffer) {
        //     Ok(0) => return Err(()),
        //     Ok(n) => {
        //         match std::str::from_utf8(&buffer[..n]) {
        //             Ok(login) => credentials.push_str(login) ,
        //             //Error at parsing
        //             Err(_) => return Err(()),
        //         };
        //     },
        //     Err(e) => { 
        //         eprintln!("Failed to read from socket: {}", e);
        //         return Err(());
        //     }
        // };
        // //Read password from stream
        // match stream.read(&mut buffer) {
        //     Ok(0) => return Err(()),
        //     Ok(n) => {
        //         match std::str::from_utf8(&buffer[..n]) {
        //             Ok(password) => credentials.push_str(password),
        //             //Error at parsing
        //             Err(_) => return Err(()),
        //         };
        //     },
        //     Err(e) => {
        //         eprintln!("Failed to read from socket: {}", e);
        //         return Err(());
        //     }
        // };
        // // Search for given entry
        // let entries = File::open(path).unwrap();
        // let file_buf = BufReader::new(entries);
        // for line in file_buf.lines() {
        //     if line.unwrap() == credentials {
        //         return Ok(());
        //     }
        // }
        // return Err(());
        let mut buffer = "1".as_bytes();
        self.user.as_ref().unwrap().write(&buffer);
        return Ok(());
    }
}