pub struct Config {
    pub abpm_dir: String,
    pub version: String,
}

fn abpm_dir() -> String {
    if std::env::var("ABPM_DIR").is_err() {
        if std::env::consts::OS.eq("windows") {
            std::env::set_var("ABPM_DIR", "C:/abpm");
        } else if std::env::consts::OS.eq("linux") {
            std::env::set_var("ABPM_DIR", "/home/bin/.abpm");
        } else if std::env::consts::OS.eq("macos") {
            std::env::set_var("ABPM_DIR", "/home/bin/.abpm");
        } else {
            println!("Unsupported OS");

            std::process::exit(0)
        }
    }

    return std::env::var("ABPM_DIR").unwrap();
}

pub fn new() -> Config {
    let abpm_dir = abpm_dir();

    Config {
        abpm_dir,
        version: String::from("0.0.1"),
    }
}
