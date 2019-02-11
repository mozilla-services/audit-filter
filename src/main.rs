extern crate docopt;
#[macro_use]
extern crate serde_derive;

extern crate audit_filter;

use audit_filter::{run, version};
use docopt::Docopt;

const USAGE: &str = "
audit-filter filters the output of \"npm audit --json\"

Usage:
  audit-filter [--json] [--audit=<->] [--nsp-config=<.nsprc>]
  audit-filter (-h | --help | --version)

Options:
  -h --help                       Show this screen.
  --version                       Show version.
  --json                          Output subset of JSON for the unfiltered advisories as an array.
  --audit=<audit>                 NPM Audit JSON file [default: -].
  --nsp-config=<config>           Default filter config [default: .nsprc].
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_audit: String,
    flag_nsp_config: String,
    flag_version: bool,
    flag_json: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(version())).deserialize())
        .unwrap_or_else(|e| e.exit());

    ::std::process::exit(run(&args.flag_audit, &args.flag_nsp_config, args.flag_json));
}
