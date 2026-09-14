fn show_message(text: &String) {
    println!("{text}");
}
fn main() {
    let mut message = String::from("joknem");
    let reader = &message;
    println!("{reader}");
    let writer = &mut message;
    writer.push_str("asd");
    show_message(&message);
    println!("{message}");
}
