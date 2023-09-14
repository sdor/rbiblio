use flate2::read::GzDecoder;
use quick_xml::{de::Deserializer, events::{BytesEnd, BytesStart, BytesText, Event}, Reader};
use quick_xml::Writer;
use std::{fs::{read_dir, File}, path::Path};
use std::io::{BufRead, BufReader, Write};
extern crate directories;
use directories::UserDirs;

pub mod pubmed;

use crate::pubmed::PubmedArticle;



fn read_article<R: BufRead>(
    reader: &mut quick_xml::Reader<R>,
) -> Result<Vec<u8>, quick_xml::Error> {
    let mut depth: u32 = 0;
    let mut buf: Vec<u8> = Vec::new();
    let mut output: Vec<u8> = Vec::new();
    let mut writer = Writer::new(&mut output);
    let pubmed_article_end_tag = BytesEnd::new("PubmedArticle");

    match writer.write_event(Event::Start(BytesStart::new("PubmedArticle"))) {
        Ok(_) => loop {
            buf.clear();
            match reader.read_event_into(&mut buf) {
                Ok(event) => match event {
                    Event::Start(e) if e.local_name().as_ref() == b"i" => {
                        let t = Event::Text(BytesText::new("<i>"));
                        match writer.write_event(t) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Event::Start(e) if e.local_name().as_ref() == b"b" => {
                        let t = Event::Text(BytesText::new("<b>"));
                        match writer.write_event(t) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Event::Start(e) if e.local_name().as_ref() == b"sup" => {
                        let t = Event::Text(BytesText::new("<sup>"));
                        match writer.write_event(t) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Event::Start(e) if e.local_name().as_ref() == b"sub" => {
                        let t = Event::Text(BytesText::new("<sub>"));
                        match writer.write_event(t) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Event::Start(e) if e.local_name().as_ref() == b"u" => {
                        let t = Event::Text(BytesText::new("<u>"));
                        match writer.write_event(t) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }

                    Event::End(e) if e.local_name().as_ref() == b"i" => {
                        let t = Event::Text(BytesText::new("</i>"));
                        match writer.write_event(t) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Event::End(e) if e.local_name().as_ref() == b"b" => {
                        let t = Event::Text(BytesText::new("</b>"));
                        match writer.write_event(t) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Event::End(e) if e.local_name().as_ref() == b"sup" => {
                        let t = Event::Text(BytesText::new("</sup>"));
                        match writer.write_event(t) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Event::End(e) if e.local_name().as_ref() == b"sub" => {
                        let t = Event::Text(BytesText::new("</sub>"));
                        match writer.write_event(t) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Event::End(e) if e.local_name().as_ref() == b"u" => {
                        let t = Event::Text(BytesText::new("</u>"));
                        match writer.write_event(t) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }

                    Event::Start(e)
                        if e.name().prefix().as_ref().is_some()
                            && e.name().prefix().unwrap().as_ref() == b"mml"
                            && depth > 0 =>
                    {
                        let t = format!("<{}>", std::str::from_utf8(e.name().as_ref()).unwrap());
                        match writer.write_event(Event::Text(BytesText::new(&t))) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Event::End(e)
                        if e.name().prefix().as_ref().is_some()
                            && e.name().prefix().unwrap().as_ref() == b"mml"
                            && depth > 0 =>
                    {
                        let t = format!("&lt/{}>", std::str::from_utf8(e.name().as_ref()).unwrap());
                        match writer.write_event(Event::Text(BytesText::new(&t))) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }

                    Event::Start(e) if e.local_name().as_ref() == b"DispFormula" => {
                        let t = Event::Text(BytesText::new("<DispFormula>"));
                        match writer.write_event(t) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Event::End(e) if e.local_name().as_ref() == b"DispFormula" => {
                        let t = Event::Text(BytesText::new("</DispFormula>"));
                        match writer.write_event(t) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }

                    Event::Start(e) => {
                        depth += 1;
                        match writer.write_event(Event::Start(e)) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Event::Text(t) if depth > 0 => match writer.write_event(Event::Text(t)) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    },
                    Event::End(e) if e == pubmed_article_end_tag => {
                        match writer.write_event(Event::End(e)) {
                            Ok(_) => return Ok(output),
                            Err(e) => return Err(e),
                        }
                    }
                    Event::End(e) => {
                        depth -= 1;
                        match writer.write_event(Event::End(e)) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    _ => (),
                },
                Err(e) => {
                    return Err(e);
                }
            }
        },
        Err(e) => return Err(e),
    }
}

fn read(path: &Path) {
    //TODO: handle file open error
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let decoder = GzDecoder::new(buf_reader);
    let gz = BufReader::new(decoder);
    let mut reader = Reader::from_reader(gz);
    let mut buf: Vec<u8> = Vec::new();

    //TODO: remove
    let errors_file_name = format!(
        "./results/{}.txt",
        path.file_name().unwrap().to_str().unwrap()
    );
    let errors_file_path = Path::new(&errors_file_name);
    let mut errors_file = File::create(errors_file_path).unwrap();
    // let mut errors_file = BufWriter::new(File::open(&path).unwrap());
    //end remove
    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) if e.name().as_ref() == b"PubmedArticle" => {
                match read_article(&mut reader) {
                    Ok(bytes) => {
                        // println!("{}", std::str::from_utf8(&bytes).unwrap());
                        let mut deserializer =
                            Deserializer::from_str(std::str::from_utf8(&bytes).unwrap());
                        let article = PubmedArticle::deserialize(&mut deserializer);
                        match article {
                            Ok(_) => {
                                // let json = serde_json::to_string_pretty(&article.unwrap()).unwrap();
                                // println!("{:?}", json);
                            }
                            Err(e) => {
                                // println!("{}", std::str::from_utf8(&bytes).unwrap());
                                let err = format!("{}\n", e);
                                errors_file.write_all(&err.as_bytes()).unwrap();
                                errors_file.write_all(&bytes).unwrap();
                                errors_file.write_all("\n".as_bytes()).unwrap();
                                //panic!("{}", e)
                            }
                        }
                    }
                    Err(e) => {
                        panic!("{}", e)
                    }
                }
            }
            _ => (),
        }
    }

    //TODO: remove
    errors_file.flush();
    if errors_file.metadata().unwrap().len() == 0 {
        std::fs::remove_file(errors_file_path).unwrap();
    }
    //end remove
}

fn read_directory(dir: &Path) -> Result<(), std::io::Error> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;

            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "gz" {
                println!("{:?}", path);
                // let file = File::open(path)?;
                read(&path);
            }
        }
    }
    Ok(())
}

fn main() {
    if let Some(user) = UserDirs::new() {
        let home_dir = user.home_dir();
        let _ = read_directory(Path::new("/Users/sdoronin/Downloads/baseline"));
    }
}
