extern crate quick_xml;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::io;
use std::fs;
use quick_xml::events::Event;
use std::collections::HashMap;
use regex::Regex;

lazy_static! {
    static ref SPACE_RE: Regex = Regex::new("^[\r\n\t ]+$").unwrap();
}

fn read_xml(path: &str) -> HashMap<String, String> {
    let mut id_txt = HashMap::new();
    let file = fs::File::open(path);
    let file_reader = io::BufReader::new(file.unwrap());
    let mut xml_reader = quick_xml::Reader::from_reader(file_reader);
    let mut buf = Vec::new();
    let mut txt = Vec::new();
    let mut id = String::new();
    loop {
        match xml_reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"seekvideo" => {
                        for attr in e.attributes() {
                            let attr = attr.unwrap();
                            if attr.key == b"id" {
                                id = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    }
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                let s = e.unescape_and_decode(&xml_reader).unwrap();
                if !SPACE_RE.is_match(&s) {
                    txt.push(s);
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name() == b"seekvideo" {
                    let s = txt.join("");
                    id_txt.insert(id.clone(), s);
                }
                txt.clear();
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", xml_reader.buffer_position(), e),
            _ => (),
        }
    }
    buf.clear();
    return id_txt
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <L0 path> <L1 path>", args[0]);
        std::process::exit(1);
    }
    
    let en = read_xml(&args[1]);
    let th = read_xml(&args[2]);

    for (id, en_txt) in en.iter() {
        match th.get(id) {
            Some(ref th_txt) => {
                println!("{}\t{}",
                         en_txt.replace("\t", " "),
                         th_txt.replace("\t", " "));
            },
            _ => {}
        }
    }
}
