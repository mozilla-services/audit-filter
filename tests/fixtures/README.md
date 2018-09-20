### Adding new test fixtures

Generally these files adhere to the following format (run from the directory using the tools or configs):

```console
npm audit --json > $(basename $(git remote get-url origin) .git)-$(git rev-parse HEAD)-npm-$(npm -v)-audit.json
nsp check --reporter=json . | python -m json.tool > $(basename $(git remote get-url origin) .git)-$(git rev-parse HEAD)-nsp-$(nsp --version)-check.json

cat .nsprc > $(basename $(git remote get-url origin) .git)-$(git rev-parse HEAD)-nsprc.json  # there's an invalid example that includes a -comment
```
