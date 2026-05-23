# Governance Policy MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-governance-policy.svg)](https://crates.io/crates/mcp-governance-policy)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)

Policy evaluation, approvals, simulation, and audit evidence for [ADK-Rust Enterprise](https://enterprise.adk-rust.com). The gatekeeper that should not be bypassable by downstream MCPs.

## Tools (8)

| Tool | Purpose |
|------|---------|
| `evaluate_policy` | Decide allow, block, review, or pause |
| `request_approval` | Open a HITL approval task |
| `list_approval_queue` | Show pending approvals |
| `resolve_approval` | Approve, reject, or request changes |
| `simulate_policy_pack` | Test policy changes against actions |
| `get_policy_decisions` | Read recent outcomes |
| `create_policy_exception` | Request time-bound exception |
| `export_audit_pack` | Export governance evidence |

## Example: Evaluate a payment action

**Prompt:** "Can this agent process a refund?"

```json
{ "action": "process_refund", "actor": "support_agent", "risk_domain": "financial_action" }
```

**Output:**
```json
{
  "decision_id": "dec_a1b2c3...",
  "decision": "review",
  "policy_id": "pol_financial_actions",
  "reason": "Financial actions require human approval"
}
```

## Example: Simulate policy pack

**Prompt:** "What would happen if these 3 actions were attempted?"

```json
{
  "actions": [
    { "action": "send_email", "actor": "agent_1", "risk_domain": "external_write" },
    { "action": "deploy_prod", "actor": "ci_bot", "risk_domain": "production_deploy" },
    { "action": "read_config", "actor": "agent_1", "risk_domain": "tool_write" }
  ]
}
```

**Output:**
```json
{
  "simulation_id": "sim_...",
  "results": [
    { "action": "send_email", "decision": "allow" },
    { "action": "deploy_prod", "decision": "review" },
    { "action": "read_config", "decision": "allow" }
  ]
}
```

## Risk Domains

| Domain | Default Policy |
|--------|---------------|
| `tool_write` | Allow |
| `external_write` | Allow |
| `financial_action` | Review (approval required) |
| `identity_action` | Pause (verification required) |
| `production_deploy` | Review (release governance) |
| `memory_write` | Allow |
| `credential_access` | Allow if scoped |
| `data_export` | Review (compliance check) |

## Installation

```json
{
  "mcpServers": {
    "governance-policy": {
      "command": "/path/to/mcp-governance-policy"
    }
  }
}
```

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START -->
| [<img src="https://github.com/jkmaina.png" width="80px;" alt=""/><br /><sub><b>James Karanja Maina</b></sub>](https://github.com/jkmaina) |
|:---:|
<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Apache-2.0

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem. Built with ❤️ by [Zavora AI](https://zavora.ai)
