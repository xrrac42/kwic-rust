use super::*;

#[test]
fn test_split_into_words() {
    assert_eq!(
        split_into_words("The quick brown fox"),
        vec!["The", "quick", "brown", "fox"]
    );
    assert_eq!(
        split_into_words("Olá, mundo! 你好"),
        vec!["Olá", "mundo", "你好"]
    );
    assert_eq!(
        split_into_words("  multiple   spaces  "),
        vec!["multiple", "spaces"]
    );
    assert_eq!(
        split_into_words("punctuation,should;split!correctly?"),
        vec!["punctuation", "should", "split", "correctly"]
    );
    assert_eq!(
        split_into_words(""),
        Vec::<&str>::new()
    );
}

#[test]
fn test_process_kwic() {
    let lines = vec![
        "The quick brown fox".to_string(),
        "A brown cat sat".to_string(),
    ];
    let stop_words = default_stop_words();
    let results = process_kwic(&lines, &stop_words, false);

    assert!(results.contains(&("brown".to_string(), "brown fox The quick".to_string())));
    assert!(results.contains(&("cat".to_string(), "cat sat A brown".to_string())));
}

#[test]
fn test_case_sensitive() {
    let lines = vec!["The Quick Brown Fox".to_string()];
    let stop_words = default_stop_words();

    let sensitive = process_kwic(&lines, &stop_words, true);
    let insensitive = process_kwic(&lines, &stop_words, false);

    assert_ne!(sensitive, insensitive);
}

#[test]
fn test_empty_lines() {
    let lines: Vec<String> = vec![];
    let stop_words = default_stop_words();
    let results = process_kwic(&lines, &stop_words, false);
    assert!(results.is_empty());
}

#[test]
fn test_only_stop_words() {
    let lines = vec!["the and of".to_string()];
    let stop_words = default_stop_words();
    let results = process_kwic(&lines, &stop_words, false);
    assert!(results.is_empty());
}

#[test]
fn test_no_stop_words() {
    let lines = vec!["the and of".to_string()];
    let stop_words: Vec<String> = vec![];
    let results = process_kwic(&lines, &stop_words, false);
    assert_eq!(results.len(), 3);
}

#[test]
fn test_unicode_handling() {
    let lines = vec!["Olá mundo 你好 мир".to_string()];
    let stop_words = vec!["mundo".to_string()];
    let results = process_kwic(&lines, &stop_words, false);
    assert!(results.iter().any(|(k, _)| k == "Olá"));
    assert!(results.iter().any(|(k, _)| k == "你好"));
    assert!(results.iter().any(|(k, _)| k == "мир"));
}

#[test]
fn test_punctuation_handling() {
    let lines = vec!["Hello, world! This is: a test.".to_string()];
    let stop_words = default_stop_words();
    let results = process_kwic(&lines, &stop_words, false);
    assert!(results.iter().any(|(k, _)| k == "Hello"));
    assert!(results.iter().any(|(k, _)| k == "world"));
    assert!(results.iter().any(|(k, _)| k == "test"));
}

#[test]
fn test_duplicate_lines() {
    let lines = vec![
        "repeat repeat".to_string(),
        "repeat repeat".to_string(),
    ];
    let stop_words = vec![];
    let results = process_kwic(&lines, &stop_words, false);
    assert_eq!(results.len(), 4);
}

#[test]
fn test_single_word_line() {
    let lines = vec!["unique".to_string()];
    let stop_words = default_stop_words();
    let results = process_kwic(&lines, &stop_words, false);
    assert_eq!(results, vec![("unique".to_string(), "unique".to_string())]);
}

#[test]
fn test_all_stop_words_removed() {
    let lines = vec!["the and of".to_string()];
    let stop_words = vec!["the".to_string(), "and".to_string(), "of".to_string()];
    let results = process_kwic(&lines, &stop_words, false);
    assert!(results.is_empty());
}

#[test]
fn test_no_stop_words_case_sensitive() {
    let lines = vec!["The the THE".to_string()];
    let stop_words = vec![];
    let results = process_kwic(&lines, &stop_words, true);
    assert_eq!(results.len(), 3);
}

#[test]
fn test_stop_words_case_insensitive() {
    let lines = vec!["The the THE".to_string()];
    let stop_words = vec!["the".to_string()];
    let results = process_kwic(&lines, &stop_words, false);
    assert!(results.is_empty());
}

