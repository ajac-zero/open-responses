#!/bin/bash
set -e

# Fetch OpenAPI spec from remote URL
URL="https://www.openresponses.org/openapi/openapi.json"
OUTPUT_FILE="openapi.json"

echo "Fetching OpenAPI spec from $URL..."
curl -s -f "$URL" -o "$OUTPUT_FILE"

echo "✓ Updated $OUTPUT_FILE"
