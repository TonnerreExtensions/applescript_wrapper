mod execute;
mod query;

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let action = args.next().expect("Failed to obtain action flag");
    let content = args.next().expect("Failed to obtain content flag");
    match action.trim() {
        "-q" | "--query" => query::query(content.trim()),
        "-x" | "--execute" => execute::execute(content.trim(), false),
        "-X" | "--alter_execute" => execute::execute(content.trim(), true),
        _ => panic!("Unsupported flag"),
    }
}
