import requests
import time
import csv
import json

# Configuration
HASURA_ENDPOINT = "http://localhost:8080/v1/graphql"
DOUBLETS_ENDPOINT = "http://localhost:5000/graphql"

HEADERS = {"Content-Type": "application/json"}

def run_single_query(endpoint, query, variables=None):
    payload = {"query": query}
    if variables:
        payload["variables"] = variables
    start = time.perf_counter()
    response = requests.post(endpoint, json=payload, headers=HEADERS)
    end = time.perf_counter()
    return end - start, response.json()

def benchmark_hasura():
    # Insert a single item
    insert_query = """
    mutation InsertItem($name: String!) {
        insert_items_one(object: {name: $name}) {
            id
            name
        }
    }
    """
    duration, result = run_single_query(HASURA_ENDPOINT, insert_query, {"name": "test"})
    inserted_id = result["data"]["insert_items_one"]["id"]

    # Query the item
    query_query = """
    query GetItem($id: Int!) {
        items_by_pk(id: $id) {
            id
            name
        }
    }
    """
    duration_q, _ = run_single_query(HASURA_ENDPOINT, query_query, {"id": inserted_id})

    # Update the item
    update_query = """
    mutation UpdateItem($id: Int!, $name: String!) {
        update_items_by_pk(pk_columns: {id: $id}, _set: {name: $name}) {
            id
            name
        }
    }
    """
    duration_u, _ = run_single_query(HASURA_ENDPOINT, update_query, {"id": inserted_id, "name": "updated"})

    # Delete the item
    delete_query = """
    mutation DeleteItem($id: Int!) {
        delete_items_by_pk(id: $id) {
            id
        }
    }
    """
    duration_d, _ = run_single_query(HASURA_ENDPOINT, delete_query, {"id": inserted_id})

    return {
        "insert": duration,
        "query": duration_q,
        "update": duration_u,
        "delete": duration_d
    }

def benchmark_doublets():
    # Insert a single item
    insert_query = """
    mutation InsertItem($name: String!) {
        createItem(name: $name) {
            id
            name
        }
    }
    """
    duration, result = run_single_query(DOUBLETS_ENDPOINT, insert_query, {"name": "test"})
    inserted_id = result["data"]["createItem"]["id"]

    # Query the item
    query_query = """
    query GetItem($id: ID!) {
        item(id: $id) {
            id
            name
        }
    }
    """
    duration_q, _ = run_single_query(DOUBLETS_ENDPOINT, query_query, {"id": inserted_id})

    # Update the item
    update_query = """
    mutation UpdateItem($id: ID!, $name: String!) {
        updateItem(id: $id, name: $name) {
            id
            name
        }
    }
    """
    duration_u, _ = run_single_query(DOUBLETS_ENDPOINT, update_query, {"id": inserted_id, "name": "updated"})

    # Delete the item
    delete_query = """
    mutation DeleteItem($id: ID!) {
        deleteItem(id: $id) {
            id
        }
    }
    """
    duration_d, _ = run_single_query(DOUBLETS_ENDPOINT, delete_query, {"id": inserted_id})

    return {
        "insert": duration,
        "query": duration_q,
        "update": duration_u,
        "delete": duration_d
    }

if __name__ == "__main__":
    print("Benchmarking Hasura...")
    hasura_results = benchmark_hasura()
    print("Hasura done.")
    print("Benchmarking Doublets...")
    doublets_results = benchmark_doublets()
    print("Doublets done.")

    with open("results.csv", "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["System", "Operation", "Duration (s)"])
        for op in ["insert", "query", "update", "delete"]:
            writer.writerow(["Hasura", op, hasura_results[op]])
            writer.writerow(["Doublets", op, doublets_results[op]])

    print("Results saved to results.csv")
