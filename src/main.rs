use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    todo: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    let mut todos: Vec<String> = Vec::new();

    let added_todo: String = args.todo.to_string();

    todos.push(added_todo);

    println!("Hello, there! your todos: {:?}", todos);
}
