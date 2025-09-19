use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::{f32, fs};
use std::{thread, time};
use rand::prelude::*;
#[tokio::main]
async fn main() {
   let strs : Vec<String> = read_file();

   let mut submissions : i32 = 0;
   loop {

      for x in strs.iter() {
          send_vote(x).await;
          submissions += 1;
          println!("Votes : {submissions}");
          thread::sleep(time::Duration::from_secs(1));
      }
   }

}


fn read_file() -> Vec<String> {
    let unsplit : String = fs::read_to_string("quotes.txt").expect("quotes.txt Does not exist. Please create it");
    let mut contents : Vec<String> = unsplit.split("\n").map(|s: &str| s.to_string()).collect();
    contents.retain(|str| str.len() > 1);
    contents
}


fn get_unix_time() -> u64 {
    let since_epoch : u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("oops").as_secs();
    return since_epoch;
}


fn randomize_string(s : &String) -> String {
    let mut rng = rand::rng();
    let mut new_string: String = String::new();

    for c in s.chars() {
        if rng.random_range(0..10)== 1 {
            let ran_char = char::from_u32(rng.random_range(65..89)).unwrap(); // should never fail
            new_string.push(ran_char);
        }

        if (c == ' ') {
            new_string.push('+');
            continue;
        }
        new_string.push(c);

    }

    new_string
}



async fn send_vote(quote : &String) {
    let quote = randomize_string(&quote);
    let mut rng = rand::rng();
    let id = rng.random_range(-2147483648..2120379099);
    let timestamp = get_unix_time();

    let url = "https://docs.google.com/forms/u/0/d/e/1FAIpQLScts7OlPmq8vow2s5uMH_glOM8vrlrwRBcXmE2WsBnhp0Etkg/formResponse";
    let message = format!("entry.660166958={quote}&fvv=1&partialResponse=%5Bnull%2Cnull%2C%22{id}%22%5D&pageHistory=0&fbzx={id}&submissionTimestamp={timestamp}");
    println!("\n New request : {}",message);
    let client = reqwest::Client::new();
    let bruh = client.post(url).header("Content-Type", "application/x-www-form-urlencoded").body(message).send().await.unwrap();
    println!(" Status code {}",bruh.status());

}
