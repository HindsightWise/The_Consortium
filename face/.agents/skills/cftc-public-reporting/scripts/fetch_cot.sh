#!/bin/bash

# CFTC Public Reporting API (Socrata) Fetcher
# Usage: ./fetch_cot.sh [legacy|disaggregated] [limit]

TYPE=${1:-legacy}
LIMIT=${2:-10}

LEGACY_ID="jun7-fc8e"
DISAGG_ID="kh3c-gbw2"

if [ "$TYPE" == "legacy" ]; then
    DATASET_ID=$LEGACY_ID
elif [ "$TYPE" == "disaggregated" ]; then
    DATASET_ID=$DISAGG_ID
else
    echo "Unknown report type: $TYPE. Use 'legacy' or 'disaggregated'."
    exit 1
fi

URL="https://publicreporting.cftc.gov/resource/${DATASET_ID}.json?\$limit=${LIMIT}&\$order=report_date_as_yyyy_mm_dd%20DESC"

echo "Fetching ${TYPE} COT data from ${URL}..."
curl -s "${URL}"
