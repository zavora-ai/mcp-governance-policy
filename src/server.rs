use adk_mcp_sdk::{HealthCheck, HealthStatus};
use chrono::Utc;
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::store::PolicyStore;
use crate::types::*;

#[derive(Clone)]
pub struct GovernancePolicyServer {
    store: PolicyStore,
}

impl GovernancePolicyServer {
    pub fn new(store: PolicyStore) -> Self {
        Self { store }
    }

    fn evaluate_rules(&self, action: &str, risk_domain: &RiskDomain, actor: &str) -> (PolicyDecision, String, String) {
        // Built-in policy rules — in production these come from OPA/Cedar
        match risk_domain {
            RiskDomain::FinancialAction => (
                PolicyDecision::Review,
                "pol_financial_actions".into(),
                "Financial actions require human approval".into(),
            ),
            RiskDomain::ProductionDeploy => (
                PolicyDecision::Review,
                "pol_production_deploy".into(),
                "Production deployments require release governance".into(),
            ),
            RiskDomain::IdentityAction => (
                PolicyDecision::Pause,
                "pol_identity_security".into(),
                "Identity/security actions require verification".into(),
            ),
            RiskDomain::CredentialAccess => (
                PolicyDecision::Allow,
                "pol_credential_access".into(),
                format!("Credential access allowed for actor: {}", actor),
            ),
            RiskDomain::DataExport => (
                PolicyDecision::Review,
                "pol_data_export".into(),
                "Data exports require approval for compliance".into(),
            ),
            _ => (
                PolicyDecision::Allow,
                "pol_default".into(),
                format!("Action '{}' allowed by default policy", action),
            ),
        }
    }
}

