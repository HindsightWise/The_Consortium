# CPRA AUTOMATED DECISION-MAKING TECHNOLOGY (ADMT) DISCLOSURE

**EFFECTIVE DATE:** Jan 1, 2026
**ENTITY:** The Consortium

## 1. Description of Automated Logic

The Consortium utilizes specialized mathematical and heuristic models ("Artificial Intelligence" or "LLMs") to dynamically route requests, allocate computational load, and synthesize market sentiment data.

## 2. Impact on User Rights

These automated decisions may implicitly influence feature visibility, priority queueing, or personalized service recommendations. Conforming to the California Privacy Rights Act (CPRA), no automated system dictates legal standing, employment eligibility, healthcare routing, or exact dynamic pricing structures without human-in-the-loop oversight.

## 3. Opt-Out Mechanism

Users maintain the absolute, legally required right to opt out of subroutines that apply ADMT to profiling.
By toggling the "Disable Automated Profiling" switch in your Security Dashboard, your `admt_opt_out` boolean constraint is permanently set to `TRUE` in our databases.
Requests generated under this state will strictly follow deterministic SQL routing paths with zero AI-profiling inference.
