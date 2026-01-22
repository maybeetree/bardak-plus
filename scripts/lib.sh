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
