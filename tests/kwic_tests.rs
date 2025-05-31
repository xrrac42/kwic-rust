use kwic_rust::*;
use std::collections::HashSet;

#[test]
fn test_case_sensitive_pass() {
    let lines = vec!["The Quick Brown Fox".to_string()];
    let stop_words: HashSet<String> = HashSet::new();

    let sensitive = process_kwic(&lines, &stop_words, true);
    let insensitive = process_kwic(&lines, &stop_words, false);

    assert_ne!(sensitive, insensitive, "Os resultados deveriam ser diferentes para case-sensitive e case-insensitive.");
}

#[test]
fn test_stop_words_pass() {
    let lines = vec!["The quick brown fox".to_string()];
    let stop_words: HashSet<String> = vec!["the".to_string(), "brown".to_string()]
        .into_iter()
        .collect();
    let results = process_kwic(&lines, &stop_words, false);

    assert!(!results.iter().any(|(k, _)| k == "the"));
    assert!(!results.iter().any(|(k, _)| k == "brown"));
    assert!(results.iter().any(|(k, _)| k == "quick"));
}

#[test]
fn test_unicode_handling_pass() {
    let lines = vec!["Olá mundo 你好 мир".to_string()];
    let stop_words: HashSet<String> = HashSet::new();
    let results = process_kwic(&lines, &stop_words, false);

    assert!(results.iter().any(|(k, _)| k == "olá"));
    assert!(results.iter().any(|(k, _)| k == "你好"));
    assert!(results.iter().any(|(k, _)| k == "мир"));
}
/// Este teste deve FALHAR porque compara resultados sensíveis com insensíveis, que são diferentes.
/// A falha é esperada e mostra que a distinção por case está funcionando corretamente.
#[test]
fn test_case_sensitive_fail() {
    let lines = vec!["The Quick Brown Fox".to_string()];
    let stop_words: HashSet<String> = HashSet::new();

    let sensitive = process_kwic(&lines, &stop_words, true);
    let insensitive = process_kwic(&lines, &stop_words, false);

    assert_eq!(sensitive, insensitive, "Os resultados deveriam ser iguais, mas não são.");
}

/// Este teste deve FALHAR porque "brown" está nas stop words e não deveria aparecer.
/// A falha é esperada e demonstra que o filtro de stop words está funcionando corretamente.
#[test]
fn test_stop_words_fail() {
    let lines = vec!["The quick brown fox".to_string()];
    let stop_words: HashSet<String> = vec!["the".to_string(), "brown".to_string()]
        .into_iter()
        .collect();
    let results = process_kwic(&lines, &stop_words, false);

    // A falha é esperada: "brown" não deveria estar nos resultados
    assert!(results.iter().any(|(k, _)| k == "brown"));
}

/// Este teste deve FALHAR porque "mundo" está nas stop words e não deveria aparecer.
/// A falha mostra que o tratamento de Unicode com stop words está correto.
#[test]
fn test_unicode_handling_fail() {
    let lines = vec!["Olá mundo 你好 мир".to_string()];
    let stop_words: HashSet<String> = ["mundo".to_string()].into_iter().collect();
    let results = process_kwic(&lines, &stop_words, false);

    // A falha é esperada: "mundo" deveria ter sido filtrada
    assert!(results.iter().any(|(k, _)| k == "mundo"));
}
