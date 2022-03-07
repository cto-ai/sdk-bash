package structs

// Instruction is an instruction to the user code simulator
// We pass an array of these to the user code when running a test.
type Instruction struct {
	// Operation specifies which action to take, e.g. make a daemon request
	Operation string
	// Name specifies the name associated with the action, like a filename or daemon HTTP path
	Name string
	// If the action involves data, it'll be passed in this variable
	Body interface{}
	// This should be true if the daemon request involves waiting for an async result
	ReadResponse bool
	// This should be true if the response body has synchronous results
	ReadResponseSync bool
}

// RequestRecord represents the details of a request received by the mock API
// When the API finishes, it passes an array of these back to the test code
type RequestRecord struct {
	Endpoint string      `json:"endpoint"`
	Method   string      `json:"method"`
	Id       string      `json:"id,omitempty"`
	JsonBody interface{} `json:"jsonBody,omitempty"`
	Body     string      `json:"-"`
}

// Responses is the struct used to store the response from the mock-api to the sdk-daemon.
// This struct should be used directly by the tests
type Response struct {
	Reply     interface{} `json:"reply"`
	ReplyType string      `json:"reply_type"`
}

type Responses []Response

// Valid reply types as defined in the sdk daemon
const (
	ReplyTypeText      = "text"
	ReplyTypeSecret    = "secret"
	ReplyTypeSelection = "selection"
	ReplyTypeBoolean   = "boolean"
)
