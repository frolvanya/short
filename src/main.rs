use clap::Parser;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use urlshortener::{client::UrlShortener, providers::Provider};

const API_KEY: &str = "69a0810a0915222f1bf6fe164c832a851a1560bd";

/// Simple program to shorten given URL
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// URL to be shorten
    #[clap(short, long)]
    url: String,
}

fn main() {
    let args = Args::parse();

    let us = UrlShortener::new().unwrap();
    let long_url = args.url;

    match us.generate(
        long_url.clone(),
        &Provider::BitLy {
            token: API_KEY.to_string(),
        },
    ) {
        Ok(res) => {
            if res == *"INVALID_URI" {
                println!("Invalid URL!");
            } else {
                println!("{} >>> {}", long_url, res.trim());

                let mut ctx = ClipboardContext::new().unwrap();
                ctx.set_contents(res.trim().to_owned()).unwrap();
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
