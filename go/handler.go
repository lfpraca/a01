package main

import (
	"context"
	"fmt"
	"net/http"
	"encoding/json"
	"os"
)

type ReqData struct {
	Texto string `json:"texto"`
}

func TbPostHandler(w http.ResponseWriter, r *http.Request) {
	var input ReqData
	err := json.NewDecoder(r.Body).Decode(&input)
	if err != nil {
		http.Error(w, fmt.Sprintf("JSON de requisição inválido: %v", err), http.StatusBadRequest)
		return
	}

	var id int
	err = pool.QueryRow(context.Background(), "INSERT INTO tb01(col_texto, col_dt) VALUES($1, now()) returning id;", input.Texto).Scan(&id)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Erro ao inserir registro em tb01: %v\n", err)
		http.Error(w, "Erro ao inserir novo registro na tabela", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}
