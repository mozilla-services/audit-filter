extern crate failure;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate wasm_bindgen;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::result::Result;

use failure::Error;
use failure::ResultExt;
use wasm_bindgen::prelude::*;

pub const STDIN_STR: &str = "-";
const NO_ADVISORIES_FOUND: &str = "No advisories found after filtering.";

pub type AdvisoryID = u32;
pub type AdvisoryURL = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct NPMAudit {
    pub advisories: HashMap<AdvisoryID, Advisory>,
}

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct Advisory {
    pub findings: Vec<AdvisoryFinding>,
    pub id: AdvisoryID,
    pub title: String,
    pub module_name: String,
    pub overview: String,
    pub recommendation: String,
    pub severity: String,
    pub url: AdvisoryURL,
}

impl Ord for Advisory {
    fn cmp(&self, other: &Advisory) -> Ordering {
        self.url.cmp(&other.url)
    }
}

impl PartialOrd for Advisory {
    fn partial_cmp(&self, other: &Advisory) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Advisory {
    fn eq(&self, other: &Advisory) -> bool {
        self.url == other.url
    }
}

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct AdvisoryFinding {
    pub version: String,
    pub paths: Vec<String>,
    pub dev: bool,
    pub optional: bool,
    pub bundled: bool,
}

impl PartialEq for AdvisoryFinding {
    fn eq(&self, other: &AdvisoryFinding) -> bool {
        self.version == other.version && self.paths == other.paths
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NSPConfig {
    pub exceptions: Vec<AdvisoryURL>,
}

pub fn parse_audit(path: &str) -> Result<NPMAudit, Error> {
    let audit: NPMAudit;

    if path == STDIN_STR {
        audit = serde_json::from_reader(io::stdin())
            .with_context(|e| format!("Error parsing audit JSON from stdin: {}", e))?
    } else {
        let fin = File::open(path)
            .with_context(|e| format!("Error opening audit JSON {}: {}", path, e))?;
        audit = serde_json::from_reader(fin)
            .with_context(|e| format!("Error parsing audit JSON: {}", e))?
    }
    Ok(audit)
}

fn parse_audit_from_str(s: &str) -> Result<NPMAudit, Error> {
    let audit: NPMAudit =
        serde_json::from_str(s).with_context(|e| format!("Error parsing audit JSON: {}", e))?;

    Ok(audit)
}

fn parse_nsp_config_from_str(s: &str) -> Result<NSPConfig, Error> {
    let config: NSPConfig = serde_json::from_str(s)
        .with_context(|e| format!("Error parsing nsp config JSON: {}", e))?;

    Ok(config)
}

pub fn parse_nsp_config(path: &str) -> Result<NSPConfig, Error> {
    let config: NSPConfig;

    if path == STDIN_STR {
        config = serde_json::from_reader(io::stdin())
            .with_context(|e| format!("Error parsing nsp config JSON from stdin: {}", e))?
    } else {
        let fin = File::open(path)
            .with_context(|e| format!("Error opening nsp config JSON {}: {}", path, e))?;
        config = serde_json::from_reader(fin)
            .with_context(|e| format!("Error parsing nsp config JSON: {}", e))?
    }
    Ok(config)
}

pub fn filter_advisories_by_url(
    audit: NPMAudit,
    nsp_config: &NSPConfig,
) -> Result<Vec<Advisory>, Error> {
    let mut unacked_advisories: Vec<Advisory> = vec![];

    for (_, advisory) in audit.advisories {
        if !nsp_config.exceptions.contains(&advisory.url) {
            unacked_advisories.push(advisory)
        }
    }

    unacked_advisories.sort_unstable_by_key(|a| a.id);
    Ok(unacked_advisories)
}

#[wasm_bindgen]
pub fn version() -> String {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
    );
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) => format!("{}.{}.{}", maj, min, pat),
        _ => "".to_owned(),
    }
}

pub fn parse_files_and_filter_advisories_by_url(
    audit_path: &str,
    nsp_config_path: &str,
) -> Result<Vec<Advisory>, Error> {
    let nsp_config = parse_nsp_config(nsp_config_path)?;
    let audit = parse_audit(audit_path)?;
    let unacked_advisories = filter_advisories_by_url(audit, &nsp_config)?;
    Ok(unacked_advisories)
}

pub fn parse_strs_and_filter_advisories_by_url(
    audit_str: &str,
    nsp_config_str: &str,
) -> Result<Vec<Advisory>, Error> {
    let nsp_config = parse_nsp_config_from_str(nsp_config_str)?;
    let audit = parse_audit_from_str(audit_str)?;
    let unacked_advisories = filter_advisories_by_url(audit, &nsp_config)?;
    Ok(unacked_advisories)
}

