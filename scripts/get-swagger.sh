#!/bin/bash

# URL of the Swagger / OpenAPI spec
SWAGGER_URL="https://www.regionoix.gasdev.fr/api-docs/openapi.json"

# Target folder and file
OUTPUT_FOLDER="./src/swaggers"
OUTPUT_FILE="$OUTPUT_FOLDER/regionoix.json"

# Create folder if it doesn't exist
mkdir -p "$OUTPUT_FOLDER"

# Download the Swagger file
echo "Downloading Swagger file from $SWAGGER_URL..."
curl -sSL "$SWAGGER_URL" -o "$OUTPUT_FILE"

if [ $? -eq 0 ]; then
    echo "Swagger file saved to $OUTPUT_FILE"
else
    echo "Failed to download Swagger file"
    exit 1
fi
