use std::env;
use std::fs;
use std::process;
use inflate::inflate_bytes_zlib;
use zune_inflate::DeflateDecoder;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use byteorder::{ByteOrder, LittleEndian}; 

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print!("no target specified!");
        process::exit(2);
    }
    let file_path = &args[1];
    print!("{}",file_path);

    run(file_path);
}

fn run(file_path:&String) {
    let contents = fs::read(file_path);
    match contents {
       Ok(file) => parse_data(&file),
       Err(error) => panic!("Problem opening the file: {:?}", error)
    };
}

#[derive(Debug)]
struct Mld {
   format_version: u16, // 2 bytes
   workshop_id: u64, // 8 bytes
   name_length:u8, // 1 byte
   name:Vec<u8>, // N bytes
   creator_name_length:u8, // 1 byte
   creator_name: Vec<u8>, // N bytes
   level_count: u16, // 2 bytes
   inflated_data: Vec<u8>, // a lot of bytes :thumbsup:
}

fn parse_data(data:&[u8]) {
    let name_length =  &data[10];
    let creator_name_length = &data[(11+name_length) as usize];
    let parsed_data = Mld {
        format_version: LittleEndian::read_u16(&data[0..=1]),
        workshop_id: LittleEndian::read_u64(&data[2..=9]),
        name_length: data[10],
        name: data[11..=(10+*name_length as usize)].to_vec(),
        creator_name_length: data[(11+name_length) as usize],
        creator_name: data[((12+name_length) as usize)..=((11+name_length+creator_name_length) as usize)].to_vec(),
        level_count: LittleEndian::read_u16(&data[((12+name_length+creator_name_length) as usize)..=((13+name_length+creator_name_length) as usize)]),
        inflated_data: inflate(&data[((14+name_length+creator_name_length) as usize)..])
    };
    write_to_file(parsed_data);
}

fn inflate(data: &[u8]) -> Vec<u8> {
    let mut decoder = DeflateDecoder::new(&data);
    decoder.decode_zlib().unwrap()
    //inflate_bytes_zlib(data).unwrap()
}

fn write_to_file(data:Mld) {
    let path = Path::new("finished.txt");
    let display = path.display();
     let mut new_file = match File::create(&path) {
         Err(why) => panic!("couldn't create {}: {}", display, why),
         Ok(new_file) => new_file,
     };

     let info:String = data.format_version.to_string() +"\n"+
                       &data.workshop_id.to_string()+"\n"+
                       &vec_to_string(data.name)+"\n"+
                       &vec_to_string(data.creator_name)+"\n";
                      
    //  match new_file.write_all(info.as_bytes()) {
    //      Err(why) => panic!("couldn't write to {}: {}", display, why),
    //      Ok(_) => println!("successfully wrote to {}", display),
    //  }
     match new_file.write_all(&data.inflated_data) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn vec_to_string(string_slice:Vec<u8>) -> String {
    String::from(
        match String::from_utf8(string_slice) {
            Ok(str) => str,
            Err(_error) => String::from("String error!")    
        }
    )
}