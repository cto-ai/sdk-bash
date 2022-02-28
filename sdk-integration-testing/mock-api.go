package main

import (
	"bytes"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"time"

	"sdk-integration-testing/sdk-compatibility-testing/structs"

	"github.com/gorilla/mux"
)

var responses structs.Responses

var records = []structs.RequestRecord{}

func outputRecord(record structs.RequestRecord) {
	records = append(records, record)
}

func outputRecords() {
	body, err := json.Marshal(records)
	if err != nil {
		panic(err)
	}

	fmt.Println(string(body))
}

func Health(w http.ResponseWriter, r *http.Request) {
	outputRecord(structs.RequestRecord{
		Endpoint: "health",
		Method:   "GET",
	})

	w.WriteHeader(http.StatusNoContent)
}

func GetUserData(w http.ResponseWriter, r *http.Request) {
	outputRecord(structs.RequestRecord{
		Endpoint: "me",
		Method:   "GET",
	})

	// TODO: check for a token

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	_, err := w.Write([]byte(`{"data": {
		"me": {
			"id": "530ef73f-40b9-46d1-bdd4-e7c6600048e3",
			"username": "testuser",
			"email": "testuser@cto.ai",
			"firstName": "Test",
			"lastName": "User"
		},
		"teams": []
	}}`))
	if err != nil {
		panic(err)
	}
}

func GetState(w http.ResponseWriter, r *http.Request) {
	id := mux.Vars(r)["runId"]
	outputRecord(structs.RequestRecord{
		Endpoint: "state",
		Method:   "GET",
		Id:       id,
	})

	f, err := os.Open("fixtures/state.tar")
	if err != nil {
		panic(err)
	}

	w.Header().Set("Content-Type", "application/tar")
	w.WriteHeader(http.StatusOK)
	_, err = io.Copy(w, f)
	if err != nil {
		panic(err)
	}
}

func PostState(w http.ResponseWriter, r *http.Request) {
	id := mux.Vars(r)["runId"]

	bodyBytes, err := ioutil.ReadAll(r.Body)
	if err != nil {
		panic(err)
	}

	outputRecord(structs.RequestRecord{
		Endpoint: "state",
		Method:   "POST",
		Id:       id,
		Body:     base64.StdEncoding.EncodeToString(bodyBytes),
	})

	w.WriteHeader(http.StatusNoContent)
}

func Lifecycle(w http.ResponseWriter, r *http.Request) {
	bodyBytes, err := ioutil.ReadAll(r.Body)
	if err != nil {
		panic(err)
	}

	var jsonBody interface{}
	err = json.Unmarshal(bodyBytes, &jsonBody)
	if err != nil {
		panic(err)
	}

	outputRecord(structs.RequestRecord{
		Endpoint: "lifecycle",
		Method:   "POST",
		JsonBody: jsonBody,
	})

	w.WriteHeader(http.StatusNoContent)
}

func Track(w http.ResponseWriter, r *http.Request) {
	bodyBytes, err := ioutil.ReadAll(r.Body)
	if err != nil {
		panic(err)
	}

	var jsonBody interface{}
	err = json.Unmarshal(bodyBytes, &jsonBody)
	if err != nil {
		panic(err)
	}

	outputRecord(structs.RequestRecord{
		Endpoint: "track",
		Method:   "POST",
		JsonBody: jsonBody,
	})

	w.WriteHeader(http.StatusNoContent)
}

func Interact(w http.ResponseWriter, r *http.Request) {
	dest := mux.Vars(r)["dest"]

	bodyBytes, err := ioutil.ReadAll(r.Body)
	if err != nil {
		panic(err)
	}

	var jsonBody interface{}
	err = json.Unmarshal(bodyBytes, &jsonBody)
	if err != nil {
		panic(err)
	}

	outputRecord(structs.RequestRecord{
		Endpoint: "interact",
		Method:   "POST",
		Id:       dest,
		JsonBody: jsonBody,
	})

	// This is crude but enough to get started
	if !bytes.Contains(bodyBytes, []byte("\"none\"")) {
		try := func() {
			for {
				time.Sleep(25 * time.Millisecond)
				reply := responses[0].Reply
				reply_type := responses[0].ReplyType
				responses = responses[1:]
				body, _ := json.Marshal(map[string]interface{}{"reply": reply, "reply_type": reply_type})
				resp, err := http.Post("http://127.0.0.1:15234/response", "application/json", bytes.NewReader(body))
				if err != nil {
					panic(err)
				}
				if resp.StatusCode <= http.StatusNoContent {
					break
				}
			}
		}
		go try()
	}

	w.WriteHeader(http.StatusNoContent)
}

