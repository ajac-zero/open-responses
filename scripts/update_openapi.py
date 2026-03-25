#!/usr/bin/env python3
"""
Script to fetch and update openapi.json from the remote OpenResponses API.
"""

import json
import urllib.request
import urllib.error
import sys
from pathlib import Path

URL = "https://www.openresponses.org/openapi/openapi.json"
OUTPUT_FILE = Path(__file__).parent.parent / "openapi.json"


def fetch_openapi() -> dict:
    """Fetch OpenAPI spec from remote URL."""
    print(f"Fetching OpenAPI spec from {URL}...")
    try:
        with urllib.request.urlopen(URL) as response:
            data = json.loads(response.read().decode())
            return data
    except urllib.error.URLError as e:
        print(f"Error: Failed to fetch from {URL}: {e}", file=sys.stderr)
        sys.exit(1)


def update_openapi():
    """Fetch and save OpenAPI spec if changed."""
    spec = fetch_openapi()
    
    # Check if file exists and compare
    if OUTPUT_FILE.exists():
        with open(OUTPUT_FILE, "r") as f:
            existing = json.load(f)
        
        if spec == existing:
            print(f"✓ No changes")
            return
    
    with open(OUTPUT_FILE, "w") as f:
        json.dump(spec, f, indent=2)
    
    print(f"✓ Updated {OUTPUT_FILE}")


if __name__ == "__main__":
    update_openapi()
