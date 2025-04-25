use std::io;
use std::fs;

fn get_json() -> serde_json::Value{
    let story = fs::read_to_string("src/story.json").expect("Unable to read file");
    return serde_json::from_str(story.as_str()).expect("Couldn't Parse JSON");
}

fn main(){
    let json: serde_json::Value = get_json();
    println!("{}", json["0"]);
    
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    
    //println!("You chose: {}!", input.trim());
    match input.trim() {
        "A"=>println!("{}", json["1"]),
        "B"=>println!("{}", json["2"]),
        _=>println!("opzione non valida"),
    }
}