#[test]
fn test_stop_words_case_sensitive() {
    let lines = vec!["The the THE".to_string()];
    let stop_words = vec!["the".to_string()];
    let results = process_kwic(&lines, &stop_words, true);
    assert_eq!(results.len(), 2);
}

#[test]
fn test_long_line() {
    let line = "a ".repeat(1000).trim().to_string();
    let lines = vec![line.clone()];
    let stop_words = vec![];
    let results = process_kwic(&lines, &stop_words, false);
    assert_eq!(results.len(), 1000);
}

#[test]
fn test_line_with_numbers() {
    let lines = vec!["abc 123 def 456".to_string()];
    let stop_words = vec![];
    let results = process_kwic(&lines, &stop_words, false);
    assert!(results.iter().any(|(k, _)| k == "123"));
    assert!(results.iter().any(|(k, _)| k == "456"));
}

#[test]
fn test_line_with_symbols() {
    let lines = vec!["hello @world #rust".to_string()];
    let stop_words = vec![];
    let results = process_kwic(&lines, &stop_words, false);
    assert!(results.iter().any(|(k, _)| k == "world"));
    assert!(results.iter().any(|(k, _)| k == "rust"));
}

#[test]
fn test_multiple_lines_varied_content() {
    let lines = vec![
        "First line".to_string(),
        "Second line".to_string(),
        "Third".to_string(),
    ];
    let stop_words = vec!["line".to_string()];
    let results = process_kwic(&lines, &stop_words, false);
    assert!(results.iter().any(|(k, _)| k == "First"));
    assert!(results.iter().any(|(k, _)| k == "Second"));
    assert!(results.iter().any(|(k, _)| k == "Third"));
}

#[test]
fn test_stop_words_partial_match() {
    let lines = vec!["testing tester test".to_string()];
    let stop_words = vec!["test".to_string()];
    let results = process_kwic(&lines, &stop_words, false);
    assert!(results.iter().any(|(k, _)| k == "testing"));
    assert!(results.iter().any(|(k, _)| k == "tester"));
    assert!(!results.iter().any(|(k, _)| k == "test"));
}

#[test]
fn test_leading_and_trailing_spaces() {
    let lines = vec!["   hello world   ".to_string()];
    let stop_words = default_stop_words();
    let results = process_kwic(&lines, &stop_words, false);
    assert!(results.iter().any(|(k, _)| k == "hello"));
    assert!(results.iter().any(|(k, _)| k == "world"));
}

#[test]
fn test_case_sensitive() {
    let lines = vec!["The Quick Brown Fox".to_string()];
    let stop_words = default_stop_words();
    
    let sensitive = process_kwic(&lines, &stop_words, true);
    let insensitive = process_kwic(&lines, &stop_words, false);
    
    assert_ne!(sensitive, insensitive);
}
// ...existing code...

#[test]
fn test_frase_simples() {
    let linhas = vec!["O rato roeu a roupa".to_string()];
    let stop = vec!["a".to_string()];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.iter().any(|(k, _)| k == "rato"));
    assert!(resultado.iter().any(|(k, _)| k == "roupa"));
}

#[test]
fn test_acentuacao() {
    let linhas = vec!["Café com açúcar".to_string()];
    let stop = vec!["com".to_string()];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.iter().any(|(k, _)| k == "Café"));
    assert!(resultado.iter().any(|(k, _)| k == "açúcar"));
}

#[test]
fn test_pontuacao_portugues() {
    let linhas = vec!["Olá, mundo! Tudo bem?".to_string()];
    let stop = vec!["tudo".to_string()];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.iter().any(|(k, _)| k == "Olá"));
    assert!(resultado.iter().any(|(k, _)| k == "mundo"));
    assert!(resultado.iter().any(|(k, _)| k == "bem"));
}

#[test]
fn test_stop_words_portugues() {
    let linhas = vec!["o e a de do da dos das".to_string()];
    let stop = vec!["o".to_string(), "e".to_string(), "a".to_string(), "de".to_string(), "do".to_string(), "da".to_string(), "dos".to_string(), "das".to_string()];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.is_empty());
}

#[test]
fn test_numeros_portugues() {
    let linhas = vec!["um dois três 4 cinco".to_string()];
    let stop = vec!["um".to_string()];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.iter().any(|(k, _)| k == "dois"));
    assert!(resultado.iter().any(|(k, _)| k == "três"));
    assert!(resultado.iter().any(|(k, _)| k == "4"));
    assert!(resultado.iter().any(|(k, _)| k == "cinco"));
}

