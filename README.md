# curlaut

cURL with OAuth support.

## to be done

Feature
- manage oauth resource servers: alias, url, username, password
- cli http client to execute GET/POST/PUT/DELETE requests with OAuth bearer token from chosen resource server
- output to be consumed by jq
- json cli flag

for example:
```shell
./curlaut POST --json-body '{"key":"value"}' https://prod/api/v1/resource -v | jq
```

Modules
1. cmdline interface by clap
2. oauth configs
3. oauth jwt tokens cache: autorenew if expired
4. core: requests with options and auth
