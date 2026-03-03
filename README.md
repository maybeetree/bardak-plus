# foo

## Queries

1. `cargo install sqlx-cli`
1. `sqlx db setup`
    - reads .env
1. `cargo sqlx prepare`
1. (optional) `git add .sqlx`
1. `cargo run`

## Database Migrations

```
sqlx migate add --sequential name_of_migration
```

will create `xxxx_name.up.sql` and `xxxx_name.down.sql`

## TODO

- structured sqlx error?
    - maybe clienbt doesnt need to know but for logging?

- thumbs
    - image:
        - avif
        - jpeg
        - webp
    - <https://github.com/lamco-admin/jxl-rust-reference>
        - jxl