func GetSecret(w http.ResponseWriter, r *http.Request) {
	bodyBytes, err := ioutil.ReadAll(r.Body)
	if err != nil {
		panic(err)
	}

	var jsonBody interface{}
	err = json.Unmarshal(bodyBytes, &jsonBody)
	if err != nil {
		panic(err)
	}

	outputRecord(structs.RequestRecord{
		Endpoint: "getsecret",
		Method:   "POST",
		JsonBody: jsonBody,
	})

	reply := responses[0].Reply
	responses = responses[1:]
	if reply == nil {
		w.WriteHeader(http.StatusNotFound)
		return
	}

	w.WriteHeader(http.StatusOK)

	body, _ := json.Marshal(map[string]interface{}{"data": reply})
	_, err = w.Write(body)
	if err != nil {
		panic(err)
	}
}

func GetSecretList(w http.ResponseWriter, r *http.Request) {
	bodyBytes, err := ioutil.ReadAll(r.Body)
	if err != nil {
		panic(err)
	}

	var jsonBody interface{}
	err = json.Unmarshal(bodyBytes, &jsonBody)
	if err != nil {
		panic(err)
	}

	outputRecord(structs.RequestRecord{
		Endpoint: "getsecretlist",
		Method:   "POST",
		JsonBody: jsonBody,
	})

	reply := responses[0].Reply
	responses = responses[1:]
	if reply == nil {
		w.WriteHeader(http.StatusNotFound)
		return
	}

	w.WriteHeader(http.StatusOK)

	body, _ := json.Marshal(map[string]interface{}{"data": reply})
	_, err = w.Write(body)
	if err != nil {
		panic(err)
	}
}

func SetSecret(w http.ResponseWriter, r *http.Request) {
	bodyBytes, err := ioutil.ReadAll(r.Body)
	if err != nil {
		panic(err)
	}

	var jsonBody interface{}
	err = json.Unmarshal(bodyBytes, &jsonBody)
	if err != nil {
		panic(err)
	}

	outputRecord(structs.RequestRecord{
		Endpoint: "setsecret",
		Method:   "POST",
		JsonBody: jsonBody,
	})

	reply := responses[0].Reply
	responses = responses[1:]
	if reply == nil {
		w.WriteHeader(http.StatusNotFound)
		return
	}

	w.WriteHeader(http.StatusNoContent)
}

var config = map[string]string{}

func GetConfig(w http.ResponseWriter, r *http.Request) {
	key := mux.Vars(r)["key"]
	outputRecord(structs.RequestRecord{
		Endpoint: "getconfig",
		Method:   "GET",
		Id:       key,
	})

	value, ok := config[key]
	if !ok {
		w.WriteHeader(http.StatusNotFound)
		return
	}

	w.WriteHeader(http.StatusOK)
	body, _ := json.Marshal(map[string]interface{}{"data": value})
	_, err := w.Write(body)
	if err != nil {
		panic(err)
	}
}

func GetAllConfig(w http.ResponseWriter, r *http.Request) {
	outputRecord(structs.RequestRecord{
		Endpoint: "getallconfig",
		Method:   "GET",
	})

	result := make([]map[string]string, 0)
	for k, v := range config {
		result = append(result, map[string]string{"key": k, "value": v})
	}

	w.WriteHeader(http.StatusOK)
	body, _ := json.Marshal(map[string]interface{}{"data": result})
	_, err := w.Write(body)
	if err != nil {
		panic(err)
	}
}

func SetConfig(w http.ResponseWriter, r *http.Request) {
	teamName := mux.Vars(r)["teamName"]
	bodyBytes, err := ioutil.ReadAll(r.Body)
	if err != nil {
		panic(err)
	}

	var jsonBody struct {
		TeamConfigs map[string]string `json:"teamConfigs"`
	}
	err = json.Unmarshal(bodyBytes, &jsonBody)
	if err != nil {
		panic(err)
	}

	outputRecord(structs.RequestRecord{
		Endpoint: "setconfig",
		Method:   "POST",
		JsonBody: jsonBody,
		Id:       teamName,
	})

	for k, v := range jsonBody.TeamConfigs {
		config[k] = v
	}

	w.WriteHeader(http.StatusNoContent)
}

