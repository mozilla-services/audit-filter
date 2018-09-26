extern crate audit_filter;

#[test]
fn it_returns_audit_urls() {
    // yes, these are from different commits
    let result = audit_filter::run(
        "tests/fixtures/screenshots-1844afe49f853f3c1d8e05ba0bdc84cd598c22d5-npm-6.4.1-audit.json",
        "tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json",
    );
    assert!(result.is_ok());
    assert!(match result {
        Ok(ref unacked_advisory_urls) => {
            unacked_advisory_urls.contains(&"https://npmjs.com/advisories/598".to_string())
        }
        _ => false,
    })
}
