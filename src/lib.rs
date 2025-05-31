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

pub fn split_into_words(s: &str) -> Vec<String> {
    use unicode_segmentation::UnicodeSegmentation;

    s.graphemes(true)
        .collect::<Vec<&str>>()
        .split(|&g| g == " " || g == "\n" || g == "\t")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| chunk.concat())
        .collect()
}

pub fn process_kwic(
    lines: &[String],
    stop_words: &HashSet<String>,
    case_sensitive: bool,
) -> Vec<(String, String)> {
    let mut results = Vec::new();

    for line in lines {
        // Divide a linha em palavras
        let words = split_into_words(line);

        for (i, word) in words.iter().enumerate() {
            // Normaliza a palavra para comparação com stop words
            let normalized_word = if case_sensitive {
                word.to_string()
            } else {
                word.to_lowercase()
            };

            // Ignora palavras que estão nas stop words
            if stop_words.contains(&normalized_word) {
                continue;
            }

            // Gera o contexto rotacionado
            let mut context = Vec::new();
            for (j, w) in words.iter().enumerate() {
                if j == i {
                    context.push(w.to_string());
                } else {
                    context.push(if case_sensitive {
                        w.to_string()
                    } else {
                        w.to_lowercase()
                    });
                }
            }

            // Adiciona a palavra-chave e o contexto aos resultados
            results.push((normalized_word, context.join(" ")));
        }
    }

    // Ordena os resultados alfabeticamente
    results.sort_by(|a, b| a.0.cmp(&b.0));
    results
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