func DeleteConfig(w http.ResponseWriter, r *http.Request) {
	key := mux.Vars(r)["key"]
	outputRecord(structs.RequestRecord{
		Endpoint: "deleteconfig",
		Method:   "DELETE",
		Id:       key,
	})

	_, ok := config[key]
	if !ok {
		w.WriteHeader(http.StatusNotFound)
		return
	}

	delete(config, key)

	w.WriteHeader(http.StatusOK)
	body, _ := json.Marshal(map[string]interface{}{"data": true})
	_, err := w.Write(body)
	if err != nil {
		panic(err)
	}
}

func Shutdown(w http.ResponseWriter, r *http.Request) {
	go func() {
		time.Sleep(25 * time.Millisecond)
		outputRecords()
		os.Exit(0)
	}()

	w.WriteHeader(http.StatusNoContent)
}

func main() {
	data, err := ioutil.ReadFile("apiserver.json")
	if err != nil {
		panic(err)
	}
	err = json.Unmarshal(data, &responses)
	if err != nil {
		panic(err)
	}

	router := mux.NewRouter()

	apiRouter := router.PathPrefix("/api/v1").Subrouter()

	apiRouter.HandleFunc("/health", Health).Methods(http.MethodGet)
	apiRouter.HandleFunc("/me", GetUserData).Methods(http.MethodGet)

	internalRouter := apiRouter.PathPrefix("/internal").Subrouter()
	internalRouter.HandleFunc("/health", Health).Methods(http.MethodGet)
	internalRouter.HandleFunc("/sdkbackend/", Lifecycle).Methods(http.MethodPost)
	internalRouter.HandleFunc("/sdkbackend/getsecret", GetSecret).Methods(http.MethodPost)
	internalRouter.HandleFunc("/sdkbackend/setsecret", SetSecret).Methods(http.MethodPost)
	internalRouter.HandleFunc("/sdkbackend/getsecretlist", GetSecretList).Methods(http.MethodPost)
	internalRouter.HandleFunc("/state/{runId}", GetState).Methods(http.MethodGet)
	internalRouter.HandleFunc("/state/{runId}", PostState).Methods(http.MethodPost)
	internalRouter.HandleFunc("/interaction/daemon/{dest}", Interact).Methods(http.MethodPost)
	internalRouter.HandleFunc("/teams/{teamName}/configs/{key}", GetConfig).Methods(http.MethodGet)
	internalRouter.HandleFunc("/teams/{teamName}/configs", SetConfig).Methods(http.MethodPost)
	internalRouter.HandleFunc("/teams/{teamName}/configs", GetAllConfig).Methods(http.MethodGet)
	internalRouter.HandleFunc("/teams/{teamName}/configs/{key}", DeleteConfig).Methods(http.MethodDelete)
	internalRouter.HandleFunc("/log/event", Track).Methods(http.MethodPost)

	privateRouter := apiRouter.PathPrefix("/private").Subrouter()
	privateRouter.HandleFunc("/teams/{teamName}/configs/{key}", GetConfig).Methods(http.MethodGet)
	privateRouter.HandleFunc("/teams/{teamName}/configs", SetConfig).Methods(http.MethodPost)
	privateRouter.HandleFunc("/teams/{teamName}/configs", GetAllConfig).Methods(http.MethodGet)
	privateRouter.HandleFunc("/teams/{teamName}/configs/{key}", DeleteConfig).Methods(http.MethodDelete)
	privateRouter.HandleFunc("/log/event", Track).Methods(http.MethodPost)
	privateRouter.HandleFunc("/me", GetUserData).Methods(http.MethodGet)

	router.HandleFunc("/end/test", Shutdown).Methods(http.MethodGet)

	server := http.Server{
		Addr:         ":2819",
		Handler:      router,
		ReadTimeout:  10 * time.Second,
		WriteTimeout: 20 * time.Second,
		IdleTimeout:  120 * time.Second,
	}

	log.Fatal(
		server.ListenAndServe(),
	)
}
