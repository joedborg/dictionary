extern crate json;
extern crate reqwest;

use std::env;
use std::process;

fn underline(word: &String) -> String {
    let mut underline = vec![];
    for _ in 2..word.len() {
        &underline.push("-");
    }

    return underline.join("");
}

fn display(parsed: &json::JsonValue) {
    for result in parsed["results"].members() {
        println!("{}", &result["id"]);
        println!("{}", underline(&result["id"].dump()));
        for lexical_entry in result["lexicalEntries"].members() {
            for entry in lexical_entry["entries"].members() {
                for sense in entry["senses"].members() {
                    for definition in sense["definitions"].members() {
                        println!("{}", definition);
                    }
                }
            }
        }
        println!();
    }
}

fn lookup(word: &String) -> json::JsonValue {
    let client = reqwest::Client::new();
    let url = &format!("https://od-api.oxforddictionaries.com:443/api/v1/entries/{}/{}", "en", word);

    let app_id = match env::var("DICTIONARY_APP_ID") {
        Ok(var) => var,
        Err(_) => {
            eprintln!("You must set DICTIONARY_APP_ID");
            process::exit(1);
        },
    };

    let app_key = match env::var("DICTIONARY_APP_KEY") {
        Ok(var) => var,
        Err(_) => {
            eprintln!("You must set DICTIONARY_APP_KEY");
            process::exit(1);
        }
    };

    let response = client.get(url)
        .header("app_id", app_id)
        .header("app_key", app_key)
        .send().expect("error")
        .text().expect("error");

    return json::parse(&response).unwrap();
}

fn main() {
    let word = match env::args().nth(1) {
        Some(word) => word,
        None => {
            eprintln!("Usage: dictionary <word>");
            process::exit(1);
        }
    };
    let parsed = lookup(&word);
    display(&parsed);
}
