use crate::AppConfig;
use crate::ImageParams;
use std::fs::File;

pub struct FileUtils {
    config: AppConfig,
}

impl FileUtils {
    pub fn new(config: AppConfig) -> FileUtils {
        FileUtils { config }
    }

    pub fn file_exists(&self, file_name: &str) -> bool {
        let full_path = self.build_path(&self.config.input_path, file_name);

        println!("{full_path}");

        let file = File::open(&full_path);
        file.is_ok()
    }

    pub fn build_output_path(
        &self,
        params: &ImageParams,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut output_path = self.config.output_path.clone();

        let height = params.height.unwrap_or(0);
        let width = params.width.unwrap_or(0);

        output_path = self.build_path(
            &output_path,
            &format!("{}{}{}", width, std::path::MAIN_SEPARATOR, height),
        );

        match std::fs::DirBuilder::new()
            .recursive(true)
            .create(&output_path)
        {
            Ok(_) => Ok(output_path),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn build_input_path(&self, params: &ImageParams) -> String {
        let mut final_path: String = self.config.input_path.clone();
        if !final_path.ends_with(std::path::MAIN_SEPARATOR) {
            final_path.push(std::path::MAIN_SEPARATOR);
        }
        final_path.push_str(&params.file_name);
        final_path
    }

    pub fn build_path(&self, path: &str, file_name: &str) -> String {
        let mut final_path: String = String::from(path);
        if !path.ends_with(std::path::MAIN_SEPARATOR) {
            final_path.push(std::path::MAIN_SEPARATOR);
        }
        final_path.push_str(file_name);
        final_path
    }
}
