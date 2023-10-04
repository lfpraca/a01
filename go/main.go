package main

import (
	"context"
	"fmt"
	"net/http"
	"os"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/gorilla/mux"
	_ "github.com/joho/godotenv/autoload"
)

var pool *pgxpool.Pool

func main() {
	var err error
	pool, err = pgxpool.New(context.Background(), os.Getenv("DATABASE_URL"))
	if err != nil {
		fmt.Fprintf(os.Stderr, "Falha ao conectar com o banco de dados: %v\n", err)
		os.Exit(1)
	}
	defer pool.Close()

	router := mux.NewRouter()

	router.HandleFunc("/tb01", TbPostHandler).Methods("POST")

    fmt.Println("Servidor iniciado em :8080")
    http.ListenAndServe(":8080", router)
}
