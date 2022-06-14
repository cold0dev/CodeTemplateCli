use std::collections::HashMap;
use std::env;
use serde_json::{Result, Value};

fn main() {
    if env::args().count() < 3 {
        println!("Not enough arguments");
        return;
    }

    let mut args = env::args();
    args.next();
    let option = args.next().unwrap();
    let name_arg = args.next().unwrap();

    let name_args_to_count = name_arg.split(":");
    let mut name_args = name_arg.split(":");

    let name = name_args.next().unwrap();

    let mut language = "";

    if name_args_to_count.count() > 1 {
        language = name_args.next().unwrap();
    }

    if option == "get" {
        let client = reqwest::blocking::Client::new();
        let mut body = HashMap::new();
        body.insert("name", name);
        let response = client.post("http://localhost:9090/name")
            .json(&body)
            .send();
        let data = response.unwrap().text();
        if data.is_err() {
            println!("Error getting text");
            return;
        }

        let root_json:Result<Value> = serde_json::from_str(data.unwrap().as_str());

        if root_json.is_err() {
            println!("Error parsing json");
            return;
        }

        let root: Value = root_json.unwrap();

        if !root.is_object() {
            println!("Data error");
            return;
        }
        let template_code_value = root.as_object().unwrap().get("code");

        if template_code_value.is_none() {
            println!("Error");
            return;
        }

        if !template_code_value.unwrap().is_string() {
            println!("Data error");
            return;
        }

        let template_code = template_code_value.unwrap().as_str();

        if template_code.is_none() {
            println!("Error");
            return;
        }

        println!("{}",template_code.unwrap());
    }
    else if option == "find" {
        let client = reqwest::blocking::Client::new();
        let response = client.get("http://localhost:9090/all")
            .send();
        if response.is_err() {
            println!("Connection error");
            return;
        }

        let data = response.unwrap().text();

        if data.is_err() {
            println!("Error getting text");
            return;
        }

        let root_json:Result<Value> = serde_json::from_str(data.unwrap().as_str());

        if root_json.is_err() {
            println!("Error parsing json");
            return;
        }

        let root: Value = root_json.unwrap();

        if !root.is_array() {
            println!("Data error");
            return;
        }

        if root.as_array().unwrap().len() == 0 {
            return;
        }

        println!("Templates:");

        for template_index in 0..root.as_array().unwrap().len() {
            if !root[template_index]["name"].is_string() {
                println!("Data error");
                return;
            }
            else if !root[template_index]["language"].is_string() {
                println!("Data error");
                return;
            }

            let template_name = root[template_index]["name"].as_str();

            if template_name.is_none() {
                println!("Error");
                return;
            }

            let template_language = root[template_index]["language"].as_str();

            if template_language.is_none() {
                println!("Error");
                return;
            }

            if template_name.unwrap().contains(name) {
                if language != "" {
                    if template_language.unwrap().contains(language){
                        println!("{} : {}",template_name.unwrap(),template_language.unwrap());
                    }
                }
                else {
                    println!("{} : {}",template_name.unwrap(),template_language.unwrap());
                }

            }

        }

    }
}
