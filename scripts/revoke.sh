#!/bin/bash

dir=$(dirname "$0")
db="$dir/../cosmetics.db"
key="$1"


row=$(sqlite3 $db "SELECT id FROM tokens WHERE token='$key'")
sqlite3 $db "DELETE FROM tokens WHERE token='$key'"
sqlite3 $db "DELETE FROM players WHERE token_id=$row"
sqlite3 $db "DELETE FROM costumes WHERE token_id=$row"