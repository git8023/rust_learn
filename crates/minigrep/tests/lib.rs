use minigrep;

///
/// 搜索测试
/// - 大小写敏感的
///
#[test]
fn test_search() {
    let query = "duct";
    let contents = "\
    Rust:
    safe, fast, productive.
    Pick three.
    Duct tape.";
    
    assert_eq!(
        vec!["safe, fast, productive."],
        minigrep::search(query, contents)
    );
}

///
/// 搜索测试
/// - 大小写不敏感的
///
#[test]
fn test_search_case_insensitive() {
    let query = "rUsT";
    let contents = "
    Rust:
    safe, fast, productive.
    Pick three.
    Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        minigrep::search_case_insensitive(query, contents)
    );
}
