#!/usr/bin/env node

const process = require('process');
const fs = require('fs');

const docopt = require('docopt').docopt;

const audit_exclude = require('./audit_exclude');

const doc = `
audit-exclude excludes the output of \"npm audit --json\"
Usage:
  audit-exclude [--json] [--audit=<->] [--nsp-config=<.nsprc>]
  audit-exclude (-h | --help | --version)
Options:
  -h --help                       Show this screen.
  --version                       Show version.
  --json                          Output subset of JSON for the unexcluded vulnerabilities as an array.
  --audit=<audit>                 NPM Audit JSON file [default: -].
  --nsp-config=<config>           Default exclude config [default: .nsprc].
`;

const opts = docopt(doc, {version: audit_exclude.version()});

var auditPath = opts['--audit'];
var configPath = opts['--nsp-config'];

if (opts['--nsp-config'] === '-') {
  configPath = '/dev/stdin';
}
if (opts['--audit'] === '-') {
  auditPath = '/dev/stdin';
}
let audit = fs.readFileSync(auditPath, {'encoding': 'utf8'});
let config = fs.readFileSync(configPath, {'encoding': 'utf8'});

process.exit(audit_exclude.run_wasm(audit, config, opts['--json']));