# Bardak Plus: Inventory management for everyone

This is a VERY EARLY WORK IN PROGRESS REPO!

The idea is to make an API (and eventually webapp, android app,
desktop client)
that makes it easy to keep track of lots and lots
of clutter,
a faster, prettier, more secure, and more flexible
successor to the [original bardak](https://github.com/maybeetree/bardak).
For makerspaces, sports organisations,
social centers,
and just very cluttered households!

## Running

`cargo run` then visit `http://localhost:3030`.
Swagger UI is available.


## For Developers

### TODO

- structured sqlx error?
    - maybe client doesnt need to know but for logging?

- thumbs
    - image:
        - [x] avif
        - [x] jpeg
        - [x] webp
        - [x] jxl

### Database Migrations

```
sqlx migate add --sequential name_of_migration
```

will create `xxxx_name.up.sql` and `xxxx_name.down.sql`

### Non-rust deps

- `xbps-install -Syu pkg-config openssl-devel`

