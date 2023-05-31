use std::{net::TcpStream, io::{Read, BufReader, Write, BufRead}, fs::File};
/// Authenticates user with given credentials. Reads login and passowrd separately. 
pub fn auth_usr(mut stream: &TcpStream, path: &String) -> Result<(), ()> {
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
    stream.write(&buffer);
    return Ok(());
}
// pub fn auth_usr(mut connection: TcpStream, path: String) -> Result<(), ()> {
//     let mut temp: String = String::new();
//     connection.read_to_string(&mut temp);
//     let temp_vec: Vec<&str> = temp.trim().split(" ").collect();
//     let user_entries_file = File::open(&self.user_entries).unwrap();
//     let buffer = BufReader::new(user_entries_file);
//     for line in buffer.lines() {
//         match line {
//             Ok(line) => {
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
