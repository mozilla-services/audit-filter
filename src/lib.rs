extern crate failure;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::result::Result;

use failure::Error;
use failure::ResultExt;

pub const STDIN_STR: &str = "-";

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
) -> Result<Vec<AdvisoryURL>, Error> {
    let mut unacked_advisory_urls: Vec<AdvisoryURL> = vec![];

    for (_, advisory) in audit.advisories {
        if !nsp_config.exceptions.contains(&advisory.url) {
            unacked_advisory_urls.push(advisory.url)
        }
    }
    unacked_advisory_urls.sort_unstable();
    Ok(unacked_advisory_urls)
}

pub fn run(audit_path: &str, nsp_config_path: &str) -> Result<Vec<AdvisoryURL>, Error> {
    let nsp_config = parse_nsp_config(nsp_config_path)?;
    let audit = parse_audit(audit_path)?;
    let unacked_urls = filter_advisories_by_url(audit, &nsp_config)?;
    Ok(unacked_urls)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        }
    }

    fn setup_test_audit() -> NPMAudit {
        let mut advisories = HashMap::new();
        advisories.insert(566, setup_test_adv_566());
        advisories.insert(577, setup_test_adv_577());

        NPMAudit {
            advisories: advisories,
        }
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
        let empty_filtered = empty_filtered_result.unwrap();
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
}
