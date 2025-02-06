use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::handlers::handler::{get_number, is_armstrong, is_perfect, is_prime};

#[derive(Deserialize)]
struct Params {
    number: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct NumberProperties {
    number: i32,
    is_prime: bool,
    is_perfect: bool,
    properties: Vec<String>,
    digit_sum: u32,
    fun_fact: String,
}

impl NumberProperties {
    fn new(
        number: i32,
        is_prime: bool,
        is_perfect: bool,
        properties: Vec<String>,
        digit_sum: u32,
        fun_fact: String,
    ) -> Self {
        NumberProperties {
            number,
            is_prime,
            is_perfect,
            properties,
            digit_sum,
            fun_fact,
        }
    }
}

#[get("/health")]
async fn get_health_status() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": 200,
        "message": "Healthy"
    }))
}

#[get("classify-number")]
async fn classify_number(query: web::Query<Params>) -> impl Responder {
    let number: i32 = query.number;
    if number < 0 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status": 400,
            "number": "not positive",
            "error": true,
            "message": "Number must be positive"
        }));
    }
    
    let fact = get_number(number).await;
    let mut properties = Vec::new();

    let is_prime = is_prime(number as u64);
    let is_perfect = is_perfect(number as u64);
    let is_even = number % 2 == 0;
    let digit_sum = number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();
    if is_armstrong(number as u32) {
        properties.push("armstrong".to_string());
    };
    if is_even {
        properties.push("even".to_string());
    } else {
        properties.push("odd".to_string());
    }

    let return_value = NumberProperties::new(
        number,
        is_prime,
        is_perfect,
        properties,
        digit_sum,
        fact.unwrap(),
    );

    HttpResponse::Ok().json(serde_json::json!(return_value))
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(get_health_status).service(classify_number);
}
