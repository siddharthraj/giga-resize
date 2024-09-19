use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub input_path: String,
    pub output_path: String,
    pub bind_address: String,
    pub cache_size: usize,
}

impl AppConfig {
    pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
        dotenv().ok();

        let input_path = Self::fetch_var("INPUT_PATH")?;
        let output_path = Self::fetch_var("OUTPUT_PATH")?;
        let cache_size: usize = Self::fetch_var("CACHE_SIZE")?;
        let bind_address = Self::fetch_var("BIND_ADDRESS")?;

        Ok(AppConfig {
            input_path,
            output_path,
            cache_size,
            bind_address,
        })
    }

    fn fetch_var<T>(var: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: ToString + std::str::FromStr,
        T::Err: std::error::Error + 'static,
    {
        match env::var(var) {
            Ok(var) => var
                .parse::<T>()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>),
            Err(e) => Err(Box::new(e)),
        }
    }
}
