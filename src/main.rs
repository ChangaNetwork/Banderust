use std::io;

fn main(){
    println!("Che parte della storia vuoi? A o B?");
    
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    
    println!("You chose: {}!", input.trim());
    match input.trim() {
        "A"=>println!("AAAAAAA"),
        "B"=>println!("BBBB, la storia andra avanti con l'opzione b"),
        _=>println!("opzione non valida"),
    }
}
