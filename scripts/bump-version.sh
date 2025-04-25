#!/bin/bash

# Simple script to bump version in Cargo.toml and package.json,
# commit the changes, and tag the commit.

set -e # Exit immediately if a command exits with a non-zero status.

NEW_VERSION=$1

# --- Validation ---
if [ -z "$NEW_VERSION" ]; then
  echo "Usage: $0 <new-version>" >&2
  echo "Example: $0 0.3.0" >&2
  exit 1
fi

# Basic format check (x.y.z)
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+([+-].+)?$ ]]; then
  echo "Error: Version format must be x.y.z (e.g., 0.3.0 or 1.0.0-beta.1)" >&2
  exit 1
fi

# Check if working directory is clean
if ! git diff --quiet HEAD --; then
    echo "Error: Working directory is not clean. Please commit or stash changes." >&2
    exit 1
fi

# --- File Updates ---
echo "Updating Cargo.toml to version ${NEW_VERSION}..."
# Use sed -i for in-place editing. The regex captures the version line.
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" Cargo.toml
rm Cargo.toml.bak # Remove backup file created by sed -i

echo "Updating package.json to version ${NEW_VERSION}..."
# Use sed -i for in-place editing. Assumes "version": "..." format on its own line or start of line.
sed -i.bak "s/^\s*\"version\": \".*\"/  \"version\": \"${NEW_VERSION}\",/" package.json
rm package.json.bak # Remove backup file

# --- Git Operations ---
echo "Staging updated files..."
git add Cargo.toml package.json

echo "Committing version bump..."
git commit -m "chore: Bump version to ${NEW_VERSION}"

echo "Creating git tag v${NEW_VERSION}..."
git tag "v${NEW_VERSION}"

# --- Done ---
echo "Version bumped to ${NEW_VERSION}. Files updated, committed, and tagged."
echo "Remember to push the commit and the tag:"
echo "  git push"
echo "  git push origin v${NEW_VERSION}" 