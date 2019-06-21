package main

import (
	"encoding/json"
	"github.com/go-chi/chi"
	"log"
	"net/http"
)

func main() {
	log.Print("Starting Lixchess lobby server")

	r := chi.NewRouter()

	r.Post("/api/authenticate", authenticate)

	err := http.ListenAndServe(":8081", r)
	if err != nil {
		log.Fatal(err)
	}


}

type AuthAttemp struct {

}

func authenticate(w http.ResponseWriter, r *http.Request) {
	r.Body.
}
