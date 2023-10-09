use colored::Colorize;

pub fn error(message: String) {
    println!("{} {}", " ERROR ".on_red().white(), message);

    std::process::exit(0);
}
