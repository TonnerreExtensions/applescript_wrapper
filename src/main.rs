use clap::{App, Arg};

mod execute;
mod query;

fn main() {
    let matches = App::new(env!("PROVIDER_NAME"))
        .version("1.0")
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .value_name("QUERY")
                .help("Query service")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("execute")
                .short("x")
                .long("execute")
                .value_name("ID")
                .help("Run service")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("alter_execute")
                .short("X")
                .long("alt-execute")
                .value_name("ID")
                .help("Run service")
                .takes_value(true),
        )
        .get_matches();
    if let Some(query) = matches.value_of("query") {
        let output = std::env::var("OUTPUT").expect("Cannot get OUTPUT from env");
        let services = query::query(query.trim());
        std::fs::write(output, services).expect("Failed to write to OUTPUT");
    } else if let Some(id) = matches.value_of("execute") {
        execute::execute(id.trim(), false);
    } else if let Some(id) = matches.value_of("alter_execute") {
        execute::execute(id.trim(), true);
    }
}
