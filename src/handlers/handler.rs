use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug)]
struct NumberFact {
    text: String,
    number: f64,
    found: bool,
    #[serde(rename = "type")]
    fact_type: String,
}

pub async fn get_number(number: i32) -> Result<String, reqwest::Error> { 
    let client = reqwest::Client::builder().use_rustls_tls().build().expect("failed to create client");
    
    let result: NumberFact = client.get(format!("http://numbersapi.com/{}/trivia?json", number))
        .send()
        .await?
        .json()
        .await?;
    
    info!("Number fact: {}", result.text);
    
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

pub fn is_perfect(num: i32) -> bool {
    if num <= 0 {
        return false; // Negative numbers and 0 are not perfect numbers
    }

    let n = num as u32;
    if n == 1 {
        return false; // 1 has no proper divisors other than itself
    }

    let mut sum = 1; // Start with 1 (smallest proper divisor)
    let sqrt_n = (n as f64).sqrt() as u32;

    // Check divisors up to sqrt(n)
    for i in 2..=sqrt_n {
        if n % i == 0 {
            sum += i; // Add divisor `i`
            let other = n / i;
            if other != i {
                sum += other; // Add the paired divisor `n/i`
            }
        }
    }

    sum == n // True if sum of proper divisors equals `n`
}

pub fn is_armstrong(n: i32) -> bool {
    let mut digits = Vec::new();
    let mut num = n;
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    let num_digits = digits.len() as u32;
    let sum: i32 = digits.iter().map(|d| d.pow(num_digits)).sum();
    sum == n
}
