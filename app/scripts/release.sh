#!/bin/bash
set -e

# Change to the app directory if the script is run from elsewhere
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR/.."

# 1. Check for GITHUB_TOKEN
if [ -z "$GITHUB_TOKEN" ]; then
    echo "Error: GITHUB_TOKEN environment variable is not set."
    echo "Please provide a Personal Access Token (PAT)."
    exit 1
fi

# 2. Extract configuration
VERSION=$(jq -r '.version' src-tauri/tauri.conf.json)
PRODUCT_NAME=$(jq -r '.productName' src-tauri/tauri.conf.json)
REPO="PaDreyer/razer-x"
TAG="v$VERSION"

echo "--- Preparing release for $PRODUCT_NAME $TAG ---"

# 3. Build the application
echo "Building signed application..."
yarn build:signed

# 4. Locate artifacts
BUNDLE_DIR="../target/release/bundle"
echo "Locating artifacts in $BUNDLE_DIR..."

UPDATER_BUNDLE=$(find "$BUNDLE_DIR" -name "*.app.tar.gz" | head -n 1)
UPDATER_SIG=$(find "$BUNDLE_DIR" -name "*.app.tar.gz.sig" | head -n 1)
LATEST_JSON=$(find "$BUNDLE_DIR" -name "latest.json" | head -n 1)
DMG_FILE=$(find "$BUNDLE_DIR" -name "*.dmg" -not -path "*/macos/rw.*" | head -n 1)

# Check if we found them
MISSING=0
if [[ -z "$UPDATER_BUNDLE" ]]; then echo "Missing updater bundle (.app.tar.gz)"; MISSING=1; fi
if [[ -z "$UPDATER_SIG" ]]; then echo "Missing updater signature (.sig)"; MISSING=1; fi
if [[ -z "$LATEST_JSON" ]]; then echo "Missing latest.json"; MISSING=1; fi
if [[ -z "$DMG_FILE" ]]; then echo "Missing DMG file"; MISSING=1; fi

if [ $MISSING -eq 1 ]; then
    echo "Error: Some artifacts were not found."
    exit 1
fi

# 5. Create or Get GitHub Release
echo "Getting or creating GitHub release $TAG..."

# Check if release exists
RELEASE_RESPONSE=$(curl -s -H "Authorization: token $GITHUB_TOKEN" "https://api.github.com/repos/$REPO/releases/tags/$TAG")
RELEASE_ID=$(echo "$RELEASE_RESPONSE" | jq -r '.id // empty')

if [ -z "$RELEASE_ID" ]; then
    echo "Creating new release $TAG..."
    RELEASE_RESPONSE=$(curl -s -X POST \
        -H "Authorization: token $GITHUB_TOKEN" \
        -H "Accept: application/vnd.github.v3+json" \
        "https://api.github.com/repos/$REPO/releases" \
        -d "{
          \"tag_name\": \"$TAG\",
          \"target_commitish\": \"main\",
          \"name\": \"$TAG\",
          \"body\": \"Release $TAG\",
          \"draft\": false,
          \"prerelease\": false
        }")
    RELEASE_ID=$(echo "$RELEASE_RESPONSE" | jq -r '.id')
    
    if [ "$RELEASE_ID" == "null" ]; then
        echo "Error creating release:"
        echo "$RELEASE_RESPONSE"
        exit 1
    fi
else
    echo "Using existing release ID: $RELEASE_ID"
fi

# 6. Upload Assets
function upload_asset {
    local file_path=$1
    local name=$(basename "$file_path")
    echo "Uploading $name..."
    
    # Check if asset already exists and delete it if it does
    local asset_id=$(curl -s -H "Authorization: token $GITHUB_TOKEN" "https://api.github.com/repos/$REPO/releases/$RELEASE_ID/assets" | jq -r ".[] | select(.name == \"$name\") | .id")
    if [ ! -z "$asset_id" ]; then
        echo "Deleting existing asset $name (ID: $asset_id)..."
        curl -s -X DELETE -H "Authorization: token $GITHUB_TOKEN" "https://api.github.com/repos/$REPO/releases/assets/$asset_id"
    fi

    # Upload the asset
    curl -s -X POST \
        -H "Authorization: token $GITHUB_TOKEN" \
        -H "Content-Type: application/octet-stream" \
        --data-binary @"$file_path" \
        "https://uploads.github.com/repos/$REPO/releases/$RELEASE_ID/assets?name=$name" \
        | jq -r '.id'
}

upload_asset "$UPDATER_BUNDLE"
upload_asset "$UPDATER_SIG"
upload_asset "$LATEST_JSON"
upload_asset "$DMG_FILE"

echo "--- Release $TAG published successfully via GitHub API! ---"
