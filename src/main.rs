fn main() {
    println!("Welcome to Sentence Re-arranger!");
    println!("Please type in your sentence:");
    let mut line = String::new();
    while line != ""{
        // do rearrangign here
        println!("test");
        line = line_read();
        println!("{:?}", line);
    }    

}

fn line_read() -> String{
    let mut line_buffer = String::new();
    std::io::stdin().read_line(&mut line_buffer).unwrap();

    return line_buffer 
}