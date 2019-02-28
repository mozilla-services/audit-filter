0.4.3

* 0f50629 fix version in package-lock.json

0.4.2 (deprecated see note for 0.4.3)

* readme updates
* cc9e1aeaa9c594123e4001e5eb879e308f97b805 fix dropped bin entry in package.json  Thanks @agwells!

0.4.1 (deprecated see note for 0.4.2)

* dep updates
* 4334c5f38f52669943c431976c8334a63e79f273 drop peer dep to fix warning

0.4.0

* add --json flag and output (fixes #13)
* sort advisories by ID (fixes #6)
* update example/ and readme
* dep updates

0.3.0

Summary: publish as an npm package instead of binaries, readme and example updates, a new test fixture

* 8f5e47195e3b2ea0830e1189b20f7133adba5e33 update readme
* d00ed3152eb2a3de4b1748a3ec96b4af6350e94a update example
* 51bf99e1b6e2b749242d74e8c6a973cb6995dcfc add wasm-pack generated pkg with cli shim
* fe9fc6190b1eb77f8debf17d5be18b25d23d2540 src: update main and lib for wasm bindgen bindings
* 0a0dbd322b80baf0b1f1fb78b87e96ec159249de ci: Add node 8 and 10 with wasm-pack; drop rust binary builds
* a5bd2a796e906312e84ac49e25270612e80aa5bc cargo: Add wasm-bindgen
* 7b410dccbb9f7eb71ae8351951145682e4c7ce7a readme: add crates.io badge
* 3f0998afaec7948e73d41d8b392caebf6f4e38c7 tests: add fxa-content-server fixture
* 13e6c9c5cd647c94cd67e26e64278a629c5ca4c8 readme: add required npm version
* f123406b468a7370fd7858921e198ba5d965e1e4 readme: fix output to reflect sorting fix for #2
* dc87fea348c03f4a5d009d316413e146a1f67b94 update advisory host in example nsprc and readme
* a87aeffc6a084774a97955a4f27f62956109720b example: update tag and sha sums in run script

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
