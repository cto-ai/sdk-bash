ux print Begin

sdk track sdk-data informational "osVisible:unknown" "homeDir:$SDK_HOME_DIR" "statePath:$SDK_STATE_DIR" "configPath:/ops"

input=$(ux prompt input --name=testInput --message="Input prompt" --default="open")

ux print Test Input: $input

confirm=$(ux prompt confirm --name=testConfirm --message="Confirm prompt")

ux print Confirm is $confirm

list=$(ux prompt list --name=testList --message="List prompt" well hello there)

ux print $list

# Note that we can't pass a default index like in Python and Node
# so we get the same result a different way
ac=$(ux prompt list --name=testAutocomplete  --message="Autocomplete prompt" --autocomplete --default="2" 1 2 3)

ux print $ac

password=$(ux prompt password --name="testPassword" --message="Password prompt")

if [ "$password" = "passwordTest" ]; then
    ux print Password matches
fi

datetime=$(ux prompt datetime --name="testDateTime" --message="DateTime prompt" --min="2020-02-20T22:38:07Z")

ux print $datetime

time=$(ux prompt datetime --time --name="testTime" --message="Time only prompt" --min="2019-01-01T00:47:28Z")

ux print $time

date=$(ux prompt datetime --date --name="testDate" --message="Date only prompt" --min="2020-02-21T00:00:00Z")

ux print $date

ux spinner start -m "Starting spinner"

ux spinner stop -m "Stopping spinner"

ux progressbar start -l 5 -i 0 --message="Starting Progress Bar"

ux progressbar advance 2

ux progressbar stop --message="Stopping Progress Bar"

# Simulate the behaviour of Python and Node config set
sdk config set -k CONFIG_KEY -v "Some random value"
sdk config get -a > /dev/null
ux print CONFIG_KEY set: Some random value

value=$(sdk config get CONFIG_KEY)
ux print CONFIG_KEY value retrieved: $value

was_deleted=$(sdk config delete CONFIG_KEY)
if [ "$was_deleted" = "true" ]; then
    ux print CONFIG_KEY value deleted
fi

new_value=$(sdk config get CONFIG_KEY)
if [ -z "$new_value" ]; then
    ux print CONFIG_KEY not found anymore
fi
