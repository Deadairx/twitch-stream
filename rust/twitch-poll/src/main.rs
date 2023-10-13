mod poll_request;

use std::env;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    title: String,

    #[arg(short, long, default_value = "1800")]
    duration: Option<u64>,

    #[arg(short, long)]
    choices: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    let user_id_key = "USER_ID";

    let poll = poll_request::PollRequest{
        broadcaster_id: env::var(user_id_key).unwrap(),
        title: args.title,
        duration: args.duration.unwrap(),
        choices: args.choices.iter().map(|c| poll_request::Choice{title: c.clone()}).collect(),
        //channel_points_voting_enabled: None,
        //channel_points_per_vote: None,
    };


    // TODO Print JSON
    println!("{}", serde_json::to_string(&poll).unwrap());
}
