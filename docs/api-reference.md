# API Reference

## evaluate_policy

Evaluate whether an action should be allowed, blocked, sent for review, or paused.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `action` | string | Yes | Action being attempted (e.g. `process_refund`) |
| `actor` | string | Yes | Who is attempting the action (agent ID, user ID) |
| `risk_domain` | enum | Yes | Risk classification of the action |
| `context` | object | No | Additional context for policy evaluation |

**Risk domains:** `tool_write`, `external_write`, `financial_action`, `identity_action`, `production_deploy`, `memory_write`, `credential_access`, `data_export`

**Returns:** `PolicyEvaluation` with decision: `allow`, `block`, `review`, or `pause`

---

## request_approval

Open a human-in-the-loop approval task when policy requires review.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `action` | string | Yes | Action requiring approval |
| `actor` | string | Yes | Who requested the action |
| `risk_domain` | enum | Yes | Risk domain |
| `reason` | string | Yes | Why approval is needed |
| `assigned_to` | string | No | Specific approver |

**Returns:** `ApprovalTask` with status `pending` and 24h expiry.

---

## list_approval_queue

Show pending approvals filtered by status.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `status` | enum | No | Filter: `pending`, `approved`, `rejected`, `changes_requested`, `expired` |
| `limit` | integer | No | Max results (default 20) |

---

## resolve_approval

Approve, reject, or request changes on a pending approval.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `approval_id` | string | Yes | Approval to resolve |
| `decision` | enum | Yes | `approved`, `rejected`, or `changes_requested` |
| `resolved_by` | string | Yes | Who made the decision |
| `reason` | string | No | Explanation |

---

## simulate_policy_pack

Test what decisions would be made for a batch of actions without executing them.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `actions` | array | Yes | List of `{action, actor, risk_domain}` objects |

**Returns:** Simulation results with decision per action.

---

## get_policy_decisions

Read recent policy evaluation outcomes.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `limit` | integer | No | Max results (default 20) |

---

## create_policy_exception

Request a time-bound override of a policy rule.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `policy_id` | string | Yes | Policy to override |
| `reason` | string | Yes | Justification |
| `duration_hours` | integer | Yes | How long the exception lasts |

**Returns:** Exception with `pending` status (requires grant).

---

## export_audit_pack

Bundle all governance evidence for compliance review.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `limit` | integer | No | Max items per category (default 100) |

**Returns:** Pack containing all decisions, approvals, and exceptions.
