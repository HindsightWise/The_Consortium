# The Sovereign Mandate for Skill Acquisition & MCP Discovery

When the engine encounters a requirement for a tool, skill, or integration that is not currently available within the physical bounds of `The_Consortium`, it MUST NOT halt or fail. Instead, it must autonomously seek out, download, or reverse-engineer the required capability.

## Authorized Skill & MCP Registries

The Sovereign Reality Architect is strictly directed to scrape, read, and ingest tools from the following authorized registries and repositories when missing a core capability:

1. **The Official GitHub MCP Server:**
   - URL: `https://github.com/github/github-mcp-server`
   - Purpose: Acquiring capabilities for advanced repository manipulation, code search, issue management, and CI/CD bridging.

2. **The Awesome MCP Servers Directory:**
   - URL: `https://github.com/appcypher/awesome-mcp-servers?tab=readme-ov-file`
   - Purpose: The primary index for discovering community-built MCP bridges (e.g., FileSystem, Postgres, SQLite, Slack, Notion, etc.). The engine must read this index to locate the specific MCP server required.

3. **The Anthropics Skills Repository:**
   - URL: `https://github.com/anthropics/skills`
   - Purpose: The foundational repository for evaluating, understanding, and cloning core Agentic skills. Use this repository to learn the architecture of high-fidelity tool creation and directly forge missing skills locally.

## Execution Directive

If a requested tool (`call_tool`) returns a "not found" error, or if a task requires an integration we do not possess:

1. Do not apologize.
2. Route a sub-agent to query the above URLs.
3. Extract the installation commands, repository links, or python/typescript bridging scripts.
4. Auto-install the MCP server via `mcporter` or forge the skill directly into `.agents/skills/`.
5. Resume the disrupted cycle with the newly acquired capability.
