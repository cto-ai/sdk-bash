![](https://cto.ai/static/sdk-banner.png)

SDK for creating ops using bash.

# Prompts

## Input

```shell
$ ux prompt input -a \
  --message="Some message" \
  --name="input" \
  --default=Value
```

## Editor

```shell
$ ux prompt editor \
   --message="Some message" \
   --name="input" \
   --default=Value
```

## Confirm

```shell
$ ux prompt confirm --default-true \
  --message="Some message" \
  --name="input name"
```

## Autocomplete

### Example 1

```shell
$ ux prompt list --autocomplete \
 -n "autocomplete" \
 -m "Autocomplete Message" \
 -d val3 \
 val1 val2 val3 val4
```

### Example 2

```shell
$ ls list_fixture | xargs ux prompt list -a \
 -n "autocomplete" \
 -m "Autocomplete Message" \
 -d file1.js
```

### Example 3

```shell
$ cat list_fixture/list.txt | tr "\n" " " | xargs ux prompt list \
 -a \
 -n "autocomplete" \
 -m "Autocomplete Message" \
 -d "item 1"
```

## Checkbox

```shell
$ cat list_fixture/list.txt | tr "\n" " " | xargs ux prompt checkbox \
 -n "checkout" \
 -m "Checkout Message"
```

## List

```shell
$ cat list_fixture/list.txt | tr "\n" " " | xargs ux prompt list \
 -n "list" \
 -m "List Message" \
 -d "item 1"
```

## Datetime

```shell
# $(date +%Y-%m-%dT%H:%M:%S%Z)
# Formats "2019-12-11T21:37:12-08:00" or "2019-12-11T13:39:37Z"
# date -u +%FT%TZ

$ ux prompt datetime \
 -n "datetime" \
 -m "Datetime Message" \
 -d "2019-12-20T00:00:00-08:00" \
 --min="2019-12-11T00:00:00-08:00" \
 --max="2019-12-28T00:00:00-08:00" \
 --date
```

## Number

```shell
$ ux prompt number \
 -n "number" \
 -m "Number Message" \
 -d 5 \
 --min=1 \
 --max=10
```

## Password

```shell
$ ux prompt password \
 -n "password" \
 -m "Password Message" \
 --confirm
```

## Secret

```shell
$ ux prompt secret -n "secret" -m "Secret Message"
```

# Spinner

```shell
$ ux spinner start -m "Starting process"
$ sleep 5
$ ux spinner stop -m "Done!"
```

# Progress Bar

```shell
$ ux progressbar start -l 5 -m Downloading
$ for ((i=1;i<=5;++i)); do sleep 1; ux progressbar advance; done
$ ux progressbar stop -m 'Download Done!'
```

# Print

```shell
$ ux print some text
```

# Secrets

## Get

```shell
$ sdk secret get name
```

## Set

```shell
$ sdk secret set -k name -v secret
```

# State

## Get

```shell
$ sdk state get name
```

## Get All

```shell
$ sdk state get -a
```

## Set

```shell
$ sdk state set -k name -v state
```

# Config

## Get

```shell
$ sdk config get name
```

## Get All

```shell
$ sdk config get -a
```

## Set

```shell
$ sdk config set -k name -v config
```
