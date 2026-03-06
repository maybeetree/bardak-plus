#!/bin/sh

watch -n 0.5 "sqlite3 bardak.sqlite 'SELECT * FROM thumbs;'"

