#!/usr/bin/bash
set -euo pipefail

SEV_VERSIONS=("3.0-0")
SEV_CERT_FILE=""

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

# Loop over to generate beacon report for all SEV certificates
for sev_version in "${SEV_VERSIONS[@]}"; do
  # Build title
  if [ -n "$OS_VERSION" ]; then
    SEV_TITLE="${OS_NAME} ${OS_VERSION} SEV version ${sev_version}"
  else
    SEV_TITLE="${OS_NAME} SEV version ${sev_version}"
  fi

  # Obtain SEV Version Content
  SEV_CERT_FILE="${HOME:-/root}/sev_certificate_v${sev_version}.txt"

  # Call beacon
  beacon report --title "$SEV_TITLE" --body "$SEV_CERT_FILE" --label "certificate" --label "${OS_LABEL}"

  echo "Published SEV certificate via beacon with title: $SEV_TITLE"
done
