#!/bin/bash

dir=$(dirname "$0")
db="$dir/../cosmetics.db"
key=$(openssl rand -base64 32)

echo "$key"
sqlite3 $db "INSERT INTO tokens(token) VALUES ('$key');"
