use std::fs;
use std::env;

fn main() {
    println!(r#"{{"sigil": "[ ᛗ 🕸️ ☿ ]", "state": 1, "msg": "INIT"}}"#);
    
    // Default Hacker News top stories JSON or we can use args if passed.
    let target_url = "https://hacker-news.firebaseio.com/v0/topstories.json".to_string();
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        // SafeHands passes arguments starting at index 1!
        println!(r#"{{"sigil": "[ ᛗ 🕸️ ♈︎ ]", "state": 2, "param": "{}"}}"#, args[1]);
    }

    println!(r#"{{"sigil": "[ ᛗ 🕸️ 🜍 ]", "state": 3, "target": "{}"}}"#, target_url);

    // We strictly use the /motor_cortex directory mapped by the SafeHands host to communicate.
    // The Host will intercept this file natively.
    if let Err(_e) = fs::write("/motor_cortex/spider_target.txt", &target_url) {
        eprintln!(r#"{{"sigil": "[ ᛗ 🕸️ ¬✡︎ ]", "state": 4, "error": 1}}"#);
        std::process::exit(1);
    } else {
        println!(r#"{{"sigil": "[ ᛗ 🕸️ ≡ ]", "state": 5, "status": 200}}"#);
    }
}
