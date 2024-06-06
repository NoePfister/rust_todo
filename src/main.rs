use std::{fs, io, process};

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
            save_path: String::from("/rust_todo.txt"),
        }
    }

    fn quit(&mut self) {
        process::exit(0);
    }

    fn show(&mut self) {
        App::clear();
        println!("Todos:");
        // print todo's
        for (i,todo) in self.todos.iter().enumerate() {
            let j = 1+i;
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

    }

    fn load_todos(&mut self) {


        let save_string = fs::read_to_string(&self.save_path).expect("Couldn't read save file");    

        println!("{}",save_string);
    }

    fn save_todos(&mut self) {
        let mut save_string = String::new();
        for todo in self.todos.iter(){
            save_string += todo;
        }

        fs::write(&self.save_path,save_string).expect("Couldn't write to file!")
    }

    fn delete_todo(&mut self){
        App::clear();
        println!("Please input the Number, you would like to delete!");

        for (i,todo) in self.todos.iter().enumerate() {
            let j = 1+i;
            println!("{j}. {}", todo);
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Couldn't read command line!");
            }
        }

        let index: usize = match input.trim().parse(){
            Ok(i) => i,
            Err(_) => {
                println!("Please input a number!");
                println!("Press Enter to continue!");
                App::wait_for_input();
                return;
            },
        };

        self.todos.remove(index-1);

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
