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