use std::io;
use std::fs;
use serde::{Serialize, Deserialize};

static STORY_PATH: &str = "src/story.json";
#[derive(Debug, Serialize, Deserialize)]
struct Node {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    b: Option<Box<Node>>,
}

fn get_json() -> Option<Box<Node>> {
    let story = fs::read_to_string(STORY_PATH).expect("Unable to read file");
    return serde_json::from_str(story.as_str()).expect("Couldn't Parse JSON");
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input;
}

fn play(node: Option<Box<Node>>) {
    if node.is_none() {
        println!("The end.");
        return;
    }
    let node = node.unwrap();
    println!("{}", node.text);
    if node.a.is_none() && node.b.is_none() {
        println!("The end.");
        return;
    }
    println!("\nChoose an option:");
    if node.a.is_some() {
        println!("A: {}", node.a.as_ref().unwrap().text);
    }
    if node.b.is_some() {
        println!("B: {}", node.b.as_ref().unwrap().text);
    }
    let input = get_user_input();
    if input.trim().eq("A") {
        play(node.a);
    } else if input.trim().eq("B") {
        play (node.b);
    } else {
        println!("Invalid input. Please enter A or B.");
        play(Some(node));
    }
    
    //println!("{}", node.text);
}
fn main(){
    let story: Option<Box<Node>> = get_json();
    play(story);
}
