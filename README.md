### audit-filter

[![Build Status](https://travis-ci.org/mozilla-services/audit-filter.svg?branch=master)](https://travis-ci.org/mozilla-services/audit-filter)

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

Using the stripped static executables (~800KB):

1. add [`example/bin/filtered_npm_audit.sh`](/example/bin/filtered_npm_audit.sh) to your npm project

1. add a script command e.g.

```json
{
  "dependencies": {
    ...
  },
  ...
  "scripts": {
    "lint:deps": "bin/filtered_npm_audit.sh"
	...
  }
}
```

1. test the script command with `CI=1 npm run-script lint:deps` and enable it in CI.

Using cargo:

```console
cargo install audit-filter
```

### Usage

Note: all commands from the project root

```console
$ audit-filter -h
audit-filter filters the output of "npm audit --json"

Usage:
  audit-filter [--audit=<->] [--nsp-config=<.nsprc>]
  audit-filter (-h | --help | --version)

Options:
  -h --help                       Show this screen.
  --version                       Show version.
  --audit=<audit>                 NPM Audit JSON file [default: -].
  --nsp-config=<config>           Default filter config [default: .nsprc].
$ cd audit-filter/example/
$ cat package.json
{
  "dependencies": {
    "moment": "2.19.2"
  }
}
$ npm audit

                       === npm audit security report ===

# Run  npm install moment@2.22.2  to resolve 1 vulnerability
┌───────────────┬──────────────────────────────────────────────────────────────┐
│ Low           │ Regular Expression Denial of Service                         │
├───────────────┼──────────────────────────────────────────────────────────────┤
│ Package       │ moment                                                       │
├───────────────┼──────────────────────────────────────────────────────────────┤
│ Dependency of │ moment                                                       │
├───────────────┼──────────────────────────────────────────────────────────────┤
│ Path          │ moment                                                       │
├───────────────┼──────────────────────────────────────────────────────────────┤
│ More info     │ https://npmjs.com/advisories/532                             │
└───────────────┴──────────────────────────────────────────────────────────────┘


found 1 low severity vulnerability in 1 scanned package
  run `npm audit fix` to fix 1 of them.
$ echo $?
1
$ cat .nsprc
{
  "exceptions": [
    "https://npmjs.com/advisories/532"
   ]
}
$ npm audit --json | audit-filter
No advisories found after filtering.
$ echo $?
0
$ # Alternatively specify audit and config file paths (note: errors print to stderr)
$ audit-filter --nsp-config tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json --audit tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-npm-6.4.1-audit.json
Unfiltered advisories:
  https://nodesecurity.io/advisories/118
  https://nodesecurity.io/advisories/534
  https://nodesecurity.io/advisories/681
$ echo $?
1
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
