### audit-filter

[![crates.io version](https://img.shields.io/crates/v/audit-filter.svg)](https://img.shields.io/crates/v/audit-filter.svg)
[![Build Status](https://travis-ci.org/mozilla-services/audit-filter.svg?branch=master)](https://travis-ci.org/mozilla-services/audit-filter)
[![npm version](https://badge.fury.io/js/audit-filter.svg)](https://badge.fury.io/js/audit-filter)

`audit-filter` takes the output of [`npm audit
--json`](https://docs.npmjs.com/cli/audit) and an
[nsp](https://github.com/nodesecurity/nsp) rc config file [*without
comments*](#fixing-comments-in-nsprc-files) and filters out advisories
according to the nsp offline exceptions format (see usage for an
example).

This provides a migration path from `nsp check` to `npm audit` and
lets projects to use `npm audit` in CI pipelines without masking all
advisories (e.g. with `npm audit || true`).

### Install

#### Requirements

* node 8.x or 10.x
* npm@6 (for `--json` support and newer package-lock.json format)

#### Local NPM package

1. Run `npm install --save-dev audit-filter` to add it as a dev dependency

1. Require an npm version with `npm audit` support in `package.json` e.g.

```json
{
  ...
  "engines": {
    "node": ">=8",
    "npm": ">=6.4.1"
  },
  ...
}
```

1. Add an empty exceptions file named `.nsprc`:


```json
{
  "exceptions": [
  ]
}
```

1. Optionally, add an npm script command:

```json
{
  "scripts": {
    "lint:deps": "npm audit --json | audit-filter --nsp-config=.nsprc --audit=-"
	...
  }
  ...
  "devDependencies": {
    "audit-filter": "0.3.0"
  },
  ...
}
```

and test it with: `npm run lint:deps` or `npm run-script lint:deps`

1. Optionally, set "The minimum level of vulnerability for npm audit to exit with a non-zero exit with [`npm config audit level ('low', 'moderate', 'high', 'critical')`](https://docs.npmjs.com/misc/config#audit-level)

#### Global NPM package

```console
npm install -g audit-filter
```

#### Cargo

```console
cargo install audit-filter
```

### Usage

Note: all commands run from the project root

```console
$ audit-filter -h
audit-filter filters the output of "npm audit --json"

Usage:
  audit-filter [--json] [--audit=<->] [--nsp-config=<.nsprc>]
  audit-filter (-h | --help | --version)

Options:
  -h --help                       Show this screen.
  --version                       Show version.
  --json                          Output subset of JSON for the unfiltered advisories as an array.
  --audit=<audit>                 NPM Audit JSON file [default: -].
  --nsp-config=<config>           Default filter config [default: .nsprc].
$ cd audit-filter/example/
$ cat package.json
{
  "dependencies": {
    "moment": "2.19.2",
    "restify": "7.0.0"
  },
  "devDependencies": {
    "audit-filter": "0.3.0",
    "lodash": "^4.17.15"
  },
  "engines": {
    "node": ">=8",
    "npm": ">=6.4.1"
  },
  "scripts": {
    "lint:deps": "npm audit --json | audit-filter --nsp-config=.nsprc --audit=-"
  }
}
$ npm --version
6.9.0
$ npm audit
[90m                                                                                [39m
[90m [39m                      === npm audit security report ===                       [90m [39m
[90m                                                                                [39m
# Run  npm install moment@2.24.0  to resolve 1 vulnerability
[90m┌───────────────[39m[90m┬──────────────────────────────────────────────────────────────┐[39m
[90m│[39m Low           [90m│[39m Regular Expression Denial of Service                         [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Package       [90m│[39m moment                                                       [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Dependency of [90m│[39m moment                                                       [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Path          [90m│[39m moment                                                       [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m More info     [90m│[39m https://npmjs.com/advisories/532                             [90m│[39m
[90m└───────────────[39m[90m┴──────────────────────────────────────────────────────────────┘[39m


# Run  npm update moment --depth 3  to resolve 1 vulnerability
[90m┌───────────────[39m[90m┬──────────────────────────────────────────────────────────────┐[39m
[90m│[39m Low           [90m│[39m Regular Expression Denial of Service                         [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Package       [90m│[39m moment                                                       [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Dependency of [90m│[39m restify                                                      [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Path          [90m│[39m restify > bunyan > moment                                    [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m More info     [90m│[39m https://npmjs.com/advisories/532                             [90m│[39m
[90m└───────────────[39m[90m┴──────────────────────────────────────────────────────────────┘[39m


# Run  npm update lodash --depth 3  to resolve 2 vulnerabilities
[90m┌───────────────[39m[90m┬──────────────────────────────────────────────────────────────┐[39m
[90m│[39m High          [90m│[39m Prototype Pollution                                          [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Package       [90m│[39m lodash                                                       [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Dependency of [90m│[39m restify                                                      [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Path          [90m│[39m restify > lodash                                             [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m More info     [90m│[39m https://npmjs.com/advisories/1065                            [90m│[39m
[90m└───────────────[39m[90m┴──────────────────────────────────────────────────────────────┘[39m


[90m┌───────────────[39m[90m┬──────────────────────────────────────────────────────────────┐[39m
[90m│[39m High          [90m│[39m Prototype Pollution                                          [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Package       [90m│[39m lodash                                                       [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Dependency of [90m│[39m restify                                                      [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Path          [90m│[39m restify > restify-errors > lodash                            [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m More info     [90m│[39m https://npmjs.com/advisories/1065                            [90m│[39m
[90m└───────────────[39m[90m┴──────────────────────────────────────────────────────────────┘[39m


found 4 vulnerabilities (2 low, 2 high) in 137 scanned packages
  run `npm audit fix` to fix 4 of them.
$ echo $?
1
$ cat .nsprc
{
  "exceptions": [
    "https://npmjs.com/advisories/532",
    "https://npmjs.com/advisories/577",
    "https://npmjs.com/advisories/782",
    "https://npmjs.com/advisories/1065"
   ]
}
$ npm audit --json | audit-filter
No advisories found after filtering.
$ echo $?
0
$ # Alternatively specify audit and config file paths (note: errors print to stderr)
$ cd .. && audit-filter --nsp-config example/.nsprc --audit tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json
Unfiltered advisories:
  https://npmjs.com/advisories/118
  https://npmjs.com/advisories/534
  https://npmjs.com/advisories/566
  https://npmjs.com/advisories/598
  https://npmjs.com/advisories/663
  https://npmjs.com/advisories/755
  https://npmjs.com/advisories/777
  https://npmjs.com/advisories/786
  https://npmjs.com/advisories/788
  https://npmjs.com/advisories/803
  https://npmjs.com/advisories/813
  https://npmjs.com/advisories/886
  https://npmjs.com/advisories/996
  https://npmjs.com/advisories/1012
  https://npmjs.com/advisories/1013
  https://npmjs.com/advisories/1071
$ echo $?
1
$ # use --json for JSON output
$ audit-filter --json --nsp-config example/.nsprc --audit tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json | head
[
  {
    "findings": [
      {
        "version": "2.0.10",
        "paths": [
          "istanbul-middleware>archiver>glob>minimatch"
        ],
        "dev": null,
        "optional": null,
```

### Fixing comments in .nsprc files

```console
$ cat tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc-comment.json
{
  // See https://github.com/mozilla-services/screenshots/issues/4397
  "exceptions": [
    "https://nodesecurity.io/advisories/566",
    "https://nodesecurity.io/advisories/577",
    "https://nodesecurity.io/advisories/598",
    "https://nodesecurity.io/advisories/663",
    "https://nodesecurity.io/advisories/664"
   ]
}
$ audit-filter --nsp-config tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc-comment.json --audit tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-npm-6.4.1-audit.json
Error parsing nsp config JSON: key must be a string at line 2 column 3
$ echo $?
2
$ cat tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc-comment.json | sed "s|// .*||g" | python -m json.tool
{
    "exceptions": [
        "https://nodesecurity.io/advisories/566",
        "https://nodesecurity.io/advisories/577",
        "https://nodesecurity.io/advisories/598",
        "https://nodesecurity.io/advisories/663",
        "https://nodesecurity.io/advisories/664"
    ]
}
$ # alternatively convert comments into valid JSON e.g.
{
  "comment": "See https://github.com/mozilla-services/screenshots/issues/4397",
  "exceptions": [
    "https://nodesecurity.io/advisories/566",
    "https://nodesecurity.io/advisories/577",
    "https://nodesecurity.io/advisories/598",
    "https://nodesecurity.io/advisories/663",
    "https://nodesecurity.io/advisories/664"
   ]
}
```

### Exit Codes

* 0 - No advisories or all advisories acked from filters
* 1 - New failures one or more unacked advisory. Rerun `npm audit` to see the errors.
* 2 - Error finding or parsing config files or audit JSON.

### Other errors

NB: error messages will differ for audit-filter installed with NPM

Enumerated here for completeness. These all exit with code 2.

#### Error opening audit file

```console
$ audit-filter --nsp-config tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json --audit no-file
Error opening audit JSON no-file: No such file or directory (os error 2)
```

#### Error parsing audit from stdin

```console
$ echo "this is not JSON" | audit-filter --nsp-config tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json --audit -
Error parsing audit JSON from stdin: expected ident at line 1 column 2
```

#### Error parsing audit from file

```console
$ echo "this is not JSON" > not_json.txt
$ audit-filter --nsp-config tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json --audit not_json.txt
Error parsing audit JSON: expected ident at line 1 column 2
```

#### Error opening nsp config file

```console
$ audit-filter --nsp-config no-file --audit tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-npm-6.4.1-audit.json
Error opening nsp config JSON no-file: No such file or directory (os error 2)
```

#### Error parsing nsp config from stdin

```console
$ echo "this is not JSON" | audit-filter --nsp-config - --audit tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-npm-6.4.1-audit.json
Error parsing nsp config JSON from stdin: expected ident at line 1 column 2
```

#### Error parsing nsp config from file

```console
$ echo "this is not JSON" > not_json.txt
$ audit-filter --nsp-config not_json.txt --audit tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-npm-6.4.1-audit.json
Error parsing nsp config JSON: expected ident at line 1 column 2
```

### Building

To build a static executable:

```console
$ rustup target add x86_64-unknown-linux-musl
...
$ cargo build --release --target x86_64-unknown-linux-musl
...
$ ls -lh ./target/x86_64-unknown-linux-musl/release/audit-filter
-rwxrwxr-x 2 gguthe gguthe 7.0M Sep 20 13:09 ./target/x86_64-unknown-linux-musl/release/audit-filter
$ ldd ./target/x86_64-unknown-linux-musl/release/audit-filter
        not a dynamic executable
```

### Contributors

* @agwells
