use std::process;

fn main() {
    if let Err(error) = r_python::cli::run() {
        eprintln!("error: {error}");
        process::exit(1);
    }
}
