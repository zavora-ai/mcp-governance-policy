# Governance Policy MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-governance-policy.svg)](https://crates.io/crates/mcp-governance-policy)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)

Policy evaluation, approvals, simulation, and audit evidence for [ADK-Rust Enterprise](https://enterprise.adk-rust.com). **The gatekeeper that should not be bypassable by downstream MCPs.**

<p align="center">
  <img src="https://raw.githubusercontent.com/zavora-ai/mcp-governance-policy/main/docs/architecture.svg" alt="Governance Policy MCP Architecture" width="800"/>
</p>

## Purpose

> Let agents and control-plane workflows evaluate policies before risky actions: tool writes, browser submits, payment intents, memory writes, release promotions, credential access, and protocol calls.

Every other MCP server in the ecosystem calls this server before executing high-risk operations. It decides **allow**, **block**, **review**, or **pause**.

## Tools (8)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `evaluate_policy` | Decide allow, block, review, or pause | Read-only |
| `request_approval` | Open a HITL approval task | External write |
| `list_approval_queue` | Show pending approvals by status | Read-only |
| `resolve_approval` | Approve, reject, or request changes | External write |
| `simulate_policy_pack` | Test policy changes against actions | Read-only |
| `get_policy_decisions` | Read recent allow/block/review outcomes | Read-only |
| `create_policy_exception` | Request time-bound policy exception | External write |
| `export_audit_pack` | Export governance evidence for compliance | Read-only |

## Risk Domains

| Domain | Default Decision | Rationale |
|--------|-----------------|-----------|
| `tool_write` | **Allow** | Low-risk internal writes |
| `external_write` | **Allow** | Standard external actions |
| `financial_action` | **Review** | Requires human approval |
| `identity_action` | **Pause** | Requires identity verification |
| `production_deploy` | **Review** | Release governance required |
| `memory_write` | **Allow** | Standard memory operations |
| `credential_access` | **Allow** | If scoped correctly |
| `data_export` | **Review** | Compliance check required |

## Example Prompts & Outputs

### Evaluate a financial action

**Prompt:** "Can this agent process a refund?"

**Tool:** `evaluate_policy`
```json
{ "action": "process_refund", "actor": "support_agent", "risk_domain": "financial_action" }
```

**Output:**
```json
{
  "decision_id": "dec_2e106e4d...",
  "decision": "review",
  "policy_id": "pol_financial_actions",
  "action": "process_refund",
  "actor": "support_agent",
  "risk_domain": "financial_action",
  "reason": "Financial actions require human approval",
  "conditions": [],
  "evaluated_at": "2026-05-23T12:58:26Z"
}
```

---

### Evaluate a low-risk action

**Prompt:** "Can agent_1 read configuration?"

```json
{ "action": "read_config", "actor": "agent_1", "risk_domain": "tool_write" }
```

**Output:**
```json
{
  "decision": "allow",
  "policy_id": "pol_default",
  "reason": "Action 'read_config' allowed by default policy"
}
```

---

### Request approval for a blocked action

**Prompt:** "Request manager approval for the refund"

**Tool:** `request_approval`
```json
{
  "action": "process_refund",
  "actor": "support_agent",
  "risk_domain": "financial_action",
  "reason": "Customer requested $89.99 refund",
  "assigned_to": "manager_jane"
}
```

**Output:**
```json
{
  "approval_id": "appr_e852cdb0...",
  "action": "process_refund",
  "status": "pending",
  "assigned_to": "manager_jane",
  "expires_at": "2026-05-24T12:58:26Z"
}
```

---

### Resolve an approval

**Prompt:** "Approve the refund request"

**Tool:** `resolve_approval`
```json
{
  "approval_id": "appr_e852cdb0...",
  "decision": "approved",
  "resolved_by": "manager_jane"
}
```

**Output:**
```json
{
  "approval_id": "appr_e852cdb0...",
  "status": "approved",
  "resolved_by": "manager_jane",
  "resolved_at": "2026-05-23T13:15:00Z"
}
```

---

### Simulate a policy pack

**Prompt:** "What would happen if these 3 actions were attempted?"

**Tool:** `simulate_policy_pack`
```json
{
  "actions": [
    { "action": "send_email", "actor": "agent_1", "risk_domain": "external_write" },
    { "action": "deploy_prod", "actor": "ci_bot", "risk_domain": "production_deploy" },
    { "action": "reset_password", "actor": "admin", "risk_domain": "identity_action" }
  ]
}
```

**Output:**
```json
{
  "simulation_id": "sim_7f2a...",
  "total": 3,
  "results": [
    { "action": "send_email", "decision": "allow", "policy_id": "pol_default" },
    { "action": "deploy_prod", "decision": "review", "policy_id": "pol_production_deploy" },
    { "action": "reset_password", "decision": "pause", "policy_id": "pol_identity_security" }
  ]
}
```

---

### Create a time-bound exception

**Prompt:** "We need a 4-hour exception for emergency refund batch processing"

**Tool:** `create_policy_exception`
```json
{
  "policy_id": "pol_financial_actions",
  "reason": "Emergency refund batch processing",
  "duration_hours": 4
}
```

**Output:**
```json
{
  "exception_id": "exc_9d74ab7b...",
  "policy_id": "pol_financial_actions",
  "status": "pending",
  "valid_from": "2026-05-23T12:58:26Z",
  "valid_until": "2026-05-23T16:58:26Z"
}
```

---

### Export audit pack

**Prompt:** "Export governance evidence for the compliance audit"

**Tool:** `export_audit_pack`
```json
{ "limit": 100 }
```

**Output:**
```json
{
  "pack_id": "audit_4f8a...",
  "exported_at": "2026-05-23T13:00:00Z",
  "decisions": 5,
  "approvals": 2,
  "exceptions": 1,
  "policy_decisions": [...],
  "approval_tasks": [...],
  "policy_exceptions": [...]
}
```

## Decision Flow

```
Action requested → evaluate_policy()
    │
    ├── ALLOW → proceed immediately
    ├── BLOCK → denied, log reason
    ├── REVIEW → request_approval() → wait for resolve_approval()
    └── PAUSE → requires verification before retry
```

## Integration with Other MCPs

| MCP | How It Calls Governance |
|-----|------------------------|
| **Credentials Vault** | Before `request_runtime_secret` and `rotate_credential` |
| **Artifact Store** | Before `write_artifact` with sensitive data class |
| **Session Memory** | Before `store_memory` with PII, `delete_memory` |
| **Environment** | Before `promote_build` and `scale_worker_pool` in production |
| **ADK-Payments** | Before every payment intent execution |

## Installation

```bash
git clone https://github.com/zavora-ai/mcp-governance-policy
cd mcp-governance-policy
cargo build --release
```

### MCP Client Config

```json
{
  "mcpServers": {
    "governance-policy": {
      "command": "/path/to/mcp-governance-policy"
    }
  }
}
```

Works with Claude Desktop, Kiro, Codex, Cursor, Windsurf, Antigravity, and Open Code.

## Contributing

PRs welcome. Run `cargo clippy` and `cargo fmt` before submitting.

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START -->
| [<img src="https://github.com/jkmaina.png" width="80px;" alt=""/><br /><sub><b>James Karanja Maina</b></sub>](https://github.com/jkmaina) |
|:---:|
<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Apache-2.0 — see [LICENSE](LICENSE) for details.

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

Built with ❤️ by [Zavora AI](https://zavora.ai)
