use std::io;
use std::fs;

fn main(){
    // println!("Che parte della storia vuoi? A o B?");
    let story = fs::read_to_string("src/story.json").expect("Unable to read file");
    //println!("{}", story);
    let json: serde_json::Value =
        serde_json::from_str(story.as_str()).expect("Couldn't Parse JSON");

    //println!("{}", json);
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
