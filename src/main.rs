use reqwest;
use std::io;
#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    println!("Enter the amount of money: ");
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).unwrap();
    let amount: f32 = amount.trim().parse().unwrap();
    
    println!("Enter the original currency: ");
    let mut src = String::new();
    io::stdin().read_line(&mut src).unwrap();
    let src = src.trim();

    println!("Enter the destination currency: ");
    let mut dest = String::new();
    io::stdin().read_line(&mut dest).unwrap();
    let dest = dest.trim();

    let response_txt = reqwest::Client::new()
        .get("https://api.exchangerate-api.com/v4/latest/".to_owned() + src)
        .send()
        .await?
        .text()
        .await?;
    match response_txt.find(&format!("\"{}\":", dest)) {
        Some(index) => {
            let mut nbr: usize = 0;
            let len = response_txt.len();
            for i in response_txt[index + dest.len() + 3..len].chars() {
                if i.is_ascii_digit() || i == '.' {
                    nbr += 1
                } else {
                    break;
                }
            }
            let exchange_rate_string = &response_txt[index + dest.len() + 3..index + dest.len() + nbr + 3];
            // Parse the exchange rate value as a float
            let converted_amount = amount * exchange_rate_string.parse::<f32>().unwrap();
            println!("{} {} to {:.2} {}", amount, src, converted_amount, dest);
        },
        None => {
            println!("No {} available", dest);
            return Ok(());
        }
    };
    Ok(())
}
