package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"os/exec"

	"github.com/go-test/deep"

	"sdk-integration-testing/sdk-compatibility-testing/items"
	"sdk-integration-testing/sdk-compatibility-testing/structs"
)

type DaemonResult struct {
	DaemonOutput  string
	RequestRecord []structs.RequestRecord
}

func writeJson(filename string, data interface{}) error {
	body, err := json.Marshal(data)
	if err != nil {
		return err
	}

	err = ioutil.WriteFile(filename, body, 0644)
	if err != nil {
		return err
	}

	return nil
}

func StartApi(responses structs.Responses) (*exec.Cmd, error) {
	err := writeJson("apiserver.json", responses)
	if err != nil {
		return nil, err
	}

	cmd := exec.Command("/bin/mock-api")

	var stdout bytes.Buffer
	cmd.Stdout = &stdout
	err = cmd.Start()
	return cmd, err
}

func FinishApi(api *exec.Cmd) ([]structs.RequestRecord, error) {
	fmt.Println(api.CombinedOutput())
	err := api.Wait()
	if err != nil {
		return nil, err
	}

	bytes := api.Stdout.(*bytes.Buffer).Bytes()
	results := []structs.RequestRecord{}
	err = json.Unmarshal(bytes, &results)
	if err != nil {
		return nil, err
	}
	return results, nil
}

func ExecDaemon(env func() []string) ([]byte, error) {
	args := os.Args
	fmt.Printf("exec daemon with args %v \n", args)

	cmd := exec.Command("/bin/sdk-daemon", os.Args[1:]...)
	cmd.Env = env()

	return cmd.CombinedOutput()
}

func TriggerApiShutdown() error {
	_, err := http.Get("http://127.0.0.1:2819/end/test")
	return err
}

