# Security Model

## Core Principle: Non-Bypassable Gatekeeper

The Governance Policy MCP is the single point of policy enforcement for the entire ADK-Rust Enterprise platform. No downstream MCP server should be able to execute a governed action without first receiving a policy decision from this server.

## Threat Model

| Threat | Mitigation |
|--------|-----------|
| Agent bypasses policy check | All governed MCPs must call evaluate_policy before execution |
| Approval forged | Approval IDs are UUIDs, resolved_by is audited |
| Exception abused | Exceptions are time-bound and require explicit grant |
| Policy decisions tampered | Append-only decision log, export for audit |
| Unauthorized approval resolution | resolve_approval requires actor identity |
| Stale approvals used | 24-hour expiry on all approval tasks |

## Access Control

| Action | Required Permission |
|--------|-------------------|
| `evaluate_policy` | Any authenticated actor |
| `request_approval` | Any authenticated actor |
| `list_approval_queue` | Operator or approver role |
| `resolve_approval` | Assigned approver or admin |
| `simulate_policy_pack` | Policy author or admin |
| `get_policy_decisions` | Operator or auditor |
| `create_policy_exception` | Admin (pending grant) |
| `export_audit_pack` | Auditor or compliance role |

## Integration Requirements

Every MCP server that performs governed actions MUST:

1. Call `evaluate_policy` before execution
2. Respect the decision (allow/block/review/pause)
3. If `review`: call `request_approval` and wait
4. Log the `decision_id` in its own audit trail
5. Never cache policy decisions beyond the current request
