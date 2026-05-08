use rand::prelude::*;

/// Commands which has text-based responses
// Messages
pub const HELP_MESSAGE: &str = "
Our enemies may rest but rust never sleeps.

Hi! You're looking for help, so am I!
If you want a feature added or fixed, make a pull request or raise an issue.
Unsure how to constribute? Ask the friendly team! Check out my source code: https://github.com/uqmars/turbo

Games:
    pong        The game pong

Text:
    banter      Just a bit of banter!
    roll        Defaults 1d20.
                !roll [max] [min]
";

pub const COMMAND_UNDER_REPAIR: &str = "This command is currently being fixed. Hold tight!";


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
///     !roll [max] [min]

pub fn roll(max: Option<i32>, min: Option<i32>) -> String {
    let max: i32 = max.unwrap_or(20);
    let min: i32 = min.unwrap_or(1);
    let number: i32;
    if max >= min {
        number = rand::thread_rng().gen_range(min..max);
    } else {
        number = rand::thread_rng().gen_range(max..min);
    }

    number.to_string()
}
