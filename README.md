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
    "lodash": "^4.0.0"
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


# Run  npm update lodash --depth 3  to resolve 5 vulnerabilities
[90m┌───────────────[39m[90m┬──────────────────────────────────────────────────────────────┐[39m
[90m│[39m Low           [90m│[39m Prototype Pollution                                          [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Package       [90m│[39m lodash                                                       [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Dependency of [90m│[39m lodash [dev]                                                 [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Path          [90m│[39m lodash                                                       [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m More info     [90m│[39m https://npmjs.com/advisories/577                             [90m│[39m
[90m└───────────────[39m[90m┴──────────────────────────────────────────────────────────────┘[39m


[90m┌───────────────[39m[90m┬──────────────────────────────────────────────────────────────┐[39m
[90m│[39m High          [90m│[39m Prototype Pollution                                          [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Package       [90m│[39m lodash                                                       [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Dependency of [90m│[39m lodash [dev]                                                 [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Path          [90m│[39m lodash                                                       [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m More info     [90m│[39m https://npmjs.com/advisories/782                             [90m│[39m
[90m└───────────────[39m[90m┴──────────────────────────────────────────────────────────────┘[39m


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


[90m┌───────────────[39m[90m┬──────────────────────────────────────────────────────────────┐[39m
[90m│[39m High          [90m│[39m Prototype Pollution                                          [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Package       [90m│[39m lodash                                                       [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Dependency of [90m│[39m lodash [dev]                                                 [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m Path          [90m│[39m lodash                                                       [90m│[39m
[90m├───────────────[39m[90m┼──────────────────────────────────────────────────────────────┤[39m
[90m│[39m More info     [90m│[39m https://npmjs.com/advisories/1065                            [90m│[39m
[90m└───────────────[39m[90m┴──────────────────────────────────────────────────────────────┘[39m


found 7 vulnerabilities (3 low, 4 high) in 137 scanned packages
  run `npm audit fix` to fix 7 of them.
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
$ cd .. && audit-filter --nsp-config tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json --audit tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json
Unfiltered advisories:
  https://npmjs.com/advisories/118
  https://npmjs.com/advisories/534
  https://npmjs.com/advisories/566
  https://npmjs.com/advisories/577
  https://npmjs.com/advisories/598
  https://npmjs.com/advisories/663
  https://npmjs.com/advisories/755
  https://npmjs.com/advisories/777
  https://npmjs.com/advisories/782
  https://npmjs.com/advisories/786
  https://npmjs.com/advisories/788
  https://npmjs.com/advisories/803
  https://npmjs.com/advisories/813
  https://npmjs.com/advisories/886
  https://npmjs.com/advisories/996
  https://npmjs.com/advisories/1012
  https://npmjs.com/advisories/1013
  https://npmjs.com/advisories/1065
  https://npmjs.com/advisories/1071
$ echo $?
1
$ # use --json for JSON output
$ audit-filter --json --nsp-config tests/fixtures/screenshots-0191b17d3bac5de51efa7acbaa0d52bb26c91573-nsprc.json --audit tests/fixtures/screenshots-e78ee92b9a76ed6796cbdf0a9f643e00efc8b8b1-npm-6.9.0-audit.json
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
        "bundled": null
      }
    ],
    "id": 118,
    "title": "Regular Expression Denial of Service",
    "module_name": "minimatch",
    "overview": "Affected versions of `minimatch` are vulnerable to regular expression denial of service attacks when user input is passed into the `pattern` argument of `minimatch(path, pattern)`.\n\n\n## Proof of Concept\n```\nvar minimatch = require(“minimatch”);\n\n// utility function for generating long strings\nvar genstr = function (len, chr) {\n  var result = “”;\n  for (i=0; i<=len; i++) {\n    result = result + chr;\n  }\n  return result;\n}\n\nvar exploit = “[!” + genstr(1000000, “\\\\”) + “A”;\n\n// minimatch exploit.\nconsole.log(“starting minimatch”);\nminimatch(“foo”, exploit);\nconsole.log(“finishing minimatch”);\n```",
    "recommendation": "Update to version 3.0.2 or later.",
    "severity": "high",
    "url": "https://npmjs.com/advisories/118"
  },
  {
    "findings": [
      {
        "version": "2.2.0",
        "paths": [
          "istanbul-middleware>body-parser>debug"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 534,
    "title": "Regular Expression Denial of Service",
    "module_name": "debug",
    "overview": "Affected versions of `debug` are vulnerable to regular expression denial of service when untrusted user input is passed into the `o` formatter. \n\nAs it takes 50,000 characters to block the event loop for 2 seconds, this issue is a low severity issue.",
    "recommendation": "Version 2.x.x: Update to version 2.6.9 or later.\nVersion 3.x.x: Update to version 3.1.0 or later.\n",
    "severity": "low",
    "url": "https://npmjs.com/advisories/534"
  },
  {
    "findings": [
      {
        "version": "2.16.3",
        "paths": [
          "jpm>sign-addon>jsonwebtoken>joi>hoek"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "2.16.3",
        "paths": [
          "jpm>sign-addon>jsonwebtoken>joi>topo>hoek"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "2.16.3",
        "paths": [
          "jpm>sign-addon>request>hawk>boom>hoek",
          "jpm>sign-addon>request>hawk>cryptiles>boom>hoek",
          "jpm>sign-addon>request>hawk>hoek",
          "jpm>sign-addon>request>hawk>sntp>hoek"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 566,
    "title": "Prototype Pollution",
    "module_name": "hoek",
    "overview": "Versions of `hoek` prior to 4.2.1 and 5.0.3 are vulnerable to prototype pollution.\n\nThe `merge` function, and the `applyToDefaults` and `applyToDefaultsWithShallow` functions which leverage `merge` behind the scenes, are vulnerable to a prototype pollution attack when provided an _unvalidated_ payload created from a JSON string containing the `__proto__` property.\n\nThis can be demonstrated like so:\n\n```javascript\nvar Hoek = require('hoek');\nvar malicious_payload = '{\"__proto__\":{\"oops\":\"It works !\"}}';\n\nvar a = {};\nconsole.log(\"Before : \" + a.oops);\nHoek.merge({}, JSON.parse(malicious_payload));\nconsole.log(\"After : \" + a.oops);\n```\n\nThis type of attack can be used to overwrite existing properties causing a potential denial of service.",
    "recommendation": "Update to version 4.2.1, 5.0.3 or later.",
    "severity": "moderate",
    "url": "https://npmjs.com/advisories/566"
  },
  {
    "findings": [
      {
        "version": "4.11.1",
        "paths": [
          "jpm>firefox-profile>archiver>archiver-utils>lodash",
          "jpm>firefox-profile>archiver>zip-stream>archiver-utils>lodash",
          "web-ext>firefox-profile>archiver>zip-stream>archiver-utils>lodash",
          "web-ext>firefox-profile>archiver>archiver-utils>lodash",
          "jpm>firefox-profile>archiver>lodash",
          "jpm>firefox-profile>archiver>zip-stream>lodash",
          "web-ext>firefox-profile>archiver>zip-stream>lodash",
          "jpm>lodash",
          "jpm>xml2js>xmlbuilder>lodash",
          "addons-linter>eslint>inquirer>lodash",
          "web-ext>addons-linter>eslint>inquirer>lodash",
          "addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>inquirer>lodash",
          "web-ext>addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>inquirer>lodash",
          "addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>lodash",
          "web-ext>addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>lodash",
          "addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>table>lodash",
          "web-ext>addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>table>lodash",
          "node-sass>sass-graph>lodash",
          "sass-lint>eslint>inquirer>lodash",
          "sass-lint>eslint>lodash",
          "sass-lint>eslint>table>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.12.0",
        "paths": [
          "jpm>firefox-profile>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "3.10.1",
        "paths": [
          "jpm>fx-runner>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "3.2.0",
        "paths": [
          "istanbul-middleware>archiver>lodash",
          "istanbul-middleware>archiver>zip-stream>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 577,
    "title": "Prototype Pollution",
    "module_name": "lodash",
    "overview": "Versions of `lodash` before 4.17.5 are vulnerable to prototype pollution. \n\nThe vulnerable functions are 'defaultsDeep', 'merge', and 'mergeWith' which allow a malicious user to modify the prototype of `Object` via `__proto__` causing the addition or modification of an existing property that will exist on all objects.\n\n",
    "recommendation": "Update to version 4.17.5 or later.",
    "severity": "low",
    "url": "https://npmjs.com/advisories/577"
  },
  {
    "findings": [
      {
        "version": "0.4.3",
        "paths": [
          "jpm>sign-addon>request>tunnel-agent"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 598,
    "title": "Memory Exposure",
    "module_name": "tunnel-agent",
    "overview": "Versions of `tunnel-agent` before 0.6.0 are vulnerable to memory exposure.\n\nThis is exploitable if user supplied input is provided to the auth value and is a number.\n\nProof-of-concept:\n```js\nrequire('request')({\n  method: 'GET',\n  uri: 'http://www.example.com',\n  tunnel: true,\n  proxy:{\n    protocol: 'http:',\n    host:'127.0.0.1',\n    port:8080,\n    auth:USERSUPPLIEDINPUT // number\n  }\n});\n```",
    "recommendation": "Update to version 0.6.0 or later.",
    "severity": "moderate",
    "url": "https://npmjs.com/advisories/598"
  },
  {
    "findings": [
      {
        "version": "0.0.5",
        "paths": [
          "jpm>open"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 663,
    "title": "Command Injection",
    "module_name": "open",
    "overview": "Versions of `open` before 6.0.0 are vulnerable to command injection when unsanitized user input is passed in.\n\nThe package does come with the following warning in the readme:\n\n```\nThe same care should be taken when calling open as if you were calling child_process.exec directly. If it is an executable it will run in a new shell.\n```",
    "recommendation": "`open` is now the deprecated `opn` package. Upgrading to the latest version is likely have unwanted effects since it now has a very different API but will prevent this vulnerability.",
    "severity": "critical",
    "url": "https://npmjs.com/advisories/663"
  },
  {
    "findings": [
      {
        "version": "4.0.12",
        "paths": [
          "istanbul-middleware>istanbul>handlebars"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 755,
    "title": "Prototype Pollution",
    "module_name": "handlebars",
    "overview": "Versions of `handlebars` prior to 4.0.14 are vulnerable to Prototype Pollution. Templates may alter an Objects' prototype, thus allowing an attacker to execute arbitrary code on the server.",
    "recommendation": "For handlebars 4.1.x upgrade to 4.1.2 or later.\nFor handlebars 4.0.x upgrade to 4.0.14 or later.",
    "severity": "high",
    "url": "https://npmjs.com/advisories/755"
  },
  {
    "findings": [
      {
        "version": "0.3.0",
        "paths": [
          "jpm>decompress-zip"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 777,
    "title": "Arbitrary File Overwrite",
    "module_name": "decompress-zip",
    "overview": "Vulnerable versions of `decompress-zip` are affected by the Zip-Slip vulnerability, an arbitrary file write vulnerability. The vulnerability occurs because `decompress-zip` does not verify that extracted files do not resolve to targets outside of the extraction root directory.\n",
    "recommendation": "For `decompress-zip` 0.2.x upgrade to 0.2.2 or later.\nFor `decompress-zip` 0.3.x upgrade to 0.3.2 or later.",
    "severity": "high",
    "url": "https://npmjs.com/advisories/777"
  },
  {
    "findings": [
      {
        "version": "4.11.1",
        "paths": [
          "jpm>firefox-profile>archiver>archiver-utils>lodash",
          "jpm>firefox-profile>archiver>zip-stream>archiver-utils>lodash",
          "web-ext>firefox-profile>archiver>zip-stream>archiver-utils>lodash",
          "web-ext>firefox-profile>archiver>archiver-utils>lodash",
          "jpm>firefox-profile>archiver>lodash",
          "jpm>firefox-profile>archiver>zip-stream>lodash",
          "web-ext>firefox-profile>archiver>zip-stream>lodash",
          "jpm>lodash",
          "jpm>xml2js>xmlbuilder>lodash",
          "addons-linter>eslint>inquirer>lodash",
          "web-ext>addons-linter>eslint>inquirer>lodash",
          "addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>inquirer>lodash",
          "web-ext>addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>inquirer>lodash",
          "addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>lodash",
          "web-ext>addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>lodash",
          "addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>table>lodash",
          "web-ext>addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>table>lodash",
          "node-sass>sass-graph>lodash",
          "sass-lint>eslint>inquirer>lodash",
          "sass-lint>eslint>lodash",
          "sass-lint>eslint>table>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.12.0",
        "paths": [
          "jpm>firefox-profile>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "3.10.1",
        "paths": [
          "jpm>fx-runner>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.10",
        "paths": [
          "fx-runner>lodash",
          "web-ext>fx-runner>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "3.2.0",
        "paths": [
          "istanbul-middleware>archiver>lodash",
          "istanbul-middleware>archiver>zip-stream>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.10",
        "paths": [
          "web-ext>addons-linter>snyk>snyk-nodejs-lockfile-parser>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 782,
    "title": "Prototype Pollution",
    "module_name": "lodash",
    "overview": "Versions of `lodash` before 4.17.5 are vulnerable to prototype pollution. \n\nThe vulnerable functions are 'defaultsDeep', 'merge', and 'mergeWith' which allow a malicious user to modify the prototype of `Object` via `{constructor: {prototype: {...}}}` causing the addition or modification of an existing property that will exist on all objects.\n\n",
    "recommendation": "Update to version 4.17.11 or later.",
    "severity": "high",
    "url": "https://npmjs.com/advisories/782"
  },
  {
    "findings": [
      {
        "version": "1.8.5",
        "paths": [
          "babel-cli>chokidar>anymatch>micromatch>braces"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 786,
    "title": "Regular Expression Denial of Service",
    "module_name": "braces",
    "overview": "Versions of `braces` prior to 2.3.1 are vulnerable to Regular Expression Denial of Service (ReDoS). Untrusted input may cause catastrophic backtracking while matching regular expressions. This can cause the application to be unresponsive leading to Denial of Service.",
    "recommendation": "Upgrade to version 2.3.1 or higher.",
    "severity": "low",
    "url": "https://npmjs.com/advisories/786"
  },
  {
    "findings": [
      {
        "version": "3.12.0",
        "paths": [
          "addons-linter>eslint>js-yaml",
          "addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>js-yaml",
          "web-ext>addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>js-yaml",
          "eslint>js-yaml",
          "istanbul-middleware>istanbul>js-yaml",
          "postcss-cli>postcss-load-config>cosmiconfig>js-yaml",
          "sass-lint>eslint>js-yaml",
          "sass-lint>front-matter>js-yaml",
          "sass-lint>js-yaml",
          "svgo>js-yaml",
          "web-ext>addons-linter>eslint>js-yaml",
          "web-ext>addons-linter>snyk>snyk-policy>js-yaml"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 788,
    "title": "Denial of Service",
    "module_name": "js-yaml",
    "overview": "Versions of `js-yaml` prior to 3.13.0 are vulnerable to Denial of Service. By parsing a carefully-crafted YAML file, the node process stalls and may exhaust system resources leading to a Denial of Service.",
    "recommendation": "Upgrade to version 3.13.0.",
    "severity": "moderate",
    "url": "https://npmjs.com/advisories/788"
  },
  {
    "findings": [
      {
        "version": "4.4.1",
        "paths": [
          "nodemon>chokidar>fsevents>node-pre-gyp>tar",
          "postcss-cli>chokidar>fsevents>node-pre-gyp>tar",
          "web-ext>watchpack>chokidar>fsevents>node-pre-gyp>tar",
          "babel-cli>chokidar>fsevents>node-pre-gyp>tar"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.0.2",
        "paths": [
          "geckodriver>tar"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "2.2.1",
        "paths": [
          "node-sass>node-gyp>tar"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 803,
    "title": "Arbitrary File Overwrite",
    "module_name": "tar",
    "overview": "Versions of `tar` prior to 4.4.2 for 4.x and 2.2.2 for 2.x are vulnerable to Arbitrary File Overwrite. Extracting tarballs containing a hardlink to a file that already exists in the system, and a file that matches the hardlink will overwrite the system's file with the contents of the extracted file.",
    "recommendation": "For tar 4.x, upgrade to version 4.4.2 or later.\nFor tar 2.x, upgrade to version 2.2.2 or later.",
    "severity": "high",
    "url": "https://npmjs.com/advisories/803"
  },
  {
    "findings": [
      {
        "version": "3.12.0",
        "paths": [
          "addons-linter>eslint>js-yaml",
          "addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>js-yaml",
          "web-ext>addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>js-yaml",
          "eslint>js-yaml",
          "istanbul-middleware>istanbul>js-yaml",
          "postcss-cli>postcss-load-config>cosmiconfig>js-yaml",
          "sass-lint>eslint>js-yaml",
          "sass-lint>front-matter>js-yaml",
          "sass-lint>js-yaml",
          "svgo>js-yaml",
          "web-ext>addons-linter>eslint>js-yaml",
          "web-ext>addons-linter>snyk>snyk-policy>js-yaml"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 813,
    "title": "Code Injection",
    "module_name": "js-yaml",
    "overview": "Versions of `js-yaml` prior to 3.13.1 are vulnerable to Code Injection. The `load()` function may execute arbitrary code injected through a malicious YAML file. Objects that have `toString` as key, JavaScript code as value and are used as explicit mapping keys allow attackers to execute the supplied code through the `load()` function. The `safeLoad()` function is unaffected.\n\nAn example payload is \n`{ toString: !<tag:yaml.org,2002:js/function> 'function (){return Date.now()}' } : 1` \nwhich returns the object \n{\n  \"1553107949161\": 1\n}",
    "recommendation": "Upgrade to version 3.13.1.",
    "severity": "high",
    "url": "https://npmjs.com/advisories/813"
  },
  {
    "findings": [
      {
        "version": "1.0.11",
        "paths": [
          "node-sass>node-gyp>fstream",
          "node-sass>node-gyp>tar>fstream"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 886,
    "title": "Arbitrary File Overwrite",
    "module_name": "fstream",
    "overview": "Versions of `fstream` prior to 1.0.12 are vulnerable to Arbitrary File Overwrite. Extracting tarballs containing a hardlink to a file that already exists in the system and a file that matches the hardlink will overwrite the system's file with the contents of the extracted file. The `fstream.DirWriter()` function is vulnerable.",
    "recommendation": "Upgrade to version 1.0.12 or later.",
    "severity": "high",
    "url": "https://npmjs.com/advisories/886"
  },
  {
    "findings": [
      {
        "version": "1.3.0",
        "paths": [
          "uglifyify>extend"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 996,
    "title": "Prototype Pollution",
    "module_name": "extend",
    "overview": "Versions of `extend` prior to 3.0.2 (for 3.x) and 2.0.2 (for 2.x) are vulnerable to Prototype Pollution. The `extend()` function allows attackers to modify the prototype of Object causing the addition or modification of an existing property that will exist on all objects.\n\n",
    "recommendation": "If you're using `extend` 3.x upgrade to 3.0.2 or later.\nIf you're using `extend` 2.x upgrade to 2.0.2 or later.",
    "severity": "moderate",
    "url": "https://npmjs.com/advisories/996"
  },
  {
    "findings": [
      {
        "version": "2.0.0",
        "paths": [
          "nodemon>chokidar>anymatch>micromatch>braces>snapdragon>base>cache-base>set-value",
          "postcss-cli>chokidar>anymatch>micromatch>braces>snapdragon>base>cache-base>set-value",
          "web-ext>watchpack>chokidar>anymatch>micromatch>braces>snapdragon>base>cache-base>set-value",
          "nodemon>chokidar>readdirp>micromatch>braces>snapdragon>base>cache-base>set-value",
          "postcss-cli>chokidar>readdirp>micromatch>braces>snapdragon>base>cache-base>set-value",
          "web-ext>watchpack>chokidar>readdirp>micromatch>braces>snapdragon>base>cache-base>set-value",
          "babel-cli>chokidar>readdirp>micromatch>braces>snapdragon>base>cache-base>set-value",
          "postcss-cli>globby>fast-glob>micromatch>braces>snapdragon>base>cache-base>set-value",
          "nodemon>chokidar>braces>snapdragon>base>cache-base>set-value",
          "postcss-cli>chokidar>braces>snapdragon>base>cache-base>set-value",
          "web-ext>watchpack>chokidar>braces>snapdragon>base>cache-base>set-value",
          "nodemon>chokidar>anymatch>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>set-value",
          "postcss-cli>chokidar>anymatch>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>set-value",
          "web-ext>watchpack>chokidar>anymatch>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>set-value",
          "nodemon>chokidar>readdirp>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>set-value",
          "postcss-cli>chokidar>readdirp>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>set-value",
          "web-ext>watchpack>chokidar>readdirp>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>set-value",
          "babel-cli>chokidar>readdirp>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>set-value",
          "postcss-cli>globby>fast-glob>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>set-value",
          "nodemon>chokidar>anymatch>micromatch>extglob>snapdragon>base>cache-base>set-value",
          "postcss-cli>chokidar>anymatch>micromatch>extglob>snapdragon>base>cache-base>set-value",
          "web-ext>watchpack>chokidar>anymatch>micromatch>extglob>snapdragon>base>cache-base>set-value",
          "nodemon>chokidar>readdirp>micromatch>extglob>snapdragon>base>cache-base>set-value",
          "postcss-cli>chokidar>readdirp>micromatch>extglob>snapdragon>base>cache-base>set-value",
          "web-ext>watchpack>chokidar>readdirp>micromatch>extglob>snapdragon>base>cache-base>set-value",
          "babel-cli>chokidar>readdirp>micromatch>extglob>snapdragon>base>cache-base>set-value",
          "postcss-cli>globby>fast-glob>micromatch>extglob>snapdragon>base>cache-base>set-value",
          "nodemon>chokidar>anymatch>micromatch>nanomatch>snapdragon>base>cache-base>set-value",
          "postcss-cli>chokidar>anymatch>micromatch>nanomatch>snapdragon>base>cache-base>set-value",
          "web-ext>watchpack>chokidar>anymatch>micromatch>nanomatch>snapdragon>base>cache-base>set-value",
          "nodemon>chokidar>readdirp>micromatch>nanomatch>snapdragon>base>cache-base>set-value",
          "postcss-cli>chokidar>readdirp>micromatch>nanomatch>snapdragon>base>cache-base>set-value",
          "web-ext>watchpack>chokidar>readdirp>micromatch>nanomatch>snapdragon>base>cache-base>set-value",
          "babel-cli>chokidar>readdirp>micromatch>nanomatch>snapdragon>base>cache-base>set-value",
          "postcss-cli>globby>fast-glob>micromatch>nanomatch>snapdragon>base>cache-base>set-value",
          "nodemon>chokidar>anymatch>micromatch>snapdragon>base>cache-base>set-value",
          "postcss-cli>chokidar>anymatch>micromatch>snapdragon>base>cache-base>set-value",
          "web-ext>watchpack>chokidar>anymatch>micromatch>snapdragon>base>cache-base>set-value",
          "nodemon>chokidar>readdirp>micromatch>snapdragon>base>cache-base>set-value",
          "postcss-cli>chokidar>readdirp>micromatch>snapdragon>base>cache-base>set-value",
          "web-ext>watchpack>chokidar>readdirp>micromatch>snapdragon>base>cache-base>set-value",
          "babel-cli>chokidar>readdirp>micromatch>snapdragon>base>cache-base>set-value",
          "postcss-cli>globby>fast-glob>micromatch>snapdragon>base>cache-base>set-value"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "0.4.3",
        "paths": [
          "nodemon>chokidar>anymatch>micromatch>braces>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>chokidar>anymatch>micromatch>braces>snapdragon>base>cache-base>union-value>set-value",
          "web-ext>watchpack>chokidar>anymatch>micromatch>braces>snapdragon>base>cache-base>union-value>set-value",
          "nodemon>chokidar>readdirp>micromatch>braces>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>chokidar>readdirp>micromatch>braces>snapdragon>base>cache-base>union-value>set-value",
          "web-ext>watchpack>chokidar>readdirp>micromatch>braces>snapdragon>base>cache-base>union-value>set-value",
          "babel-cli>chokidar>readdirp>micromatch>braces>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>globby>fast-glob>micromatch>braces>snapdragon>base>cache-base>union-value>set-value",
          "nodemon>chokidar>braces>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>chokidar>braces>snapdragon>base>cache-base>union-value>set-value",
          "web-ext>watchpack>chokidar>braces>snapdragon>base>cache-base>union-value>set-value",
          "nodemon>chokidar>anymatch>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>chokidar>anymatch>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>union-value>set-value",
          "web-ext>watchpack>chokidar>anymatch>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>union-value>set-value",
          "nodemon>chokidar>readdirp>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>chokidar>readdirp>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>union-value>set-value",
          "web-ext>watchpack>chokidar>readdirp>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>union-value>set-value",
          "babel-cli>chokidar>readdirp>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>globby>fast-glob>micromatch>extglob>expand-brackets>snapdragon>base>cache-base>union-value>set-value",
          "nodemon>chokidar>anymatch>micromatch>extglob>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>chokidar>anymatch>micromatch>extglob>snapdragon>base>cache-base>union-value>set-value",
          "web-ext>watchpack>chokidar>anymatch>micromatch>extglob>snapdragon>base>cache-base>union-value>set-value",
          "nodemon>chokidar>readdirp>micromatch>extglob>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>chokidar>readdirp>micromatch>extglob>snapdragon>base>cache-base>union-value>set-value",
          "web-ext>watchpack>chokidar>readdirp>micromatch>extglob>snapdragon>base>cache-base>union-value>set-value",
          "babel-cli>chokidar>readdirp>micromatch>extglob>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>globby>fast-glob>micromatch>extglob>snapdragon>base>cache-base>union-value>set-value",
          "nodemon>chokidar>anymatch>micromatch>nanomatch>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>chokidar>anymatch>micromatch>nanomatch>snapdragon>base>cache-base>union-value>set-value",
          "web-ext>watchpack>chokidar>anymatch>micromatch>nanomatch>snapdragon>base>cache-base>union-value>set-value",
          "nodemon>chokidar>readdirp>micromatch>nanomatch>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>chokidar>readdirp>micromatch>nanomatch>snapdragon>base>cache-base>union-value>set-value",
          "web-ext>watchpack>chokidar>readdirp>micromatch>nanomatch>snapdragon>base>cache-base>union-value>set-value",
          "babel-cli>chokidar>readdirp>micromatch>nanomatch>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>globby>fast-glob>micromatch>nanomatch>snapdragon>base>cache-base>union-value>set-value",
          "nodemon>chokidar>anymatch>micromatch>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>chokidar>anymatch>micromatch>snapdragon>base>cache-base>union-value>set-value",
          "web-ext>watchpack>chokidar>anymatch>micromatch>snapdragon>base>cache-base>union-value>set-value",
          "nodemon>chokidar>readdirp>micromatch>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>chokidar>readdirp>micromatch>snapdragon>base>cache-base>union-value>set-value",
          "web-ext>watchpack>chokidar>readdirp>micromatch>snapdragon>base>cache-base>union-value>set-value",
          "babel-cli>chokidar>readdirp>micromatch>snapdragon>base>cache-base>union-value>set-value",
          "postcss-cli>globby>fast-glob>micromatch>snapdragon>base>cache-base>union-value>set-value"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 1012,
    "title": "Prototype Pollution",
    "module_name": "set-value",
    "overview": "Versions of `set-value` prior to 3.0.1 or 2.0.1 are vulnerable to Prototype Pollution. The `set` function fails to validate which Object properties it updates. This allows attackers to modify the prototype of Object, causing the addition or modification of an existing property on all objects.\n\n",
    "recommendation": "If you are using `set-value` 3.x, upgrade to version 3.0.1 or later.\nIf you are using `set-value` 2.x, upgrade to version 2.0.1 or later.\n",
    "severity": "high",
    "url": "https://npmjs.com/advisories/1012"
  },
  {
    "findings": [
      {
        "version": "1.3.1",
        "paths": [
          "nodemon>chokidar>anymatch>micromatch>braces>snapdragon>base>mixin-deep",
          "postcss-cli>chokidar>anymatch>micromatch>braces>snapdragon>base>mixin-deep",
          "web-ext>watchpack>chokidar>anymatch>micromatch>braces>snapdragon>base>mixin-deep",
          "nodemon>chokidar>readdirp>micromatch>braces>snapdragon>base>mixin-deep",
          "postcss-cli>chokidar>readdirp>micromatch>braces>snapdragon>base>mixin-deep",
          "web-ext>watchpack>chokidar>readdirp>micromatch>braces>snapdragon>base>mixin-deep",
          "babel-cli>chokidar>readdirp>micromatch>braces>snapdragon>base>mixin-deep",
          "postcss-cli>globby>fast-glob>micromatch>braces>snapdragon>base>mixin-deep",
          "nodemon>chokidar>braces>snapdragon>base>mixin-deep",
          "postcss-cli>chokidar>braces>snapdragon>base>mixin-deep",
          "web-ext>watchpack>chokidar>braces>snapdragon>base>mixin-deep",
          "nodemon>chokidar>anymatch>micromatch>extglob>expand-brackets>snapdragon>base>mixin-deep",
          "postcss-cli>chokidar>anymatch>micromatch>extglob>expand-brackets>snapdragon>base>mixin-deep",
          "web-ext>watchpack>chokidar>anymatch>micromatch>extglob>expand-brackets>snapdragon>base>mixin-deep",
          "nodemon>chokidar>readdirp>micromatch>extglob>expand-brackets>snapdragon>base>mixin-deep",
          "postcss-cli>chokidar>readdirp>micromatch>extglob>expand-brackets>snapdragon>base>mixin-deep",
          "web-ext>watchpack>chokidar>readdirp>micromatch>extglob>expand-brackets>snapdragon>base>mixin-deep",
          "babel-cli>chokidar>readdirp>micromatch>extglob>expand-brackets>snapdragon>base>mixin-deep",
          "postcss-cli>globby>fast-glob>micromatch>extglob>expand-brackets>snapdragon>base>mixin-deep",
          "nodemon>chokidar>anymatch>micromatch>extglob>snapdragon>base>mixin-deep",
          "postcss-cli>chokidar>anymatch>micromatch>extglob>snapdragon>base>mixin-deep",
          "web-ext>watchpack>chokidar>anymatch>micromatch>extglob>snapdragon>base>mixin-deep",
          "nodemon>chokidar>readdirp>micromatch>extglob>snapdragon>base>mixin-deep",
          "postcss-cli>chokidar>readdirp>micromatch>extglob>snapdragon>base>mixin-deep",
          "web-ext>watchpack>chokidar>readdirp>micromatch>extglob>snapdragon>base>mixin-deep",
          "babel-cli>chokidar>readdirp>micromatch>extglob>snapdragon>base>mixin-deep",
          "postcss-cli>globby>fast-glob>micromatch>extglob>snapdragon>base>mixin-deep",
          "nodemon>chokidar>anymatch>micromatch>nanomatch>snapdragon>base>mixin-deep",
          "postcss-cli>chokidar>anymatch>micromatch>nanomatch>snapdragon>base>mixin-deep",
          "web-ext>watchpack>chokidar>anymatch>micromatch>nanomatch>snapdragon>base>mixin-deep",
          "nodemon>chokidar>readdirp>micromatch>nanomatch>snapdragon>base>mixin-deep",
          "postcss-cli>chokidar>readdirp>micromatch>nanomatch>snapdragon>base>mixin-deep",
          "web-ext>watchpack>chokidar>readdirp>micromatch>nanomatch>snapdragon>base>mixin-deep",
          "babel-cli>chokidar>readdirp>micromatch>nanomatch>snapdragon>base>mixin-deep",
          "postcss-cli>globby>fast-glob>micromatch>nanomatch>snapdragon>base>mixin-deep",
          "nodemon>chokidar>anymatch>micromatch>snapdragon>base>mixin-deep",
          "postcss-cli>chokidar>anymatch>micromatch>snapdragon>base>mixin-deep",
          "web-ext>watchpack>chokidar>anymatch>micromatch>snapdragon>base>mixin-deep",
          "nodemon>chokidar>readdirp>micromatch>snapdragon>base>mixin-deep",
          "postcss-cli>chokidar>readdirp>micromatch>snapdragon>base>mixin-deep",
          "web-ext>watchpack>chokidar>readdirp>micromatch>snapdragon>base>mixin-deep",
          "babel-cli>chokidar>readdirp>micromatch>snapdragon>base>mixin-deep",
          "postcss-cli>globby>fast-glob>micromatch>snapdragon>base>mixin-deep"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 1013,
    "title": "Prototype Pollution",
    "module_name": "mixin-deep",
    "overview": "Versions of `mixin-deep` prior to 2.0.1 or 1.3.2 are vulnerable to Prototype Pollution. The `mixinDeep` function fails to validate which Object properties it updates. This allows attackers to modify the prototype of Object, causing the addition or modification of an existing property on all objects.\n\n",
    "recommendation": "If you are using `mixin-deep` 2.x, upgrade to version 2.0.1 or later.\nIf you are using `mixin-deep` 1.x, upgrade to version 1.3.2 or later.",
    "severity": "high",
    "url": "https://npmjs.com/advisories/1013"
  },
  {
    "findings": [
      {
        "version": "4.11.1",
        "paths": [
          "jpm>firefox-profile>archiver>archiver-utils>lodash",
          "jpm>firefox-profile>archiver>zip-stream>archiver-utils>lodash",
          "web-ext>firefox-profile>archiver>zip-stream>archiver-utils>lodash",
          "web-ext>firefox-profile>archiver>archiver-utils>lodash",
          "jpm>firefox-profile>archiver>lodash",
          "jpm>firefox-profile>archiver>zip-stream>lodash",
          "web-ext>firefox-profile>archiver>zip-stream>lodash",
          "jpm>lodash",
          "jpm>xml2js>xmlbuilder>lodash",
          "addons-linter>eslint>inquirer>lodash",
          "web-ext>addons-linter>eslint>inquirer>lodash",
          "addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>inquirer>lodash",
          "web-ext>addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>inquirer>lodash",
          "addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>lodash",
          "web-ext>addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>lodash",
          "addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>table>lodash",
          "web-ext>addons-linter>eslint-plugin-no-unsafe-innerhtml>eslint>table>lodash",
          "node-sass>sass-graph>lodash",
          "sass-lint>eslint>inquirer>lodash",
          "sass-lint>eslint>lodash",
          "sass-lint>eslint>table>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "jpm>firefox-profile>archiver>async>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.12.0",
        "paths": [
          "jpm>firefox-profile>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "3.10.1",
        "paths": [
          "jpm>fx-runner>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "jsdom>request-promise-native>request-promise-core>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "react-masonry-component>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "addons-linter>cheerio>lodash",
          "web-ext>addons-linter>cheerio>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "addons-linter>dispensary>async>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "addons-linter>eslint>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "addons-linter>eslint>table>lodash",
          "web-ext>addons-linter>eslint>table>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "babel-cli>babel-core>babel-generator>babel-types>lodash",
          "babel-cli>babel-register>babel-core>babel-generator>babel-types>lodash",
          "babel-cli>babel-core>babel-helpers>babel-template>babel-traverse>babel-types>lodash",
          "babel-cli>babel-register>babel-core>babel-helpers>babel-template>babel-traverse>babel-types>lodash",
          "babel-cli>babel-core>babel-template>babel-traverse>babel-types>lodash",
          "babel-cli>babel-register>babel-core>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-block-scoping>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-define-map>babel-helper-function-name>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-function-name>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-function-name>babel-helper-function-name>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-replace-supers>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-object-super>babel-helper-replace-supers>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-computed-properties>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-amd>babel-plugin-transform-es2015-modules-commonjs>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-plugin-transform-es2015-modules-amd>babel-plugin-transform-es2015-modules-commonjs>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-commonjs>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-amd>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-plugin-transform-es2015-modules-amd>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-systemjs>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-template>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-parameters>babel-template>babel-traverse>babel-types>lodash",
          "babel-cli>babel-core>babel-traverse>babel-types>lodash",
          "babel-cli>babel-register>babel-core>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-block-scoping>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-define-map>babel-helper-function-name>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-function-name>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-function-name>babel-helper-function-name>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-replace-supers>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-object-super>babel-helper-replace-supers>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-parameters>babel-helper-call-delegate>babel-traverse>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-parameters>babel-traverse>babel-types>lodash",
          "babel-cli>babel-core>babel-helpers>babel-template>babel-types>lodash",
          "babel-cli>babel-register>babel-core>babel-helpers>babel-template>babel-types>lodash",
          "babel-cli>babel-core>babel-template>babel-types>lodash",
          "babel-cli>babel-register>babel-core>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-block-scoping>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-define-map>babel-helper-function-name>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-function-name>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-function-name>babel-helper-function-name>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-replace-supers>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-object-super>babel-helper-replace-supers>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-computed-properties>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-amd>babel-plugin-transform-es2015-modules-commonjs>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-plugin-transform-es2015-modules-amd>babel-plugin-transform-es2015-modules-commonjs>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-commonjs>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-amd>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-plugin-transform-es2015-modules-amd>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-systemjs>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-template>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-parameters>babel-template>babel-types>lodash",
          "babel-cli>babel-core>babel-types>lodash",
          "babel-cli>babel-register>babel-core>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-block-scoping>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-define-map>babel-helper-function-name>babel-helper-get-function-arity>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-function-name>babel-helper-get-function-arity>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-function-name>babel-helper-function-name>babel-helper-get-function-arity>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-parameters>babel-helper-get-function-arity>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-define-map>babel-helper-function-name>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-function-name>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-function-name>babel-helper-function-name>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-define-map>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-optimise-call-expression>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-replace-supers>babel-helper-optimise-call-expression>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-object-super>babel-helper-replace-supers>babel-helper-optimise-call-expression>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-replace-supers>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-object-super>babel-helper-replace-supers>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-duplicate-keys>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-function-name>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-amd>babel-plugin-transform-es2015-modules-commonjs>babel-plugin-transform-strict-mode>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-plugin-transform-es2015-modules-amd>babel-plugin-transform-es2015-modules-commonjs>babel-plugin-transform-strict-mode>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-commonjs>babel-plugin-transform-strict-mode>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-amd>babel-plugin-transform-es2015-modules-commonjs>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-plugin-transform-es2015-modules-amd>babel-plugin-transform-es2015-modules-commonjs>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-commonjs>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-systemjs>babel-helper-hoist-variables>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-parameters>babel-helper-call-delegate>babel-helper-hoist-variables>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-parameters>babel-helper-call-delegate>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-parameters>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-shorthand-properties>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-sticky-regex>babel-helper-regex>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-unicode-regex>babel-helper-regex>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-sticky-regex>babel-types>lodash",
          "babel-preset-es2015>babel-plugin-transform-regenerator>regenerator-transform>babel-types>lodash",
          "babel-preset-react>babel-plugin-transform-react-jsx>babel-helper-builder-react-jsx>babel-types>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "babel-cli>babel-core>babel-generator>lodash",
          "babel-cli>babel-register>babel-core>babel-generator>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "babel-cli>babel-core>babel-helpers>babel-template>babel-traverse>lodash",
          "babel-cli>babel-register>babel-core>babel-helpers>babel-template>babel-traverse>lodash",
          "babel-cli>babel-core>babel-template>babel-traverse>lodash",
          "babel-cli>babel-register>babel-core>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-block-scoping>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-define-map>babel-helper-function-name>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-function-name>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-function-name>babel-helper-function-name>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-replace-supers>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-object-super>babel-helper-replace-supers>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-computed-properties>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-amd>babel-plugin-transform-es2015-modules-commonjs>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-plugin-transform-es2015-modules-amd>babel-plugin-transform-es2015-modules-commonjs>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-commonjs>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-amd>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-plugin-transform-es2015-modules-amd>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-systemjs>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-template>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-parameters>babel-template>babel-traverse>lodash",
          "babel-cli>babel-core>babel-traverse>lodash",
          "babel-cli>babel-register>babel-core>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-block-scoping>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-define-map>babel-helper-function-name>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-function-name>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-function-name>babel-helper-function-name>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-replace-supers>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-object-super>babel-helper-replace-supers>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-parameters>babel-helper-call-delegate>babel-traverse>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-parameters>babel-traverse>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "babel-cli>babel-core>babel-helpers>babel-template>lodash",
          "babel-cli>babel-register>babel-core>babel-helpers>babel-template>lodash",
          "babel-cli>babel-core>babel-template>lodash",
          "babel-cli>babel-register>babel-core>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-block-scoping>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-define-map>babel-helper-function-name>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-function-name>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-function-name>babel-helper-function-name>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-replace-supers>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-object-super>babel-helper-replace-supers>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-computed-properties>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-amd>babel-plugin-transform-es2015-modules-commonjs>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-plugin-transform-es2015-modules-amd>babel-plugin-transform-es2015-modules-commonjs>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-commonjs>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-amd>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-plugin-transform-es2015-modules-amd>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-systemjs>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-modules-umd>babel-template>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-parameters>babel-template>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "babel-cli>babel-core>babel-register>lodash",
          "babel-cli>babel-register>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "babel-cli>babel-core>lodash",
          "babel-cli>babel-register>babel-core>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "babel-cli>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "babel-preset-es2015>babel-plugin-transform-es2015-block-scoping>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "babel-preset-es2015>babel-plugin-transform-es2015-classes>babel-helper-define-map>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "babel-preset-es2015>babel-plugin-transform-es2015-sticky-regex>babel-helper-regex>lodash",
          "babel-preset-es2015>babel-plugin-transform-es2015-unicode-regex>babel-helper-regex>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "eslint>inquirer>lodash",
          "eslint>lodash",
          "eslint>table>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.10",
        "paths": [
          "fx-runner>lodash",
          "web-ext>fx-runner>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "3.2.0",
        "paths": [
          "istanbul-middleware>archiver>lodash",
          "istanbul-middleware>archiver>zip-stream>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "istanbul-middleware>istanbul>handlebars>async>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "node-sass>gaze>globule>lodash",
          "sass-lint>globule>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "postcss-cli>postcss-reporter>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "web-ext>addons-linter>@babel/register>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "web-ext>addons-linter>dispensary>async>lodash",
          "web-ext>firefox-profile>archiver>async>lodash",
          "web-ext>addons-linter>eslint>lodash",
          "web-ext>firefox-profile>archiver>lodash",
          "web-ext>firefox-profile>async>lodash",
          "web-ext>firefox-profile>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "web-ext>addons-linter>snyk>inquirer>lodash",
          "web-ext>addons-linter>snyk>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "web-ext>addons-linter>snyk>snyk-config>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "web-ext>addons-linter>snyk>snyk-go-plugin>graphlib>lodash",
          "web-ext>addons-linter>snyk>snyk-nodejs-lockfile-parser>graphlib>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.10",
        "paths": [
          "web-ext>addons-linter>snyk>snyk-nodejs-lockfile-parser>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "web-ext>addons-linter>snyk>snyk-nuget-plugin>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      },
      {
        "version": "4.17.11",
        "paths": [
          "web-ext>addons-linter>snyk>snyk-php-plugin>lodash"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 1065,
    "title": "Prototype Pollution",
    "module_name": "lodash",
    "overview": "Versions of `lodash` before 4.17.12 are vulnerable to Prototype Pollution.  The function `defaultsDeep` allows a malicious user to modify the prototype of `Object` via `{constructor: {prototype: {...}}}` causing the addition or modification of an existing property that will exist on all objects.\n\n",
    "recommendation": "Update to version 4.17.12 or later.",
    "severity": "high",
    "url": "https://npmjs.com/advisories/1065"
  },
  {
    "findings": [
      {
        "version": "4.6.1",
        "paths": [
          "node-sass>lodash.mergewith"
        ],
        "dev": null,
        "optional": null,
        "bundled": null
      }
    ],
    "id": 1071,
    "title": "Prototype Pollution",
    "module_name": "lodash.mergewith",
    "overview": "Versions of `lodash.mergewith` before 4.6.2 are vulnerable to prototype pollution. The function `mergeWith` may allow a malicious user to modify the prototype of `Object` via `{constructor: {prototype: {...}}}` causing the addition or modification of an existing property that will exist on all objects.\n\n",
    "recommendation": "Update to version 4.6.2 or later.",
    "severity": "high",
    "url": "https://npmjs.com/advisories/1071"
  }
]
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
