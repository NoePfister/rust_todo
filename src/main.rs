use std::{fs::{self, File}, io, path::Path, process};

const SAVE_PATH :&str = "D://Dokumente/coding/rust_todo/save.txt";

fn main() {
    let mut app = App::new();
    app.load_todos();

    'command: loop {
        App::clear();

        println!("TODO Manager!");
        println!("Please enter your command:");
        println!("q -> Quit application");
        println!("s -> Show Todo's");
        println!("n -> New Todo");
        println!("d -> Mark Todo as done");

        let mut command = String::new();

        match io::stdin().read_line(&mut command) {
            Ok(_) => (),
            Err(_) => {
                println!("Invalid command");
                continue 'command;
            }
        }

        command = command.trim().to_lowercase();

        match command.as_str() {
            "q" => app.quit(),
            "s" => app.show(),
            "n" => app.new_todo(),
            "d" => app.delete_todo(),
            _ => {
                println!("Invalid command");
                continue 'command;
            }
        }
    }
}

struct App {
    todos: Vec<String>,
    save_path: String,
}

impl App {
    fn new() -> App {
        App {
            todos: Vec::new(),
            save_path: String::from(SAVE_PATH),
        }
    }

    fn quit(&mut self) {
        process::exit(0);
    }

    fn show(&mut self) {
        App::clear();
        println!("Todos:");
        self.load_todos();
        // print todo's
        for (i, todo) in self.todos.iter().enumerate() {
            let j = 1 + i;
            println!("{j}. {}", todo);
        }

        //wait for key press:
        println!("Press enter to continue tu menu");
        App::wait_for_input();
    }

    fn new_todo(&mut self) {
        App::clear();
        println!("Please input your new Todo:");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Couldn't read command line!");
            }
        }

        self.todos.push(input.trim().to_string());
        self.save_todos();

    }

    fn check_file(&mut self){
        //check if file exists. if not, create it
        if !Path::new(&self.save_path).exists(){
            match File::create(&self.save_path){
                Ok(_) => return,
                Err(_) => {
                    println!("Couldn't create file");
                    return
                }
            }
        }
    }

    fn load_todos(&mut self) {

        self.check_file();

        self.todos.clear();

        let save_string = match fs::read_to_string(&self.save_path){
            Ok(content) => content,
            Err(_) => {
                println!("Couldn't read save file.");
                String::from("Save file not found\n")
            }
        };

        //iterate throuogh each line
        for line in save_string.lines(){
            self.todos.push(String::from(line));

        }

    }

    fn save_todos(&mut self) {
        let mut save_string = String::new();
        for todo in self.todos.iter() {
            save_string =save_string + todo + "\n";
        }

        fs::write(&self.save_path, save_string).expect("Couldn't write to file!")
    }

    fn delete_todo(&mut self) {
        App::clear();
        println!("Please input the Number, you would like to delete!");

        for (i, todo) in self.todos.iter().enumerate() {
            let j = 1 + i;
            println!("{j}. {}", todo);
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Couldn't read command line!");
            }
        }

        let index: usize = match input.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("Please input a number!");
                println!("Press Enter to continue!");
                App::wait_for_input();
                return;
            }
        };

        self.todos.remove(index - 1);
        self.save_todos();
    }

    fn clear() {
        print!("{}[2J", 27 as char);
    }

    fn wait_for_input() {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Couldn't read command line!");
            }
        }
    }
}
