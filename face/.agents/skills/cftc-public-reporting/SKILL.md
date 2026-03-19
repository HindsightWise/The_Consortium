---
name: cftc-public-reporting
description: "Fetch CFTC Commitments of Traders (COT) reports including Legacy and Disaggregated Futures & Options Combined data via Socrata API."
metadata:
  {
    "openclaw":
      {
        "emoji": "📈",
        "requires": { "bins": ["curl", "jq"] },
        "install": [],
      },
  }
---

# CFTC Public Reporting Skill

This skill provides access to the Commodity Futures Trading Commission (CFTC) Public Reporting API, specifically for Futures and Options Combined data used in market analysis.

## Dataset IDs (Socrata)

*   **Legacy Futures & Options Combined:** `jun7-fc8e`
*   **Disaggregated Futures & Options Combined:** `kh3c-gbw2`
*   **Base URL:** `https://publicreporting.cftc.gov/resource/{id}.json`

## Usage

### Simple Fetch (Script)

Use the helper script to fetch the latest 10 reports:

```bash
./scripts/fetch_cot.sh legacy 10
./scripts/fetch_cot.sh disaggregated 5
```

### Advanced Query (SODA API)

You can query the Socrata endpoint directly using `curl` with SoQL filters.

**Example: Traders in Financial Futures (TFF) for a specific contract**

```bash
curl "https://publicreporting.cftc.gov/resource.json?\$where=commodity_name='COPPER'"
```

*(Note: Check specific dataset documentation for available fields)*

## Key Fields

*   `report_date_as_yyyy_mm_dd`: The date of the report.
*   `market_and_exchange_names`: The name of the market (e.g., "CHICAGO MERCANTILE EXCHANGE").
*   `open_interest_all`: Total open interest.
*   `noncomm_positions_long_all`: Non-commercial long positions (speculators).
*   `noncomm_positions_short_all`: Non-commercial short positions.

## Automation Idea

Combine this with `jq` to track net positioning changes for specific assets as part of the "Empire Building" protocol.
