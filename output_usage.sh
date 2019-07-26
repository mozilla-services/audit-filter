#!/usr/bin/env bash

## outputs the usage section for the audit-filter README to readme_usage.md

cat > readme_usage.md <<EOF
\$ audit-filter -h
$(audit-filter -h)
\$ cd audit-filter/example/
\$ cat package.json
$(cat example/package.json)
\$ npm --version
$(npm --version)
\$ npm audit
$(cd example/ && npm audit)
\$ echo \$?
$(echo $?)
\$ cat .nsprc
$(cat example/.nsprc)
\$ npm audit --json | audit-filter
$(cd example/ && npm audit --json | audit-filter)
\$ echo \$?
$(echo $?)
\$ # Alternatively specify audit and config file paths (note: errors print to stderr)
\$ cd .. && audit-filter --nsp-config tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json --audit tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json
$(audit-filter --nsp-config tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json --audit tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json 2>&1)
\$ echo \$?
$(echo $?)
\$ # use --json for JSON output
\$ audit-filter --json --nsp-config tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json --audit tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json
$(audit-filter --json --nsp-config tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json --audit tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json 2>&1)
EOF
