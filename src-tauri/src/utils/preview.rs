use std::path::{Path};
use infer::{MatcherType};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct File {
    file_type: String,
    path: String,
    extension: String,
}

impl File {

    pub fn file_type(&self) -> String {
        return self.file_type.clone();
    }
    pub fn path(&self) -> String {
        return self.path.clone();
    }
    pub fn extension(&self) -> String {
        return self.extension.clone();
    }

    fn book(path: String, file_type: String, extension: String) -> Result<File, String> {
        println!("Book: {:?}", path);
        Ok(File { file_type, path, extension })
    }
    fn image(path: String, file_type: String, extension: String) -> Result<File, String> {
        println!("image: {:?}", path);
        Ok(File { file_type, path, extension })
    }

    fn video(path: String, file_type: String, extension: String) -> Result<File, String> {
        println!("video: {:?}", path);
        Ok(File { file_type, path, extension })
    }

    fn audio(path: String, file_type: String, extension: String) -> Result<File, String> {
        println!("audio: {:?}", path);
        Ok(File { file_type, path, extension })
    }

    fn text(path: String, file_type: String, extension: String) -> Result<File, String> {
        Ok(File { file_type, path, extension })
    }

    fn docs(path: String, file_type: String, extension: String) -> Result<File, String> {
        Ok(File { file_type, path, extension })
    }

    fn app(path: String, file_type: String, extension: String) -> Result<File, String> {
        Ok(File { file_type, path, extension })
    }

    fn font(path: String, file_type: String, extension: String) -> Result<File, String> {
        Ok(File { file_type, path, extension })
    }

    fn archive(path: String, file_type: String, extension: String) -> Result<File, String> {
        Ok(File { file_type, path, extension })
    }

    fn custom(path: String, file_type: String, extension: String) -> Result<File, String> {
        Ok(File { file_type, path, extension })
    }

    fn get_file_type(path: &String) -> Option<infer::Type> {
        let path = Path::new(path);
        if let Ok(type_str) = infer::get_from_path(path) {
            return type_str;
        }
        None
    }
}

pub fn preview_file(path: String) -> Result<File, String>
{
    if path.is_empty() {
        return Err(String::from("path is empty"));
    }
    if let Some(file_type) = File::get_file_type(&path) {
        println!("file_type: {:?}", file_type);
        let matcher_type =  file_type.matcher_type();
        // let path_buf = PathBuf::from(path);
        let extension = file_type.extension();

        match matcher_type {
            MatcherType::Book => File::book(path.clone(), String::from("Book"), extension.to_string()),
            MatcherType::Image => File::image(path.clone(), String::from("Image"), extension.to_string()),
            MatcherType::Video => File::video(path.clone(), String::from("Video"), extension.to_string()),
            MatcherType::Audio => File::audio(path.clone(), String::from("Audio"), extension.to_string()),
            MatcherType::Text => File::text(path.clone(), String::from("Text"), extension.to_string()),
            MatcherType::Doc => File::docs(path.clone(), String::from("Doc"), extension.to_string()),
            MatcherType::App => File::app(path.clone(), String::from("App"), extension.to_string()),
            MatcherType::Archive => File::archive(path.clone(), String::from("Archive"), extension.to_string()),
            MatcherType::Font => File::font(path.clone(), String::from("Font"), extension.to_string()),
            MatcherType::Custom => File::custom(path.clone(), String::from("other"), extension.to_string()),
        }
    } else {
        let extension = Path::new(&path).extension().unwrap();
        File::custom(path.clone(), String::from("other"), extension.to_string_lossy().to_string())
    }
}