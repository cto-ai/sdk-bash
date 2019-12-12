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

cat list_fixture/list.txt | tr "\n" " " | xargs ./target/debug/bash-sdk checkbox \
  -n "checkout" \
  -m "Checkout Message" \
  -c
