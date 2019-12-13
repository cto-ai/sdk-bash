#!/bin/bash

## Prompts

# Input

# ./target/debug/bash-sdk prompt input -a \
#   --message "Some message" \
#   --name "input" \
#   --default-value Value

# Editor

# ./target/debug/bash-sdk prompt editor \
#    --message "Some message" \
#    --name "input" \
#    --default-value Value

# Confirm

# ./target/debug/bash-sdk prompt confirm -f --message "Some message" --name "input name"
# ./target/debug/bash-sdk prompt confirm --default-true --message "Some message" --name "input name"

# Autocomplete

# ./target/debug/bash-sdk prompt autocomplete \
#   -n "autocomplete" \
#   -m "Autocomplete Message" \
#   -d val3 \
#   -c val1 val2 val3 val4

# ls list_fixture | xargs ./target/debug/bash-sdk prompt autocomplete \
#   -n "autocomplete" \
#   -m "Autocomplete Message" \
#   -d  file1.js \
#   -c

# cat list_fixture/list.txt | tr "\n" " " | xargs ./target/debug/bash-sdk prompt autocomplete \
#   -n "autocomplete" \
#   -m "Autocomplete Message" \
#   -d  "item 1" \
#   -c

# Checkout

# cat list_fixture/list.txt | tr "\n" " " | xargs ./target/debug/bash-sdk prompt checkbox \
#   -n "checkout" \
#   -m "Checkout Message" \
#   -c

# List

# cat list_fixture/list.txt | tr "\n" " " | xargs ./target/debug/bash-sdk prompt list \
#   -n "list" \
#   -m "List Message" \
#   -d  "item 1" \
#   -c

# Datetime
# $(date +%Y-%m-%dT%H:%M:%S%Z)
# Formats "2019-12-11T21:37:12-08:00" or "2019-12-11T13:39:37Z"
# date -u +%FT%TZ

# ./target/debug/bash-sdk prompt datetime \
#   -n "datetime" \
#   -m "Datetime Message" \
#   -d "2019-12-20T00:00:00-08:00" \
#   --min "2019-12-11T00:00:00-08:00" \
#   --max "2019-12-28T00:00:00-08:00" \
#   --variant "date"

# Number

# ./target/debug/bash-sdk prompt number \
#   -n "number" \
#   -m "Number Message" \
#   -d 5 \
#   --min 1 \
#   --max 10 \

# Password

# ./target/debug/bash-sdk prompt password \
#   -n "password" \
#   -m "Password Message" \
#   --confirm

# Secret

# ./target/debug/bash-sdk prompt secret -n "secret" -m "Secret Message"

# Spinner

# ./target/debug/bash-sdk spinner start -m "Starting process"
# sleep 5
# ./target/debug/bash-sdk spinner stop -m "Done!"

# Progress Bar

# ./target/debug/bash-sdk progressbar start -l 5 -m "Downloading"
#
# for ((i=1;i<=5;++i))
# do
#   sleep 1
#   ./target/debug/bash-sdk progressbar advance
# done
#
# ./target/debug/bash-sdk progressbar stop

# Print

./target/debug/bash-sdk print some text
