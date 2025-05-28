use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::io::BufRead; 
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub enum KwicError {
    EmptyInput,
    IoError(std::io::Error),
}

impl fmt::Display for KwicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KwicError::EmptyInput => write!(f, "Input text is empty"),
            KwicError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for KwicError {}

impl From<std::io::Error> for KwicError {
    fn from(err: std::io::Error) -> Self {
        KwicError::IoError(err)
    }
}

pub fn default_stop_words() -> HashSet<String> {
    ["a", "o", "as", "os", "um", "uma", "é", "de", "do", "da", "no", "na"]
        .iter()
        .map(|&s| s.to_string())
        .collect()
}

pub fn load_stop_words<P: AsRef<std::path::Path>>(
    path: Option<P>,
) -> Result<HashSet<String>, KwicError> {
    match path {
        Some(p) => {
            let file = std::fs::File::open(p)?;
            let stop_words: HashSet<String> = std::io::BufReader::new(file)
                .lines()
                .collect::<Result<_, _>>()?;
            Ok(stop_words)
        }
        None => Ok(default_stop_words()),
    }
}

pub fn split_into_words(s: &str) -> Vec<&str> {
    s.unicode_words().collect()
}

pub fn process_kwic(
    lines: &[String],
    stop_words: &HashSet<String>,
    case_sensitive: bool,
) -> Vec<(String, String)> {
    let mut results = Vec::new();

    for line in lines {
        let words = split_into_words(line);
        
        for (i, word) in words.iter().enumerate() {
            let normalized_word = if case_sensitive {
                word.to_string()
            } else {
                word.to_lowercase()
            };
            
            if !stop_words.contains(&normalized_word) {
                let shifted: Vec<&str> = words[i..].iter().chain(&words[..i]).copied().collect();
                let shifted_str = shifted.join(" ");
                results.push((word.to_string(), shifted_str, line.clone()));
            }
        }
    }

    results.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
    results.into_iter().map(|(kw, ctx, _)| (kw, ctx)).collect()
}

pub fn read_lines<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>, KwicError> {
    let file = std::fs::File::open(path)?;
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()?;
    
    if lines.is_empty() {
        Err(KwicError::EmptyInput)
    } else {
        Ok(lines)
    }
}