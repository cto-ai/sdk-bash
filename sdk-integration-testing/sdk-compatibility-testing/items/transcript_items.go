package items

import (
	"sdk-integration-testing/sdk-compatibility-testing/structs"
)

var (
	// Start is the transcript entry expected if the user code starts normally
	Start = map[string]interface{}{"Start": true}
	// End is the transcript entry expected if the user code completes normally
	End = map[string]interface{}{"Start": false}
)

// RequestItem returns a transcript entry for a daemon HTTP request
func RequestItem(url string, statusCode int) map[string]interface{} {
	return map[string]interface{}{"URL": url, "StatusCode": float64(statusCode)}
}

// ReplyItem returns a transcript entry for a prompt reply
func ReplyItem(key string, value interface{}) map[string]interface{} {
	return map[string]interface{}{"Body": map[string]interface{}{key: value}}
}

func CompleteTranscript(entries ...interface{}) []interface{} {
	return append(
		append(
			[]interface{}{Start},
			entries...,
		),
		End,
	)
}

// LifecycleBody builds an expected request body for a lifecycle request
func LifecycleBody(event, contents string) map[string]interface{} {
	return map[string]interface{}{
		"event":         event,
		"contents":      contents,
		"run_id":        RunId,
		"schedule_id":   2000.0,
		"workflow_id":   WorkflowId,
		"workflow_step": 0.0,
		"debug":         false,
	}
}

var (
	// HealthRecord is the RequestRecord corresponding to a health check
	HealthRecord = structs.RequestRecord{Endpoint: "health", Method: "GET"}
	// StartRecord is the RequestRecord we expect for a starting lifecycle message
	StartRecord = structs.RequestRecord{
		Endpoint: "lifecycle",
		Method:   "POST",
		JsonBody: LifecycleBody("started", "Status Started"),
	}
	// CompleteRecord is the RequestRecord we expect for a successful lifecycle message
	CompleteRecord = structs.RequestRecord{
		Endpoint: "lifecycle",
		Method:   "POST",
		JsonBody: LifecycleBody("complete", ""),
	}
	FailedRecord = structs.RequestRecord{
		Endpoint: "lifecycle",
		Method:   "POST",
		JsonBody: LifecycleBody("failed", ""),
	}
	StateSaveRecord = structs.RequestRecord{
		Endpoint: "state",
		Method:   "POST",
		Id:       RunId,
	}
	MeRecord = structs.RequestRecord{Endpoint: "me", Method: "GET"}
)

func InteractionRecord(jsonBody map[string]interface{}) structs.RequestRecord {
	jsonBody["run_id"] = RunId
	jsonBody["step"] = 0.0
	return structs.RequestRecord{
		Endpoint: "interact",
		Method:   "POST",
		Id:       "testremote",
		JsonBody: jsonBody,
	}
}

func PrintRecord(message string) structs.RequestRecord {
	return InteractionRecord(map[string]interface{}{
		"message":    message,
		"reply_type": "none",
		"name":       "",
	})
}

func SuccessfulExecution(steps ...structs.RequestRecord) []structs.RequestRecord {
	return append(
		append([]structs.RequestRecord{HealthRecord, StartRecord}, steps...),
		StateSaveRecord,
		CompleteRecord,
	)
}

func SetConfigRecord(key, value, id string) structs.RequestRecord {
	jsonBody := map[string]interface{}{
		"teamConfigs": map[string]interface{}{key: value},
	}
	return structs.RequestRecord{
		Endpoint: "setconfig",
		Method:   "POST",
		JsonBody: jsonBody,
		Id:       id,
	}
}
