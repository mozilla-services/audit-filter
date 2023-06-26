extern crate audit_exclude;
extern crate serde_json;

#[test]
fn it_returns_audit_urls_and_json() {
    // yes, these are from different commits
    let result = audit_exclude::parse_files_and_filter_advisories_by_url(
        "tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json",
        "tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json",
    );
    assert!(result.is_ok());
    let unacked_advisories = result.unwrap();

    // check we can output parseable JSON
    let json_result = audit_exclude::format_json_output(&unacked_advisories);
    assert!(json_result.is_ok());
    let json_output: &str = &json_result.unwrap();
    println!("{}", json_output);
    assert!(json_output.contains(&"https://npmjs.com/advisories/598".to_string()));
    let deserialized_result: Result<serde_json::Value, _> = serde_json::from_str(json_output);
    assert!(deserialized_result.is_ok());

    // and text
    assert!(audit_exclude::get_advisory_urls(unacked_advisories)
        .contains(&"https://npmjs.com/advisories/598".to_string()));
}

#[test]
fn it_supports_recursive_npm_9_5_1() {
    let result = audit_exclude::parse_files_and_filter_advisories_by_url(
        "tests/fixtures/screenshots-recursion-issue-npm-9.5.1-audit.json",
        "tests/fixtures/screenshots-recursion-issue-npm-9.5.1-nsprc.json",
    );
    assert!(result.is_ok());
    let unacked_advisories = result.unwrap();

    // check we can output parseable JSON
    let json_result = audit_exclude::format_json_output(&unacked_advisories);

    assert!(json_result.is_ok());
    let json_output: &str = &json_result.unwrap();
    println!("{}", json_output);
    assert!(json_output.contains(&"https://github.com/advisories/GHSA-6w63-h3fj-q4vw".to_string()));
    let deserialized_result: Result<serde_json::Value, _> = serde_json::from_str(json_output);
    assert!(deserialized_result.is_ok());

    // and text
    assert!(audit_exclude::get_advisory_urls(unacked_advisories)
        .contains(&"https://github.com/advisories/GHSA-6w63-h3fj-q4vw".to_string()));
}