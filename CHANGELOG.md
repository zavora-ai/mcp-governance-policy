# Changelog

## [1.0.0] - 2026-05-23

### Added

- **8 MCP tools** — evaluate_policy, request_approval, list_approval_queue, resolve_approval, simulate_policy_pack, get_policy_decisions, create_policy_exception, export_audit_pack
- **Policy evaluation engine** — built-in rules for financial, identity, production, credential, and data export actions
- **Approval workflow** — create, assign, resolve (approve/reject/changes_requested)
- **Policy simulation** — test policy packs against multiple actions
- **Time-bound exceptions** — request temporary policy overrides
- **Audit export** — bundle all decisions, approvals, and exceptions for compliance
- **Risk domains** — tool_write, external_write, financial_action, identity_action, production_deploy, memory_write, credential_access, data_export
- **rmcp 1.7** — latest MCP protocol SDK
