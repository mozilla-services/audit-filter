extern crate audit_filter;
extern crate serde_json;

#[test]
fn it_returns_audit_urls_and_json() {
    // yes, these are from different commits
    let result = audit_filter::parse_files_and_filter_advisories_by_url(
        "tests/fixtures/screenshots-1844afe49f853f3c1d8e05ba0bdc84cd598c22d5-npm-6.4.1-audit.json",
        "tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json",
    );
    assert!(result.is_ok());
    let unacked_advisories = result.unwrap();

    // check we can output parseable JSON
    let json_result = audit_filter::format_json_output(&unacked_advisories);
    assert!(json_result.is_ok());
    let json_output: &str = &json_result.unwrap();
    assert!(json_output.contains(&"https://npmjs.com/advisories/598".to_string()));
    let deserialized_result: Result<serde_json::Value, _> = serde_json::from_str(json_output);
    assert!(deserialized_result.is_ok());

    // and text
    assert!(audit_filter::get_advisory_urls(unacked_advisories)
        .contains(&"https://npmjs.com/advisories/598".to_string()));
}
