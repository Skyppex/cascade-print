const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz 1234567890.:,;!?-_()[]{}/\\'\"@#£¤$%€&*+<=>|~^`´¨";

fn main() {
    loop {
        println!("Enter your message: ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut output = String::new();

        for c in input.chars() {
            for a in ALPHABET.chars() {
                if c.to_lowercase().to_string() == a.to_lowercase().to_string() {
                    output.push_str(c.to_string().as_str());
                    break;
                }

                println!("{}{}", output, if c.is_uppercase() { a.to_uppercase().to_string() } else { a.to_string() });
                std::thread::sleep(std::time::Duration::from_millis(20));
            }

            println!("{}", output);
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }
}
