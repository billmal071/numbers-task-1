use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct NumberFact {
    text: String,
    number: f64,
    found: bool,
    #[serde(rename = "type")]
    fact_type: String,
}

pub async fn get_number(number: i32) -> Result<String, reqwest::Error> { 
    let client = reqwest::Client::new();
    
    let result: NumberFact = client.get(format!("http:://numbersapi.com/{number}/trivia?json"))
        .send()
        .await?
        .json()
        .await?;

    Ok(result.text)
}

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn is_perfect(n: u64) -> bool {
    let mut sum = 0;
    for i in 1..n {
        if n % i == 0 {
            sum += i;
        }
    }
    sum == n
}

pub fn is_armstrong(n: u32) -> bool {
    let mut digits = Vec::new();
    let mut num = n;
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    let num_digits = digits.len() as u32;
    let sum: u32 = digits.iter().map(|d| d.pow(num_digits)).sum();
    sum == n
}
