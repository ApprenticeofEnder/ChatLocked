package main

import (
	"context"
	"fmt"
	"log"

	chroma "github.com/amikos-tech/chroma-go"
	defaultef "github.com/amikos-tech/chroma-go/pkg/embeddings/default_ef"
)

func main() {
	client, err := chroma.NewClient(chroma.WithBasePath("http://localhost:8000"))
	if err != nil {
		log.Fatalf("Error creating client: %s \n", err)
		return
	}

	ef, closeef, efErr := defaultef.NewDefaultEmbeddingFunction()

	defer func() {
		err := closeef()
		if err != nil {
			fmt.Printf("Error closing default embedding function: %s \n", err)
		}
	}()

	if efErr != nil {
		fmt.Printf("Error creating default embedding function: %s \n", efErr)
	}

	documents := []string{
		"Here is the Netflix password: Password1!",
	}
	resp, reqErr := ef.EmbedDocuments(context.Background(), documents)
	if reqErr != nil {
		fmt.Printf("Error embedding documents: %s \n", reqErr)
	}
	fmt.Printf("Embedding response: %v \n", resp)

	err = client.Close()
	if err != nil {
		fmt.Printf("Failed to close client: %v", err)
	}
}
