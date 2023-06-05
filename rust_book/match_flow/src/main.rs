#[derive(Debug)]
enum Version {
    Classic,
    Real,
}

#[derive(Debug)]
enum Token {
    Eth(Version),
    Btc(Version),
    Sol, 
    Matic,
}



fn main() {

    let five = Some(5);
    let six = plus_one(five);

    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }


    let other_config: Option<u8> = None;

    if let Some(max) = other_config {
        println!("It's something");
    } else {
        println!("It's a None");
    }

    let mut count = 0;
    let coin: Token = Token::Eth(Version::Real);

    if let Token::Eth(version) = coin {
        println!("{:?} Eth", version);
    } else {
        count += 1;
    }

}



fn current_value_in_dollars(coin: Token) -> f64 {
    match coin {
        Token::Eth(version) => {
            match version {
                Version::Classic => 18.07,
                Version::Real => {
                    println!("Noice");
                    1_892.28
                }
            }
        },
        Token::Btc(version) => {
            match version {
                Version::Classic => 114.59,
                Version::Real => {
                    println!("How do you feel about ordinals ?");
                    27_135.45
                }
            }
        },
        Token::Sol => 21.23,
        Token::Matic => 0.9079,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn reroll() {}







