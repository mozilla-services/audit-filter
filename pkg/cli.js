#!/usr/bin/env node

const process = require('process');
const fs = require('fs');

const docopt = require('docopt').docopt;

const audit_filter = require('./audit_filter');

const doc = `
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
`;

const opts = docopt(doc, {version: audit_filter.version()});

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

process.exit(audit_filter.run_wasm(audit, config, opts['--json']));
