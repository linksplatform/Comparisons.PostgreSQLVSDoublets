#!/bin/bash
# This script downloads the PostgresPro Airlines demo database.
# It will be executed automatically when the container starts (if the DB doesn't exist yet).

set -e

# Default to 6 months dataset (can be overridden with DEMO_SIZE env var)
DEMO_SIZE=${DEMO_SIZE:-6m}
DEMO_DATE="20250901"

echo "Downloading PostgresPro Airlines demo database (${DEMO_SIZE})..."

cd /tmp
wget -q "https://edu.postgrespro.ru/demo-${DEMO_DATE}-${DEMO_SIZE}.sql.gz" -O demo.sql.gz

echo "Extracting and loading demo database..."
gunzip -c demo.sql.gz | psql -U postgres -d postgres

echo "Demo database loaded successfully!"
rm demo.sql.gz
