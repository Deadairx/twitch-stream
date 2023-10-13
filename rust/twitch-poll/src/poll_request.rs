// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PollRequest {
    #[serde(rename = "broadcaster_id")]
    pub broadcaster_id: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "choices")]
    pub choices: Vec<Choice>,

//    #[serde(rename = "channel_points_voting_enabled")]
//    pub channel_points_voting_enabled: Option<bool>,
//
//    #[serde(rename = "channel_points_per_vote")]
//    pub channel_points_per_vote: Option<u64>,

    #[serde(rename = "duration")]
    pub duration: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    #[serde(rename = "title")]
    pub title: String,
}
