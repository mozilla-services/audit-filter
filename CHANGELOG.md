0.2.5

* b9439f5 readme: Add install as npm script
* b32d316 example: Add npm script
* 6a75ca2 ci: remove artifacts/ from shasum path

0.2.4

* 966205a ci: fix file glob for uploading releases

0.2.3

* 265fac6 ci: don't include top level artifacts/ dir in release tarballs

0.2.2

* 20a2dba CI: per target artifacts dirs; fix missing sha256sum on osx

0.2.1

* CI fix to publish releases

0.2.0

* return output sorted ascending fixes #1
* return URLs unchanged (they aren't all nodesecurity.io) fixes #2
* enable CI and use it to build executables, strip executables, publish tags to Github Releases, add builds for OSX

0.1.1

* 5f48439 readme: add install w/ wget directions
* 0b50578 fix printing version
* 005bea5 parse nsp config first (to fail faster and since audit defaults to stdin)

0.1.0

* initial release
