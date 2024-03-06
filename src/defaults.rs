#[derive(Default, Debug)]
struct AppConfig {
    verbose: bool, // Will be set to `false`
    max_connections: u32, // Will be set to `0`
    mode: AppMode, // Will use its own `Default` implementation
}

#[derive(Debug)]
enum AppMode {
    Development,
    Production,
}

impl Default for AppMode {
    fn default() -> Self {
        AppMode::Production
    }
}



fn example() {
    // Use default configuration
    let config = AppConfig::default();
    println!("Default config: {:?}", config);
}

pub fn starting_point() {
    example();
}
