package main

import (
	"fmt"
	"log"
	"net/http"
)

// Port we listen on.
const portNum string = ":8080"

func ListenAndServe() {
	log.Println("Starting our simple http server.")

	// Registering our handler functions, and creating paths.
	http.HandleFunc("/", Home)
	http.HandleFunc("/info", Info)

	log.Println("Started on port", portNum)
	fmt.Println("To close connection CTRL+C :-)")

	// Spinning up the server.
	err := http.ListenAndServe(portNum, nil)
	if err != nil {
		log.Fatal(err)
	}
}

// Handler functions.
func Home(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Homepage")
	fmt.Fprint(w, r.URL.Query().Get("test"))
}

func Info(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Info page")
}
