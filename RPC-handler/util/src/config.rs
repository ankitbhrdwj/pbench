use confy::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyConfig {
    pub server: String,
    pub ports: u64,
}

impl Default for MyConfig {
    fn default() -> Self {
        MyConfig {
            server: "10.10.1.1".to_string(),
            ports: 1,
        }
    }
}

pub fn load_config() -> MyConfig {
    let config = load("../config");
    match config {
        Ok(config) => config,
        Err(err) => {
            println!("{}. Taking default config.", err);
            MyConfig::default()
        }
    }
}