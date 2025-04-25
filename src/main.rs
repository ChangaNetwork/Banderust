use std::io;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
struct Story {
    beginning: String,
    A: Node,
    B: Node,
}


#[derive(Debug, Serialize, Deserialize)]
struct Node {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    A: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    B: Option<Box<Node>>,
}

fn get_json() -> Story {
    let story = fs::read_to_string("src/test.json").expect("Unable to read file");
    return serde_json::from_str(story.as_str()).expect("Couldn't Parse JSON");
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input;
}

fn print_node_text(node: Option<Box<Node>>){
match node {
    Some(node) => println!("{}", node.text),
    None => println!("No node found"),
    }
}
fn main(){
    let story: Story = get_json();

    println!("{}", story.beginning);
    println!("{}", story.A.text);
    print_node_text(story.A.A);
    print_node_text(story.A.B);
    print_node_text(story.B.A);
    print_node_text(story.B.B);
    //print_node_text(story.A.A?.A);
    //println!("{}", story.b.a.unwrap().text);
    
    /*let mut input = get_user_input(); 
    match_user_input(&input, &json, "1", "2");
    //println!("{} A? {} - B? {}", input.trim(), input.trim().eq("A"), input.trim().eq("B"));
    
    if input.trim().eq("A"){
        input = get_user_input();
        match_user_input(&input, &json, "3", "4");

    } else if input.trim().eq("B") {
        input = get_user_input();
        match_user_input(&input, &json, "5", "6");
    }*/
}
