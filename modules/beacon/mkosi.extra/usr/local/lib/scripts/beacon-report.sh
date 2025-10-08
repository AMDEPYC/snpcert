#!/usr/bin/bash
set -euo pipefail

CERT_FILE="${HOME:-/root}/sev_certificate.txt"

# Determine OS name and version
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS_NAME="${ID}"            
    OS_VERSION="${VERSION_ID}" 
    OS_LABEL="${OS_NAME}-${OS_VERSION}"
else
    OS_NAME="$(uname -s)"
    OS_VERSION=""
    OS_LABEL="${OS_NAME}"
fi

# Get current date
TODAY=$(date +%Y-%m-%d)

# Build title
if [ -n "$OS_VERSION" ]; then
    TITLE="${OS_NAME} ${OS_VERSION} ${TODAY} SEV Certificate"
else
    TITLE="${OS_NAME} ${TODAY} SEV Certificate"
fi

# Call beacon
beacon report --title "$TITLE" --body "$CERT_FILE" --label "certificate" --label "${OS_LABEL}"

echo "Published SEV certificate via beacon with title: $TITLE"
