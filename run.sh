#!/bin/sh

# echo to stderr
eecho() {
	echo "$@" >&2
}

# echo to stderr and exit with error
die() {
	eecho "$@"
	exit 1
}

cd_ass() {
	cd "$1" || die "Failed to cd into ${2:-$1}"
}

export DATABASE_URL="sqlite:bardak.sqlite"
export RUST_LOG=warp=debug,hyper=debug

rm -rf ./.sqlx || die "cannot rm sqlx"
rm -rf ./bardak.sqlite || die "cannot rm db"
sqlx db create || die "cannot db create"
sqlx migrate run || die "cannot run migrations"
exec cargo run

