#!/bin/bash
set -e

echo "Starting PostgreSQL + Hasura..."
docker-compose -f docker-compose.postgres-hasura.yaml up -d

echo "Waiting for Hasura to be ready..."
sleep 10

# Apply schema and seed data
# This would normally use Hasura metadata apply; for simplicity we assume a schema is pre-loaded.
# In practice, use hasura CLI or REST API to create table 'doublets' with columns id, source, target.

echo "Starting Doublets GQL..."
docker-compose -f docker-compose.doublets.yaml up -d

echo "Both services running."