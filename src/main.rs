extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process;

fn main() {
    let matches = App::new("see")
        .version("0.1.0")
        .author("rooat welling1234@gmail.com")
        .about("A drop-in cat replacement written in Rust")
        .arg(
            Arg::with_name("FILE")
                .help("File to print.")
                .empty_values(false),
        )
        .get_matches();

    if let Some(file) = matches.value_of("FILE") {
        if Path::new(&file).exists() {
            match File::open(file) {
                Ok(mut f) => {
                    let mut data = String::new();
                    f.read_to_string(&mut data)
                        .expect("[see Error] Unable to read the  file.");
                    let stdout = std::io::stdout(); // get the global stdout entity
                    let mut handle = std::io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
                    match writeln!(handle, "{}", data) {
                        Ok(_res) => {}
                        Err(err) => {
                            eprintln!("[see Error] Unable to display the file contents. {:?}", err);
                            process::exit(1);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("[see Error] Unable to read the file. {:?}", err);
                    process::exit(1);
                }
            }
        } else {
            eprintln!("[see Error] No such file or directory.");
            process::exit(1);
        }
    }
}

// use std::io::stdin;

// fn main() {
//     println!("Please input commond!");
//     loop{
//     	let mut input  = String::new();
// 	    stdin().read_line(&mut input).expect("Commond not found");
// 	    input.trim().to_string();
// 	    println!("{:?}",input);
// 	    let splitted: Vec<&str> = input.split(' ').collect();

//         /* get() returns &&str, so we mention result type &str
//            and get it from a reference (*) */
//         let command: &str = match splitted.get(0) {
//             Some(value) => *value,
//             None => { continue; }
//         };

//         const ADD_BLOCK: &str = "add_block";
//         const SEE_BLOCKCHAIN: &str = "list_blocks";
//         const ADD_PEER: &str = "add_peer";
//         const LIST_PEERS: &str = "list_peers";
//         const EXIT: &str = "exit";
//         const HELP: &str = "help";

//         let _option = match splitted.get(1) {
//             Some(_option) => _option,
//             None => {

//                 if command == ADD_BLOCK ||
//                     command == ADD_PEER {
//                     continue;
//                 }

//                 ""
//             }
//         };

//         if command == ADD_BLOCK {
//             println!("New block added.");
//         }
//         else if command == SEE_BLOCKCHAIN {
// 			println!("list_blocks.");
//         }
//         else if command == ADD_PEER {
// 			println!("add_peer.");
//         }
//         else if command == LIST_PEERS {
// 			println!("list_peers.");
//         }
//         else if command == HELP {
// 			println!("help.");
//         }
//         else if command == EXIT {
//             break;
//         }
//     }
    
// }
