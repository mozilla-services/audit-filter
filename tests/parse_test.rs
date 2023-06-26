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
fn it_parses_npm_8_11_0_audit_output() {
    let path =
        "tests/fixtures/screenshots-977f8734b4f57db9f8fc82d613cd724e5766ed2d-npm-8.11.0-audit.json"
            .to_string();
    let parsed_result = audit_exclude::parse_audit(&path);
    
    assert!(match parsed_result {
        Ok(ref parsed) => parsed.vulnerabilities.as_ref().unwrap().contains_key("dicer"),
        Err(err) => {
            println!("Error: {:?}", err);
            false
        },
    })
}

#[test]
fn it_parses_npm_9_5_1_audit_output() {
    let path =
        "tests/fixtures/screenshots-recursion-issue-npm-9.5.1-audit.json"
            .to_string();
    let parsed_result = audit_exclude::parse_audit(&path);
    
    assert!(match parsed_result {
        Ok(ref parsed) => {
            parsed.vulnerabilities.as_ref().unwrap().contains_key("semantic-release")
        }
        Err(err) => {
            println!("Error: {:?}", err);
            false
        },
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
