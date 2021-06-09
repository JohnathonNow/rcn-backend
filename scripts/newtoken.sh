#!/bin/bash

dir=$(dirname "$0")
db="$dir/../cosmetics.db"
key=$(openssl rand -hex 32)
name="$1"

echo "$key"
sqlite3 $db "INSERT INTO tokens(token, name) VALUES ('$key', '$name');"
