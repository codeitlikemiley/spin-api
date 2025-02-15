## Rust Spin + Open API + Swagger UI

> Deployed on Spin Cloud at https://app-h7qoscnn.fermyon.app/

## Development

Use Turso and Migrate Initial DB Schema

```sh
spin watch --runtime-config-file ./runtime-config.toml --sqlite @migration.sql
```

## Runtime Config

`runtime-config.toml`

```toml
[sqlite_database.default]
type = "libsql"
url = "" # add your turso url
token = "" # add turso token
```

## Deployment
1. `spin build`
2. `spin deploy`

