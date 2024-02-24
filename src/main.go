package main

import (
	"time"
)

func main() {
	go ListenAndServe()

	for {
		time.Sleep(1000)
	}
}
