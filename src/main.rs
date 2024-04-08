use clap::{ArgGroup, Parser};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz 1234567890.,:;!?-_()[]{}/\\'\"@#£¤$%€&*+<=>|~^`´¨";

#[derive(Debug, Parser)]
#[command(about, version, author)]
#[command(group(ArgGroup::new("mode").multiple(false)))]
pub struct CascadeArgs {
    /// The message to cascade print
    #[arg(short, long, group = "mode")]
    pub message: Option<String>,

    /// Whether to loop the program
    #[arg(short, long, group = "mode")]
    pub _loop: bool,
}

fn main() {
    let args = CascadeArgs::parse();

    if let Some(message) = args.message {
        cascade_print(&message);
        return;
    }

    if args._loop {
        loop {
            let input = get_user_input();

            if input.is_empty() {
                println!("Message was empty.");
                continue;
            }

            cascade_print(&input);
        }
    }

    let input = get_user_input();

    if input.is_empty() {
        println!("Message was empty.");
        return;
    }

    cascade_print(&input);
}

fn get_user_input() -> String {
    println!("Enter your message: ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_owned()
}

fn cascade_print(input: &str) {
    let input = clean_input(input);
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

fn clean_input(input: &str) -> String {
    input.replace("\r\n", " ")
        .replace("\r", " ")
        .replace("\n", " ")
        .replace("\t", " ")
        .chars().filter(|c| ALPHABET.contains(&c.to_lowercase().to_string())).collect()
}