pub fn get_advisory_urls(advisories: Vec<Advisory>) -> Vec<AdvisoryURL> {
    advisories
        .into_iter()
        .map(|a| a.url)
        .collect::<Vec<AdvisoryURL>>()
}

pub fn format_json_output(advisories: &Vec<Advisory>) -> Result<String, Error> {
    let formatted = serde_json::to_string_pretty(&advisories).with_context(|e| {
        format!(
            "{{\"error\": \"error formatting advisories as json: {}\"}}",
            e
        )
    })?;
    Ok(formatted)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: &str);
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}
macro_rules! err {
    ($($t:tt)*) => (error(&format!($($t)*)))
}

#[wasm_bindgen]
pub fn run_wasm(audit_str: &str, nsp_config_str: &str, output_json: bool) -> i32 {
    match parse_strs_and_filter_advisories_by_url(audit_str, nsp_config_str) {
        Ok(unacked_advisories) => {
            if output_json {
                log!("{}", format_json_output(&unacked_advisories).unwrap())
            }
            if unacked_advisories.is_empty() {
                if !output_json {
                    log!("{}", NO_ADVISORIES_FOUND);
                }
                return 0;
            } else if !unacked_advisories.is_empty() {
                if !output_json {
                    err!(
                        "Unfiltered advisories:\n  {}",
                        get_advisory_urls(unacked_advisories).join("\n  ")
                    );
                }
                return 1;
            }
            unimplemented!() // should never haappen
        }
        Err(err) => {
            err!("{}", err);
            2
        }
    }
}

