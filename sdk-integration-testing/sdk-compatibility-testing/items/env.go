package items

import "os"

const (
	Token        = "not a real token"
	OpId         = "24e87318-e71d-46d7-8743-5edba21e1051"
	TeamId       = "dd8cda6d-5e49-4c4e-abe4-154ebeb5d741"
	RunId        = "07684782-4743-4e04-9b86-21377f15169f"
	WorkflowId   = "d0982cf9-7baa-4c67-bc55-bcd1a8bb3241"
	UserId       = "530ef73f-40b9-46d1-bdd4-e7c6600048e3"
	TeamName     = "integration testing"
	HostPlatform = "linux"
)

func LocalEnv() []string {
	return append(
		os.Environ(),
		"OPS_API_HOST=http://127.0.0.1:2819",
		"OPS_API_PATH=api/v1",
		"OPS_ACCESS_TOKEN="+Token,
		"OPS_TEAM_ID="+TeamId,
		"OPS_TEAM_NAME="+TeamName,
		"OPS_HOST_PLATFORM="+HostPlatform,
		"OPS_OP_ID="+OpId,
		"STATE_DIR=/root",
		"CONFIG_DIR=/root",
		"RUST_BACKTRACE=1",
		"SDK_RUN_ID="+RunId,
	)
}

func RemoteEnv() []string {
	return append(
		os.Environ(),
		"SDK_RUN_SOURCE=testremote",
		"SDK_BACKEND_URL=http://127.0.0.1:2819",
		"SDK_RUN_ID="+RunId,
		"SDK_SCHEDULE_ID=2000",
		"SDK_WORKFLOW_ID="+WorkflowId,
		"SDK_WORKFLOW_STEP=0",
		"SDK_OP_ID="+OpId,
		"SDK_USER_ID="+UserId,
		"SDK_TEAM_ID="+TeamId,
		"RUST_BACKTRACE=1",
	)
}
