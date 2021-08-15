use itertools::Itertools;

fn main() {
    println!("Welcome to Sentence Re-arranger!");
    println!("Please type in your sentence:");
    let mut line = String::from(" ");

    while line != ""{
        line = line_read();
        trim_newline(&mut line);
        println!("You typed in: {:?}", line);
        let split = line.split(" ");
        let words: Vec<&str> = split.collect();
        
        let mut counter: i32 = 1;

        let mut reconstructed: String = String::new();

        for perm in words.iter().permutations(words.len()).unique(){
            for w in perm{
                reconstructed += w;
                reconstructed += " ";
            }
            reconstructed.pop();

            println!("{:?}: {:?}", counter, reconstructed);
            reconstructed = "".to_string();
            counter += 1;
        }
    }    
    println!("Exiting")
}

fn line_read() -> String{
    let mut line_buffer = String::new();
    std::io::stdin().read_line(&mut line_buffer).unwrap();
    return line_buffer 
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}