// --- Input types ---

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct EvaluatePolicyInput {
    pub action: String,
    pub actor: String,
    pub risk_domain: RiskDomain,
    #[serde(default)] pub context: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct RequestApprovalInput {
    pub action: String,
    pub actor: String,
    pub risk_domain: RiskDomain,
    pub reason: String,
    #[serde(default)] pub assigned_to: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ListApprovalQueueInput {
    #[serde(default)] pub status: Option<ApprovalStatus>,
    #[serde(default)] pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ResolveApprovalInput {
    pub approval_id: String,
    pub decision: ApprovalStatus,
    pub resolved_by: String,
    #[serde(default)] pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct SimulatePolicyPackInput {
    pub actions: Vec<SimulatedAction>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct SimulatedAction {
    pub action: String,
    pub actor: String,
    pub risk_domain: RiskDomain,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct GetPolicyDecisionsInput {
    #[serde(default)] pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct CreatePolicyExceptionInput {
    pub policy_id: String,
    pub reason: String,
    pub duration_hours: u32,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ExportAuditPackInput {
    #[serde(default)] pub limit: Option<usize>,
}

// --- Tool implementations ---

#[tool_router(server_handler)]
impl GovernancePolicyServer {
    #[tool(description = "Evaluate policy — decide allow, block, review, or pause")]
    async fn evaluate_policy(&self, Parameters(i): Parameters<EvaluatePolicyInput>) -> String {
        let (decision, policy_id, reason) = self.evaluate_rules(&i.action, &i.risk_domain, &i.actor);
        let eval = PolicyEvaluation {
            decision_id: format!("dec_{}", Uuid::new_v4().simple()),
            decision: decision.clone(),
            policy_id, action: i.action, actor: i.actor,
            risk_domain: i.risk_domain, reason,
            conditions: vec![],
            evaluated_at: Utc::now(),
        };
        self.store.record_decision(eval.clone()).await;
        serde_json::to_string_pretty(&eval).unwrap()
    }

    #[tool(description = "Open a HITL approval task")]
    async fn request_approval(&self, Parameters(i): Parameters<RequestApprovalInput>) -> String {
        let task = ApprovalTask {
            approval_id: format!("appr_{}", Uuid::new_v4().simple()),
            action: i.action, actor: i.actor, risk_domain: i.risk_domain,
            reason: i.reason, status: ApprovalStatus::Pending,
            assigned_to: i.assigned_to, decision_id: None,
            resolved_by: None, resolved_at: None,
            created_at: Utc::now(),
            expires_at: Some(Utc::now() + chrono::Duration::hours(24)),
        };
        self.store.put_approval(task.clone()).await;
        serde_json::to_string_pretty(&task).unwrap()
    }

    #[tool(description = "Show pending approvals by owner/risk/domain")]
    async fn list_approval_queue(&self, Parameters(i): Parameters<ListApprovalQueueInput>) -> String {
        let tasks = self.store.list_approvals(i.status.as_ref(), i.limit.unwrap_or(20)).await;
        serde_json::to_string_pretty(&tasks).unwrap()
    }

    #[tool(description = "Approve, reject, or request changes")]
    async fn resolve_approval(&self, Parameters(i): Parameters<ResolveApprovalInput>) -> String {
        match self.store.get_approval(&i.approval_id).await {
            Some(mut task) => {
                task.status = i.decision;
                task.resolved_by = Some(i.resolved_by);
                task.resolved_at = Some(Utc::now());
                self.store.put_approval(task.clone()).await;
                serde_json::to_string_pretty(&task).unwrap()
            }
            None => format!("Approval not found: {}", i.approval_id),
        }
    }

    #[tool(description = "Test policy changes against fixtures or history")]
    async fn simulate_policy_pack(&self, Parameters(i): Parameters<SimulatePolicyPackInput>) -> String {
        let results: Vec<_> = i.actions.iter().map(|a| {
            let (decision, policy_id, reason) = self.evaluate_rules(&a.action, &a.risk_domain, &a.actor);
            serde_json::json!({
                "action": a.action, "actor": a.actor, "risk_domain": a.risk_domain,
                "decision": decision, "policy_id": policy_id, "reason": reason,
            })
        }).collect();
        serde_json::to_string_pretty(&serde_json::json!({
            "simulation_id": format!("sim_{}", Uuid::new_v4().simple()),
            "results": results, "total": results.len(),
        })).unwrap()
    }

    #[tool(description = "Read recent allow/block/review outcomes")]
    async fn get_policy_decisions(&self, Parameters(i): Parameters<GetPolicyDecisionsInput>) -> String {
        let decisions = self.store.get_decisions(i.limit.unwrap_or(20)).await;
        serde_json::to_string_pretty(&decisions).unwrap()
    }

    #[tool(description = "Request time-bound policy exception")]
    async fn create_policy_exception(&self, Parameters(i): Parameters<CreatePolicyExceptionInput>) -> String {
        let now = Utc::now();
        let exc = PolicyException {
            exception_id: format!("exc_{}", Uuid::new_v4().simple()),
            policy_id: i.policy_id, reason: i.reason,
            granted_by: None, status: ApprovalStatus::Pending,
            valid_from: now,
            valid_until: now + chrono::Duration::hours(i.duration_hours as i64),
            created_at: now,
        };
        self.store.put_exception(exc.clone()).await;
        serde_json::to_string_pretty(&exc).unwrap()
    }

    #[tool(description = "Export governance evidence for compliance")]
    async fn export_audit_pack(&self, Parameters(i): Parameters<ExportAuditPackInput>) -> String {
        let decisions = self.store.get_decisions(i.limit.unwrap_or(100)).await;
        let approvals = self.store.list_approvals(None, i.limit.unwrap_or(100)).await;
        let exceptions = self.store.list_exceptions().await;
        serde_json::to_string_pretty(&serde_json::json!({
            "pack_id": format!("audit_{}", Uuid::new_v4().simple()),
            "exported_at": Utc::now(),
            "decisions": decisions.len(),
            "approvals": approvals.len(),
            "exceptions": exceptions.len(),
            "policy_decisions": decisions,
            "approval_tasks": approvals,
            "policy_exceptions": exceptions,
        })).unwrap()
    }
}

#[async_trait::async_trait]
impl HealthCheck for GovernancePolicyServer {
    async fn check_health(&self) -> HealthStatus {
        HealthStatus {
            healthy: true,
            message: Some("operational".into()),
            latency_ms: Some(1),
        }
    }
}
