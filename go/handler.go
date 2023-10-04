package main

import (
	"context"
	"fmt"
	"net/http"
	"os"
)

func TbPostHandler(w http.ResponseWriter, r *http.Request) {
	texto := r.FormValue("texto")

	var id int
	err := pool.QueryRow(context.Background(), "INSERT INTO tb01(col_texto, col_dt) VALUES($1, now()) returning id;", texto).Scan(&id)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Erro ao inserir registro em tb01: %v\n", err)
		http.Error(w, "Erro ao inserir novo registro na tabela", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}
