#!/bin/bash

# Input

# ./target/debug/bash-sdk input -a --message "Some message" --name "input name" --default-value Value

# Editor

# ./target/debug/bash-sdk editor --message "Some message" --name "input name" --default-value Value

# Confirm

# ./target/debug/bash-sdk confirm -f --message "Some message" --name "input name"
# ./target/debug/bash-sdk confirm --default-true --message "Some message" --name "input name"

# Autocomplete

# ./target/debug/bash-sdk autocomplete \
#   -n "autocomplete" \
#   -m "Autocomplete Message" \
#   -d val3 \
#   -c val1 val2 val3 val4

# ls list_fixture | xargs ./target/debug/bash-sdk autocomplete \
#   -n "autocomplete" \
#   -m "Autocomplete Message" \
#   -d  file1.js \
#   -c

# cat list_fixture/list.txt | tr "\n" " " | xargs ./target/debug/bash-sdk autocomplete \
#   -n "autocomplete" \
#   -m "Autocomplete Message" \
#   -d  "item 1" \
#   -c

# Checkout

# cat list_fixture/list.txt | tr "\n" " " | xargs ./target/debug/bash-sdk checkbox \
#   -n "checkout" \
#   -m "Checkout Message" \
#   -c

# List

# cat list_fixture/list.txt | tr "\n" " " | xargs ./target/debug/bash-sdk list \
#   -n "list" \
#   -m "List Message" \
#   -d  "item 1" \
#   -c

# Datetime
# $(date +%Y-%m-%dT%H:%M:%S%Z)
# Formats "2019-12-11T21:37:12-08:00" or "2019-12-11T13:39:37Z"
# date -u +%FT%TZ

./target/debug/bash-sdk datetime \
  -n "datetime" \
  -m "Datetime Message" \
  -d "2019-12-20T00:00:00-08:00" \
  --min "2019-12-11T00:00:00-08:00" \
  --max "2019-12-28T00:00:00-08:00" \
  --variant "date"
