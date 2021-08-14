use colored::*;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Deserialize, Debug, Serialize)]
struct SignUpReq {
    name: String,
    password: String,
}

struct ApiResp {
    msg: ColoredString,
}

#[derive(Deserialize, Debug, Serialize)]
struct AddBookmarkReq {
    name: String,
    password: String,
    cmd: String,
    url: String,
}

fn main() {
    let api_url = "https://bookshelf-server-api.herokuapp.com/";
    let options = loop {
        let signup = loop {
            println!("{}", "Do you want to sign up?: y/n".bright_cyan().bold());
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input!");
            let f = input.to_owned().to_uppercase();
            let f: &str = f.trim();

            let result = match (f == "Y", f == "N") {
                (true, false) => "Y",
                (false, true) => "N",
                _ => continue,
            };
            break result;
        };
        if signup == "Y" {
            break "signup";
        }
        let bookmark = loop {
            println!(
                "{}",
                "Do you want to add a new bookmark?: y/n"
                    .bright_green()
                    .bold()
            );
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input!");

            let f = input.to_owned().to_uppercase();
            let f: &str = f.trim();
            let result = match (f == "Y", f == "N") {
                (true, false) => "Y",
                (false, true) => "N",
                _ => continue,
            };
            break result;
        };
        if bookmark == "Y" {
            break "bookmark";
        } else {
            break "None";
        }
    };

    match options {
        "signup" => {
            let data = get_sign_up_info();
            let mut url = api_url.to_owned();
            url.push_str("signup");
            let resp = sign_up_req(&url, data);
            println!("{}", resp.msg.bright_yellow().bold());
            println!("{}", "Please restart to add commands".bright_green().bold())
        }
        "bookmark" => {
            let data = get_bookmark_info();
            let mut url = api_url.to_owned();
            url.push_str("setcmd");
            let resp = add_bookmark_req(&url, data);
            println!("{}", resp.msg.bright_yellow().bold());
        }
        _ => println!(
            "{}",
            "No option selected. Exiting Bookshelf CLI.".bright_red()
        ),
    }
}

fn get_sign_up_info() -> SignUpReq {
    let name = loop {
        println!("{}", "Input username:".bright_cyan().bold());
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        let f = input.to_owned();

        if f.len() > 3 {
            break f;
        } else {
            continue;
        }
    };
    let password = loop {
        println!("{}", "Input password:".bright_cyan().bold());
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        let f: String = input.to_owned();

        if f.len() > 3 {
            break f;
        } else {
            continue;
        }
    };
    return SignUpReq { name, password };
}

fn sign_up_req(url: &str, data: SignUpReq) -> ApiResp {
    let created_post = blocking::Client::new().post(url).json(&data).send();
    match created_post {
        Ok(_) => {
            return ApiResp {
                msg: String::from("Sign up success!").bright_green(),
            }
        }
        Err(_) => {
            return ApiResp {
                msg: String::from("Sign up failed!").bright_red(),
            }
        }
    }
}

fn get_bookmark_info() -> AddBookmarkReq {
    let name = loop {
        println!("{}", "Input username:".bright_cyan().bold());
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        let f = input.to_owned();

        if f.len() > 3 {
            break f;
        } else {
            continue;
        }
    };
    let password = loop {
        println!("{}", "Input password:".bright_cyan().bold());
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        let f: String = input.to_owned();

        if f.len() > 3 {
            break f;
        } else {
            continue;
        }
    };
    let cmd = loop {
        println!("{}", "Input bookmark command:".bright_cyan().bold());
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        let f: String = input.to_owned();

        break f;
    };
    let url = loop {
        println!("{}", "Input bookmark url:".bright_cyan().bold());
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        let f: String = input.to_owned();

        break f;
    };
    return AddBookmarkReq {
        name,
        password,
        cmd,
        url,
    };
}

fn add_bookmark_req(url: &str, data: AddBookmarkReq) -> ApiResp {
    let created_post = blocking::Client::new().post(url).json(&data).send();
    println!("{:?}", created_post);
    match created_post {
        Ok(_) => {
            return ApiResp {
                msg: String::from("Added Bookmark!").bright_green(),
            }
        }
        Err(_) => {
            return ApiResp {
                msg: String::from("Err: adding bookmark failed!").bright_red(),
            }
        }
    }
}
