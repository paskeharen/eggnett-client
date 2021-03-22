use std::io::{self, Write};
use clap::{Arg, App};
use crossterm::event::{read, Event, KeyCode};
use md5;
use reqwest::{blocking::Response, header::USER_AGENT};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = App::new("Eggnett client")
        .version("1.0")
        .author("Påskeharen")
        .about("Eggnett lar deg koble til eksterne endepunkt")
        .arg(Arg::with_name("mottaker")
            .value_name("mottaker")
            .required(true)
            .help("Mottaker, e.g. http://example.com/")
            .takes_value(true)
        )
        .arg(Arg::with_name("hemmelighet")
            .short("h")
            .long("hemmelighet")
            .value_name("hemmelighet")
            .default_value("***********")
            .help("Delt hemmelighet")
            .takes_value(true)
        )
        .get_matches();
    
    let url = args.value_of("mottaker").unwrap();
    let secret = args.value_of("hemmelighet").unwrap();
    let client = reqwest::blocking::Client::new();

    println!(r"Eggnett 1.0
Videreformidler tastetrykk til {}:", url);
    loop {
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Esc => break,
                KeyCode::Char(c) => {
                    io::stdout().write(c.to_string().as_bytes())?;
                    io::stdout().flush()?;
                    match send(&client, url, secret, c) {
                        Ok(_) => continue,
                        Err(_) => {
                            eprintln!("\nWhoops! Noe gikk galt.");
                            break;
                        }
                    };
                },
                KeyCode::Enter => {
                    io::stdout().write('\n'.to_string().as_bytes())?;
                    io::stdout().flush()?;
                    break;
                }
                _ => (),
            },
            _ => (),
        }
    }
    Ok(())
}

fn send(client: &reqwest::blocking::Client, url: &str, secret: &str, c: char) -> reqwest::Result<Response> {
    let payload = shroud_in_secrecy(secret, c);
    client.get(url)
        .header(USER_AGENT, "Eggnett/1.0 <github.com/paskeharen/eggnett-client>")
        .header("X-EGG", payload)
        .send()
}
    
fn shroud_in_secrecy(secret: &str, c: char) -> String {
    let mut buff = secret.clone().as_bytes().to_vec();
    buff.append(&mut c.to_string().as_bytes().to_vec());
    format!("{:x}", md5::compute(buff))
}