func main() {
	var result DaemonResult

	api, err := StartApi(structs.Responses{{"abc123", structs.ReplyTypeText}, {true, structs.ReplyTypeBoolean}, {"well", structs.ReplyTypeText}, {"1", structs.ReplyTypeText}, {"passwordTest", structs.ReplyTypeSecret}, {"2020-02-20T22:38:07Z", structs.ReplyTypeText}, {"2020-01-01T00:47:28Z", structs.ReplyTypeText}, {"2020-02-21T00:00:00Z", structs.ReplyTypeText}})
	if err != nil {
		panic(err)
	}

	daemonOutput, err := ExecDaemon(items.RemoteEnv)
	result.DaemonOutput = string(daemonOutput)
	fmt.Println(result.DaemonOutput)
	if err != nil {
		fmt.Println(string(err.(*exec.ExitError).Stderr))
		panic(err)
	}

	err = TriggerApiShutdown()
	if err != nil {
		panic(err)
	}

	requests, err := FinishApi(api)
	if err != nil {
		panic(err)
	}

	result.RequestRecord = requests

	expected := items.SuccessfulExecution(
		items.PrintRecord("Begin"),
		structs.RequestRecord{
			Endpoint: "track",
			Method:   "POST",
			JsonBody: map[string]interface{}{
				"opid":   items.OpId,
				"runid":  items.RunId,
				"teamid": items.TeamId,
				"tags":   []interface{}{"informational"},
				"userid": items.UserId,
				"metadata": map[string]interface{}{
					"error":      nil,
					"event":      "sdk-data",
					"username":   "",
					"os":         "unknown",
					"osVisible":  "unknown",
					"homeDir":    "/home/ops",
					"statePath":  "/tmp/state",
					"configPath": "/ops",
				},
			},
		},
		items.InteractionRecord(map[string]interface{}{
			"message":    "Input prompt",
			"reply_type": "text",
			"name":       "testInput",
		}),
		items.PrintRecord("Test Input: abc123"),
		items.InteractionRecord(map[string]interface{}{
			"message":    "Confirm prompt",
			"reply_type": "boolean",
			"name":       "testConfirm",
		}),
		items.PrintRecord("Confirm is true"),
		items.InteractionRecord(map[string]interface{}{
			"message":    "List prompt",
			"reply_type": "single_choice",
			"choices":    []interface{}{"well", "hello", "there"},
			"name":       "testList",
			"default":    nil,
		}),
		items.PrintRecord("well"),
		items.InteractionRecord(map[string]interface{}{
			"message":    "Autocomplete prompt",
			"reply_type": "single_choice",
			"choices":    []interface{}{"1", "2", "3"},
			"name":       "testAutocomplete",
			"default":    float64(1),
		}),
		items.PrintRecord("1"),
		items.InteractionRecord(map[string]interface{}{
			"message":    "Password prompt",
			"reply_type": "password",
			"name":       "testPassword",
		}),
		items.PrintRecord("Password matches"),
		items.InteractionRecord(map[string]interface{}{
			"message":    "DateTime prompt",
			"reply_type": "datetime",
			"default":    nil,
			"minimum":    "2020-02-20T22:38:07Z",
			"maximum":    nil,
			"name":       "testDateTime",
			"date_type":  "datetime",
		}),
		items.PrintRecord("2020-02-20T22:38:07Z"),
		items.InteractionRecord(map[string]interface{}{
			"message":    "Time only prompt",
			"reply_type": "datetime",
			"default":    nil,
			"minimum":    "2019-01-01T00:47:28Z",
			"maximum":    nil,
			"name":       "testTime",
			"date_type":  "time",
		}),
		items.PrintRecord("2020-01-01T00:47:28Z"),
		items.InteractionRecord(map[string]interface{}{
			"message":    "Date only prompt",
			"reply_type": "datetime",
			"default":    nil,
			"minimum":    "2020-02-21T00:00:00Z",
			"maximum":    nil,
			"name":       "testDate",
			"date_type":  "date",
		}),
		items.PrintRecord("2020-02-21T00:00:00Z"),
		items.PrintRecord("Starting spinner"),
		items.PrintRecord("Stopping spinner"),
		items.PrintRecord("Starting Progress Bar: 0% complete"),
		items.PrintRecord("40% complete"),
		items.PrintRecord("Complete: Stopping Progress Bar"),
		structs.RequestRecord{
			Endpoint: "setconfig",
			Method:   "POST",
			JsonBody: map[string]interface{}{
				"teamConfigs": map[string]interface{}{"CONFIG_KEY": "Some random value"},
			},
			Id: items.TeamId,
		},
		structs.RequestRecord{
			Endpoint: "getallconfig",
			Method:   "GET",
		},
		items.PrintRecord("CONFIG_KEY set: Some random value"),
		structs.RequestRecord{
			Endpoint: "getconfig",
			Method:   "GET",
			Id:       "CONFIG_KEY",
		},
		items.PrintRecord("CONFIG_KEY value retrieved: Some random value"),
		structs.RequestRecord{
			Endpoint: "deleteconfig",
			Method:   "DELETE",
			Id:       "CONFIG_KEY",
		},
		items.PrintRecord("CONFIG_KEY value deleted"),
		structs.RequestRecord{
			Endpoint: "getconfig",
			Method:   "GET",
			Id:       "CONFIG_KEY",
		},
		items.PrintRecord("CONFIG_KEY not found anymore"),
	)

	if len(requests) != len(expected) {
		fmt.Printf("WARNING: number of steps in the actual request record is different than expected.\n")
		fmt.Printf("got %d steps, expected %d steps\n", len(requests), len(expected))
	}

	if diff := deep.Equal(requests, expected); diff != nil {
		fmt.Println("ERROR: steps in the actual request record are different than expected!")
		for i, item := range diff {
			fmt.Printf("step %d diff (actual vs expected):\n", i+1)
			fmt.Println("	" + item)
		}
		panic("didn't match")
	}
	fmt.Println("PASS: Exemplar matches expectation")
}
