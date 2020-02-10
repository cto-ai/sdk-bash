SDK for creating ops using bash.

# Prompts

## Input

```shell
$ bash-sdk prompt input -a \
  --message "Some message" \
  --name "input" \
  --default-value Value
```

## Editor

```shell
$ bash-sdk prompt editor \
   --message "Some message" \
   --name "input" \
   --default-value Value
```

## Confirm

```shell
$ bash-sdk prompt confirm --default-true \
  --message "Some message" \
  --name "input name"
```

## Autocomplete

### Example 1

```shell
$ bash-sdk prompt autocomplete \
 -n "autocomplete" \
 -m "Autocomplete Message" \
 -d val3 \
 -c val1 val2 val3 val4
```

### Example 2

```shell
$ ls list_fixture | xargs bash-sdk prompt autocomplete \
 -n "autocomplete" \
 -m "Autocomplete Message" \
 -d file1.js \
 -c
```

### Example 3

```shell
$ cat list_fixture/list.txt | tr "\n" " " | xargs bash-sdk prompt autocomplete \
 -n "autocomplete" \
 -m "Autocomplete Message" \
 -d "item 1" \
 -c
```

## Checkout

```shell
$ cat list_fixture/list.txt | tr "\n" " " | xargs bash-sdk prompt checkbox \
 -n "checkout" \
 -m "Checkout Message" \
 -c
```

## List

```shell
$ cat list_fixture/list.txt | tr "\n" " " | xargs bash-sdk prompt list \
 -n "list" \
 -m "List Message" \
 -d "item 1" \
 -c
```

## Datetime

```shell
# $(date +%Y-%m-%dT%H:%M:%S%Z)
# Formats "2019-12-11T21:37:12-08:00" or "2019-12-11T13:39:37Z"
# date -u +%FT%TZ

$ bash-sdk prompt datetime \
 -n "datetime" \
 -m "Datetime Message" \
 -d "2019-12-20T00:00:00-08:00" \
 --min "2019-12-11T00:00:00-08:00" \
 --max "2019-12-28T00:00:00-08:00" \
 --variant "date"
```

## Number

```shell
$ bash-sdk prompt number \
 -n "number" \
 -m "Number Message" \
 -d 5 \
 --min 1 \
 --max 10
```

## Password

```shell
$ bash-sdk prompt password \
 -n "password" \
 -m "Password Message" \
 --confirm
```

## Secret

```shell
$ bash-sdk prompt secret -n "secret" -m "Secret Message"
```

# Spinner

```shell
$ bash-sdk spinner start -m "Starting process"
$ sleep 5
$ bash-sdk spinner stop -m "Done!"
```

# Progress Bar

```shell
$ bash-sdk progressbar start -l 5 -m Downloading
$ for ((i=1;i<=5;++i)); do sleep 1; bash-sdk progressbar advance; done
$ bash-sdk progressbar stop -m Download Done!
```

# Print

```shell
$ bash-sdk print some text
```

# Secrets

## Get

```shell
# bash-sdk secrets get -n name
```

## Set

```shell
# bash-sdk secrets set -n name -s secret
```
