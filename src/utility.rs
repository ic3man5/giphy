use getopts::Options;

/// Returns the application version specified in the Cargo.toml file.
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Gets a "help" String to print to the console.
pub fn print_usage_string(program_name: &str, opts: &Options) {
    let version: String = get_app_version(); //env!("CARGO_PKG_VERSION");
    let brief = format!("Usage: {} FILE [options]", program_name);
    println!("Version: {}\n{}", version, opts.usage(&brief));
}

 

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_app_version() {
        let version = utility::get_app_version();
        let actual_version = env!("CARGO_PKG_VERSION").to_string();
        assert_eq!(version, actual_version);
    }
}