package main

import (
	"encoding/json"
	"fmt"
	"github.com/BurntSushi/toml"
	"github.com/go-chi/chi"
	"github.com/jinzhu/gorm"
	"log"
	"net/http"
)
import _ "github.com/go-sql-driver/mysql"

type config struct {
	Database database
}

type database struct {
	url string
	database string
	user string
	password string
}

var db *gorm.DB

func main() {
	log.Println("Starting LixChess lobby server")

	var config config
	_, err := toml.DecodeFile("config.toml", &config)
	if err != nil {
		log.Fatal(err)
	}

	r := chi.NewRouter()

	r.Post("/api/authenticate", authenticate)

	_db, dbErr := gorm.Open("mysql", fmt.Sprintf("%s:%s@/%s"))
	if dbErr != nil {
		log.Fatal(dbErr)
	}
	db = _db
	defer db.Close()

	db.AutoMigrate(&user{})

	httpErr := http.ListenAndServe(":8081", r)
	if httpErr != nil {
		log.Fatal(httpErr)
	}


}

type authAttempt struct {
	Username string
	Password string
}

type authResult struct {
	Success bool
	SessionId []byte
}

type user struct {
	gorm.Model
	Name string
	Password []byte
}

func authenticate(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")

	dec := json.NewDecoder(r.Body)
	var attempt authAttempt
	err := dec.Decode(&attempt)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	log.Println("Authenticating user")

	var user user
	if err := db.Where("name = ?", attempt.Username).First(&user).Error; err != nil {
		json.NewEncoder(w).Encode(map[string]bool { "success": false })
		return
	}


}
