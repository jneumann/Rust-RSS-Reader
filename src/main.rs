extern crate hyper;
extern crate hyper_native_tls;
extern crate rss;
extern crate xml;

use rss::{
    Channel,
    Item,
};
use std::fs::File;
use std::io;
use std::io::{
    BufReader,
};
use xml::reader::{
    EventReader,
    XmlEvent
};
use xml::reader::XmlEvent::{
    Characters,
};

fn load_file() -> BufReader<File> {
    let file = File::open("feedly_export.opm").unwrap();
    let file = BufReader::new(file);

    file
}

fn get_rss_list(buf: BufReader<File>) -> Vec<Vec<String>> {
    let parser = EventReader::new(buf);
    let mut ret_vec = Vec::new();

	for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let mut title = "".to_string();
                let mut url = "".to_string();
                for a in attributes {
                    match a.name.local_name.as_ref() {
                        "title" => {
                            title = a.value.clone()
                        },
                        "xmlUrl" => {
                            url = a.value.clone()
                        },
                        _ => {}
                    }
                }
                if title != "".to_string() && url != "".to_string() {
                    ret_vec.push(vec![title, url]);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    ret_vec
}

fn main() {
    let file = load_file();

    let processed_file = get_rss_list(file);

    for index in 1..processed_file.len() {
        println!("{}) {} | {}", index, processed_file[index][0], processed_file[index][1]);
    }

    println!("");
    println!("     * * * * *     ");
    println!("");
	println!("Enter the number of the feed you would like to see:");
    let reader = io::stdin();
    let mut input_text = String::new();

    reader.read_line(&mut input_text).expect("failed to read line");

    let input_opt = input_text.trim().parse::<i32>();

    let input_int = match input_opt {
        Ok(input_int) => input_int,
        Err(e) => {
            println!("please input a number ({})", e);
            return;
        }
    };

    println!("Downloading Feed");
    let channel = Channel::from_url(&processed_file[{input_int as usize}][1].as_str()).unwrap();
    let channel_items = Channel::items(&channel);
    for i in 0..5 {
        println!("{}", Item::pub_date(&channel_items[i]).unwrap());
        let parser = EventReader::from_str(Item::description(&channel_items[i]).unwrap());

        for e in parser {
            match e {
                Ok(Characters(data)) => {
                    println!("{}", data);
                },
                _ => {}
            }
        }

        println!("");
    }
}