pub fn run(audit_path: &str, nsp_config_path: &str, output_json: bool) -> i32 {
    match parse_files_and_filter_advisories_by_url(audit_path, nsp_config_path) {
        Ok(unacked_advisories) => {
            if output_json {
                println!("{}", format_json_output(&unacked_advisories).unwrap())
            }
            if unacked_advisories.is_empty() {
                if !output_json {
                    println!("{}", NO_ADVISORIES_FOUND);
                }
                return 0;
            } else if !unacked_advisories.is_empty() {
                if !output_json {
                    eprintln!(
                        "Unfiltered advisories:\n  {}",
                        get_advisory_urls(unacked_advisories).join("\n  ")
                    );
                }
                return 1;
            }
            unimplemented!() // should never haappen
        }
        Err(err) => {
            eprintln!("{}", err);
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // not a real advisory just a copy of 566 to test numeric sorting by ID
    fn setup_test_adv_51600() -> Advisory {
        Advisory {
            findings: vec![AdvisoryFinding {
                version: "2.16.3".to_string(),
                paths: vec![
                    "david>npm>npm-registry-client>request>hawk>boom>hoek".to_string(),
                    "david>npm>npm-registry-client>request>hawk>hoek".to_string(),
                ],
                dev: false,
                optional: false,
                bundled: false,
            }],
            id: 51600,
            title: "Prototype Pollution".to_string(),
            module_name: "hoek".to_string(),
            url: "https://nodesecurity.io/advisories/51600".to_string(),
            overview: "Versions of `hoek` prior to 4.2.1 and 5.0.3 are vulnerable to prototype pollution.\n\nThe `merge` function, and the `applyToDefaults` and
 `applyToDefaultsWithShallow` functions which leverage `merge` behind the scenes, are vulnerable to a prototype pollution attack when provided an _unvalidated
_ payload created from a JSON string containing the `__proto__` property.\n\nThis can be demonstrated like so:\n\n```javascript\nvar Hoek = require('hoek');\n
var malicious_payload = '{\"__proto__\":{\"oops\":\"It works !\"}}';\n\nvar a = {};\nconsole.log(\"Before : \" + a.oops);\nHoek.merge({}, JSON.parse(malicious
_payload));\nconsole.log(\"After : \" + a.oops);\n```\n\nThis type of attack can be used to overwrite existing properties causing a potential denial of servic
e.".to_string(),
            severity: "moderate".to_string(),
            recommendation: "Update to version 4.2.1, 5.0.3 or later.".to_string(),
        }
    }

    fn setup_test_adv_566() -> Advisory {
        Advisory {
            findings: vec![AdvisoryFinding {
                version: "2.16.3".to_string(),
                paths: vec![
                    "david>npm>npm-registry-client>request>hawk>boom>hoek".to_string(),
                    "david>npm>npm-registry-client>request>hawk>hoek".to_string(),
                ],
                dev: false,
                optional: false,
                bundled: false,
            }],
            id: 566,
            title: "Prototype Pollution".to_string(),
            module_name: "hoek".to_string(),
            url: "https://nodesecurity.io/advisories/566".to_string(),
            overview: "Versions of `hoek` prior to 4.2.1 and 5.0.3 are vulnerable to prototype pollution.\n\nThe `merge` function, and the `applyToDefaults` and
 `applyToDefaultsWithShallow` functions which leverage `merge` behind the scenes, are vulnerable to a prototype pollution attack when provided an _unvalidated
_ payload created from a JSON string containing the `__proto__` property.\n\nThis can be demonstrated like so:\n\n```javascript\nvar Hoek = require('hoek');\n
var malicious_payload = '{\"__proto__\":{\"oops\":\"It works !\"}}';\n\nvar a = {};\nconsole.log(\"Before : \" + a.oops);\nHoek.merge({}, JSON.parse(malicious
_payload));\nconsole.log(\"After : \" + a.oops);\n```\n\nThis type of attack can be used to overwrite existing properties causing a potential denial of servic
e.".to_string(),
            severity: "moderate".to_string(),
            recommendation: "Update to version 4.2.1, 5.0.3 or later.".to_string(),
        }
    }

    fn setup_test_adv_577() -> Advisory {
        Advisory {
            findings: vec![AdvisoryFinding {
                version: "4.12.0".to_string(),
                paths: vec!["jpm>firefox-profile>lodash".to_string()],
                dev: false,
                optional: false,
                bundled: false,
            }],
            id: 577,
            title: "Prototype Pollution".to_string(),
            module_name: "lodash".to_string(),
            url: "https://nodesecurity.io/advisories/577".to_string(),
            overview: "Versions of `lodash` before 4.17.5 are vulnerable to prototype pollution. \n\nThe vulnerable functions are 'defaultsDeep', 'merge', and 'me
rgeWith' which allow a malicious user to modify the prototype of `Object` via `__proto__` causing the addition or modification of an existing property that wi
ll exist on all objects.\n\n".to_string(),
            severity: "low".to_string(),
            recommendation: "Update to version 4.17.5 or later.".to_string(),
        }
    }

    fn setup_test_audit() -> NPMAudit {
        let mut advisories = HashMap::new();
        advisories.insert(566, setup_test_adv_566());
        advisories.insert(577, setup_test_adv_577());

        NPMAudit { advisories }
    }

    #[test]
    fn it_should_treat_advisories_with_the_same_url_as_equal() {
        assert_eq!(setup_test_adv_566(), setup_test_adv_566())
    }

    #[test]
    fn it_should_treat_advisories_with_different_urls_as_not_equal() {
        assert_ne!(setup_test_adv_566(), setup_test_adv_577())
    }

    #[test]
    fn it_should_filter_none_for_empty_nsp_config() {
        let audit = setup_test_audit();
        let empty_nsp_config = &NSPConfig { exceptions: vec![] };

        let empty_filtered_result = filter_advisories_by_url(audit, empty_nsp_config);
        assert!(empty_filtered_result.is_ok());
        let empty_filtered = get_advisory_urls(empty_filtered_result.unwrap());
        assert_eq!(
            vec![
                "https://nodesecurity.io/advisories/566".to_string(),
                "https://nodesecurity.io/advisories/577".to_string(),
            ],
            empty_filtered
        );
    }

    #[test]
    fn it_should_filter_an_advisory() {
        let audit = setup_test_audit();
        let nsp_config = &NSPConfig {
            exceptions: vec![
                "https://nodesecurity.io/advisories/577".to_string(),
                "https://nodesecurity.io/advisories/566".to_string(),
            ],
        };

        let filtered_result = filter_advisories_by_url(audit, nsp_config);
        assert!(filtered_result.is_ok());
        let filtered = filtered_result.unwrap();
        assert!(filtered.is_empty());
    }

    #[test]
    fn it_should_filter_an_advisory_into_an_numerically_sorted_list() {
        let mut audit = setup_test_audit();
        audit.advisories.insert(5660, setup_test_adv_51600());

        let nsp_config = &NSPConfig { exceptions: vec![] };

        let filtered_result = filter_advisories_by_url(audit, nsp_config);
        assert!(filtered_result.is_ok());
        let filtered = get_advisory_urls(filtered_result.unwrap());
        assert_eq!(
            vec![
                "https://nodesecurity.io/advisories/566".to_string(),
                "https://nodesecurity.io/advisories/577".to_string(),
                "https://nodesecurity.io/advisories/51600".to_string(),
            ],
            filtered
        );
    }
}
