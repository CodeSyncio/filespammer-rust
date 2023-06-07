use std::fs::File;
use std::io::{stdin, stdout, Write};
struct FileSpam {
    name: String,
    content: String,
    iterations: i32,
}
impl FileSpam {
    pub fn user_input(display_text: String) -> String {
        let mut input = String::new();
        print!("{}", display_text);
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Not a correct input type.");
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }
        input
    }
    pub fn spam_instance(name: String, content: String, iterations: i32) -> FileSpam {
        FileSpam {
            name,
            content,
            iterations,
        }
    }

    pub fn spam_files(self) {
        for file in 0..self.iterations {
            let filename = [&self.name, "_", &file.to_string()].join("");

            let mut f = File::create(filename).expect("Failed to create file");
            f.write((&self.content).as_ref())
                .expect("Failed to write to file");

            if file % 100 == 0 {
                println!("written [{}] lines", file)
            }
        }
    }
}

fn main() {
    let filename_input = FileSpam::user_input("File names:".to_string());
    let filencontent_input = FileSpam::user_input("File content:".to_string());
    let fileamount_input = FileSpam::user_input("Amount of files:".to_string())
        .parse::<i32>()
        .unwrap();

    let spammer = FileSpam::spam_instance(filename_input, filencontent_input, fileamount_input);
    spammer.spam_files()
}