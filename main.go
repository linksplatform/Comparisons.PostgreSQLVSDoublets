package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"time"
	"context"
	"github.com/linksplatform/Data.Doublets.Gql/client" // hypothetical client; actually we call raw HTTP
)

const (
	hasuraURL   = "http://localhost:8080/v1/graphql"
	doubletsURL = "http://localhost:9090/graphql"
	numRecords  = 1000
)

type benchmarkResult struct {
	system    string
	operation string
	duration  time.Duration
	err       error
}

func main() {
	log.Println("Starting benchmark...")

	// Warm up
	warmUp()

	// Test queries
	hasuraResults := make([]time.Duration, 0, numRecords)
	doubletsResults := make([]time.Duration, 0, numRecords)

	for i := 0; i < numRecords; i++ {
		id := i + 1

		dur, err := measureHasuraGetByID(id)
		if err != nil {
			log.Printf("Hasura error at id %d: %v", id, err)
			continue
		}
		hasuraResults = append(hasuraResults, dur)

		dur, err = measureDoubletsGetByID(id)
		if err != nil {
			log.Printf("Doublets error at id %d: %v", id, err)
			continue
		}
		doubletsResults = append(doubletsResults, dur)
	}

	printStats("Hasura + PostgreSQL", hasuraResults)
	printStats("Doublets + Gql", doubletsResults)
}

func warmUp() {
	// Execute a few queries to warm caches
	for i := 1; i <= 10; i++ {
		measureHasuraGetByID(i)
		measureDoubletsGetByID(i)
	}
	time.Sleep(100 * time.Millisecond)
}

func measureHasuraGetByID(id int) (time.Duration, error) {
	query := fmt.Sprintf(`{"query":"query($id:Int!){entities_by_pk(id:$id){id name value}}","variables":{"id":%d}}`, id)
	return measureGraphQLRequest(hasuraURL, query)
}

func measureDoubletsGetByID(id int) (time.Duration, error) {
	// Doublets GQL schema: query { entity(id: $id) { id name value } }
	query := fmt.Sprintf(`{"query":"query($id:ID!){entity(id:$id){id name value}}","variables":{"id":"%d"}}`, id)
	return measureGraphQLRequest(doubletsURL, query)
}

func measureGraphQLRequest(url, queryJSON string) (time.Duration, error) {
	req, err := http.NewRequestWithContext(context.Background(), http.MethodPost, url, bytes.NewBufferString(queryJSON))
	if err != nil {
		return 0, err
	}
	req.Header.Set("Content-Type", "application/json")

	start := time.Now()
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return 0, err
	}
	defer resp.Body.Close()

	// Read body to ensure request completes
	io.Copy(io.Discard, resp.Body)

	return time.Since(start), nil
}

func printStats(system string, durations []time.Duration) {
	if len(durations) == 0 {
		fmt.Printf("%s: No successful queries\n", system)
		return
	}

	total := time.Duration(0)
	min := durations[0]
	max := durations[0]

	for _, d := range durations {
		total += d
		if d < min {
			min = d
		}
		if d > max {
			max = d
		}
	}

	avg := total / time.Duration(len(durations))
	fmt.Printf("=== %s ===\n", system)
	fmt.Printf("Total: %v\n", total)
	fmt.Printf("Average: %v\n", avg)
	fmt.Printf("Min: %v\n", min)
	fmt.Printf("Max: %v\n", max)
	fmt.Printf("Successful: %d/%d\n", len(durations), numRecords)
}
