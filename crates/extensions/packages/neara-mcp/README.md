# neara-mcp — NEARA hosted-MCP extension

NEARA (https://neara.fun) is a token launchpad on NEAR mainnet. This extension
connects its keyless MCP server. Extension id: `neara`. A **data-only
package**: a hosted-MCP extension declares `[mcp]` in place of a `[runtime]`
section; past activation, discovered tools are ordinary tool surfaces.

- **Surfaces:** `[mcp]` hosted server + 1 statically pinned read-only tool
  (`neara.list_tokens`), replaced by the live catalog after `tools/list`
  discovery (10 tools: reads, quotes, and transaction builders).
- **Credentials:** none. Read tools use public chain data; build tools return
  unsigned transactions plus a `sign_url` on neara.fun where the account owner
  approves them in their own wallet. No key is ever handled by IronClaw, the
  extension, or the server.
- **Effects:** `network` only.
- **Docs:** https://neara.fun/docs#agents · https://neara.fun/llms.txt
