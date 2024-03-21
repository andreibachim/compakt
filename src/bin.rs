use compakt::{compile, Options};

fn main() {
    match compile(Options {
        source_directory: "../source/",
        target_directory: "../target/",
        overwrite_target_directory: true,
    }) {
        Ok(_) => println!("Success"),
        Err(e) => eprintln!("{}", e),
    }
}