#[test]
fn test_simbolos_portugues() {
    let linhas = vec!["Olá @mundo #rustaceos".to_string()];
    let stop = vec![];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.iter().any(|(k, _)| k == "mundo"));
    assert!(resultado.iter().any(|(k, _)| k == "rustaceos"));
}

#[test]
fn test_frase_com_hifen() {
    let linhas = vec!["auto-estima é importante".to_string()];
    let stop = vec!["é".to_string()];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.iter().any(|(k, _)| k == "auto"));
    assert!(resultado.iter().any(|(k, _)| k == "estima"));
    assert!(resultado.iter().any(|(k, _)| k == "importante"));
}

#[test]
fn test_frase_com_apostrofo() {
    let linhas = vec!["Hoje é d'água".to_string()];
    let stop = vec!["é".to_string()];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.iter().any(|(k, _)| k == "Hoje"));
    assert!(resultado.iter().any(|(k, _)| k == "água"));
}

#[test]
fn test_frase_com_maiusculas() {
    let linhas = vec!["BRASIL é lindo".to_string()];
    let stop = vec!["é".to_string()];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.iter().any(|(k, _)| k == "BRASIL"));
    assert!(resultado.iter().any(|(k, _)| k == "lindo"));
}

#[test]
fn test_frase_com_minusculas() {
    let linhas = vec!["brasil é lindo".to_string()];
    let stop = vec!["é".to_string()];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.iter().any(|(k, _)| k == "brasil"));
    assert!(resultado.iter().any(|(k, _)| k == "lindo"));
}

#[test]
fn test_frase_com_espacos_extras() {
    let linhas = vec!["   espaço   extra   ".to_string()];
    let stop = vec![];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.iter().any(|(k, _)| k == "espaço"));
    assert!(resultado.iter().any(|(k, _)| k == "extra"));
}

#[test]
fn test_frase_vazia_portugues() {
    let linhas: Vec<String> = vec![];
    let stop = vec![];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.is_empty());
}

#[test]
fn test_frase_so_stopwords_portugues() {
    let linhas = vec!["de do da".to_string()];
    let stop = vec!["de".to_string(), "do".to_string(), "da".to_string()];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.is_empty());
}

#[test]
fn test_frase_com_palavras_repetidas() {
    let linhas = vec!["amor amor amor".to_string()];
    let stop = vec![];
    let resultado = process_kwic(&linhas, &stop, false);
    assert_eq!(resultado.len(), 3);
}

#[test]
fn test_frase_com_umapalavra() {
    let linhas = vec!["unicidade".to_string()];
    let stop = vec![];
    let resultado = process_kwic(&linhas, &stop, false);
    assert_eq!(resultado, vec![("unicidade".to_string(), "unicidade".to_string())]);
}

#[test]
fn test_frase_com_stopwords_case_sensitive() {
    let linhas = vec!["O o Ó ó".to_string()];
    let stop = vec!["o".to_string()];
    let resultado = process_kwic(&linhas, &stop, true);
    assert_eq!(resultado.len(), 3);
}

#[test]
fn test_frase_com_stopwords_case_insensitive() {
    let linhas = vec!["O o Ó ó".to_string()];
    let stop = vec!["o".to_string()];
    let resultado = process_kwic(&linhas, &stop, false);
    assert_eq!(resultado.len(), 2);
}

#[test]
fn test_frase_com_palavras_compostas() {
    let linhas = vec!["guarda-chuva para-raios".to_string()];
    let stop = vec![];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.iter().any(|(k, _)| k == "guarda"));
    assert!(resultado.iter().any(|(k, _)| k == "chuva"));
    assert!(resultado.iter().any(|(k, _)| k == "para"));
    assert!(resultado.iter().any(|(k, _)| k == "raios"));
}

#[test]
fn test_frase_com_palavra_com_ç() {
    let linhas = vec!["coração maçã".to_string()];
    let stop = vec![];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.iter().any(|(k, _)| k == "coração"));
    assert!(resultado.iter().any(|(k, _)| k == "maçã"));
}

#[test]
fn test_frase_com_palavra_com_til() {
    let linhas = vec!["pão limão mamão".to_string()];
    let stop = vec![];
    let resultado = process_kwic(&linhas, &stop, false);
    assert!(resultado.iter().any(|(k, _)| k == "pão"));
    assert!(resultado.iter().any(|(k, _)| k == "limão"));
    assert!(resultado.iter().any(|(k, _)| k == "mamão"));
}


