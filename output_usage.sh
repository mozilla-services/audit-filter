#!/usr/bin/env bash

## outputs the usage section for the audit-exclude README to readme_usage.md

cat > readme_usage.md <<EOF
\$ audit-exclude -h
$(audit-exclude -h)
\$ cd audit-exclude/example/
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
\$ npm audit --json | audit-exclude
$(cd example/ && npm audit --json | audit-exclude)
\$ echo \$?
$(echo $?)
\$ # Alternatively specify audit and config file paths (note: errors print to stderr)
\$ cd .. && audit-exclude --nsp-config example/.nsprc --audit tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json
$(audit-exclude --nsp-config example/.nsprc --audit tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json 2>&1)
\$ echo \$?
$(echo $?)
\$ # use --json for JSON output
\$ audit-exclude --json --nsp-config example/.nsprc --audit tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json | head
$(audit-exclude --json --nsp-config example/.nsprc --audit tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json 2>&1 | head)
EOF
