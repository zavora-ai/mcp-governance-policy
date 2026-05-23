use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDecision {
    Allow,
    Block,
    Review,
    Pause,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    ChangesRequested,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RiskDomain {
    ToolWrite,
    ExternalWrite,
    FinancialAction,
    IdentityAction,
    ProductionDeploy,
    MemoryWrite,
    CredentialAccess,
    DataExport,
}

/// Result of a policy evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluation {
    pub decision_id: String,
    pub decision: PolicyDecision,
    pub policy_id: String,
    pub action: String,
    pub actor: String,
    pub risk_domain: RiskDomain,
    pub reason: String,
    pub conditions: Vec<String>,
    pub evaluated_at: DateTime<Utc>,
}

/// An approval task in the queue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalTask {
    pub approval_id: String,
    pub action: String,
    pub actor: String,
    pub risk_domain: RiskDomain,
    pub reason: String,
    pub status: ApprovalStatus,
    pub assigned_to: Option<String>,
    pub decision_id: Option<String>,
    pub resolved_by: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// A time-bound policy exception.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyException {
    pub exception_id: String,
    pub policy_id: String,
    pub reason: String,
    pub granted_by: Option<String>,
    pub status: ApprovalStatus,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
