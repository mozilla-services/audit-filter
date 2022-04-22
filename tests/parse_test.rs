extern crate audit_exclude;

#[test]
fn it_parses_npm_audit_output() {
    let path =
        "tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-npm-6.4.1-audit.json"
            .to_string();
    let parsed_result = audit_exclude::parse_audit(&path);
    assert!(parsed_result.is_ok());

    assert!(match parsed_result {
        Ok(ref parsed) => parsed.advisories.as_ref().unwrap().contains_key(&118),
        _ => false,
    })
}

#[test]
fn it_parses_npm_6_9_0_audit_output() {
    let path =
        "tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json"
            .to_string();
    let parsed_result = audit_exclude::parse_audit(&path);
    assert!(parsed_result.is_ok());

    assert!(match parsed_result {
        Ok(ref parsed) => parsed.advisories.as_ref().unwrap().contains_key(&118),
        _ => false,
    })
}

#[test]
fn it_parses_an_nsp_config() {
    let path = "tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json"
        .to_string();
    let parsed_result = audit_exclude::parse_nsp_config(&path);
    assert!(parsed_result.is_ok());

    assert!(match parsed_result {
        Ok(ref parsed) => parsed
            .exceptions
            .contains(&"https://nodesecurity.io/advisories/566".to_string()),
        _ => false,
    })
}
