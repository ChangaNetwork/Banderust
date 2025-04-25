use std::io;
use std::fs;

fn get_json() -> serde_json::Value {
    let story = fs::read_to_string("src/story.json").expect("Unable to read file");
    return serde_json::from_str(story.as_str()).expect("Couldn't Parse JSON");
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input;
}

fn match_user_input(value: &String, options: &serde_json::Value, a: &str, b: &str ) {
    match value.trim(){
        "A" => {
            println!("{}", &options[a])
        },
        "B" => {
            println!("{}", &options[b])
        },
        _=>println!("Option not valid")
    }
}

fn main(){
    let json: serde_json::Value = get_json();

    println!("{}", json["0"]);
    
    let mut input = get_user_input(); 
    match_user_input(&input, &json, "1", "2");
    //println!("{} A? {} - B? {}", input.trim(), input.trim().eq("A"), input.trim().eq("B"));
    
    if input.trim().eq("A"){
        input = get_user_input();
        match_user_input(&input, &json, "3", "4");

    } else if input.trim().eq("B") {
        input = get_user_input();
        match_user_input(&input, &json, "5", "6");
    }
}
