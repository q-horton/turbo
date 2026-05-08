use std::env;
use serde_json::Value;

use rand::prelude::*;

/// Commands which has text-based responses

/// Banter Command 
///
/// Responds with a bit of banter
const BANTER_REPLY_1: &str = "
Bant her? I only just met her!
";

const BANTER_REPLY_2: &str = "
I hardly know her!
";

const BANTER_REPLY_3: &str = "
Only if she'll let ya!
";

const BANTER_REPLY_0: &str = "
You may say, it is impossible for a man to become like the Machine. And I would reply, that only the smallest mind strives to comprehend its limits.
";

pub fn banter() -> String {
    let response: i32 = rand::thread_rng().gen_range(0..3);

    match response {
        0 => return BANTER_REPLY_0.to_string(),
        1 => return BANTER_REPLY_1.to_string(),
        2 => return BANTER_REPLY_2.to_string(),
        3 => return BANTER_REPLY_3.to_string(),
        _ => return "SPAGETTI".to_string(),
    }
}



/// Roll Command
///
/// RNG that defaults as a 1d20 with 1 as the lowest number.
/// Uses:
///     !roll
///     !roll [max]
///     !roll [min] [max]

pub fn roll(max: Option<i32>, min: Option<i32>, range: Option<i32>) -> String {
    let max: i32 = max.unwrap_or(20);
    let min: i32 = min.unwrap_or(1);
    let range: i32 = range.unwrap_or(1);
    let mut collection: Vec<i32> = (1..range).collect();
    let mut number = rand::thread_rng().gen_range(min..max);

    // Range selection not working due to type errors.
    let selection: String = collection.iter()
        .map(|&collection| collection.to_string() + " ")
        .collect();

    return number.to_string();
}

/// Memberships command
///
/// Pulls current membership count and quorum, returning them as a string.
pub async fn memberships() -> Result<String, reqwest::Error> {
    let sheet_id = env::var("MEMBER_STATS_SHEET_ID").expect("Expected a google sheet ID in the environment");
    let cell_id = env::var("MEMBER_STATS_CELL").expect("Expected a google sheet cell ID in the environment");
    let sheets_api = env::var("GOOGLE_SHEETS_API_KEY").expect("Expected a google sheets API key in the environment");
    let membership_url: String = format!("https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}", sheet_id, cell_id, sheets_api);
    let resp = reqwest::get(membership_url)
        .await?
        .text()
        .await?;

    let parsed: Value = serde_json::from_str(&resp).unwrap();

    // Access fields using square brackets
    let val_a = &parsed["values"][0].get(0).unwrap();
    let members: i32 = val_a.as_str().unwrap().parse().unwrap();
    let val_b = &parsed["values"][1].get(0).unwrap();
    let quorum: i32 = val_b.as_str().unwrap().parse().unwrap();

    let msg: String = format!("There are currently {members} members of UQ MARS, making the current quorum {quorum}");
    println!("{msg}");
    Ok(msg)
}
