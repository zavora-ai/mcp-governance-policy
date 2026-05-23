use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::types::*;

/// In-memory policy store.
#[derive(Clone)]
pub struct PolicyStore {
    decisions: Arc<RwLock<Vec<PolicyEvaluation>>>,
    approvals: Arc<RwLock<HashMap<String, ApprovalTask>>>,
    exceptions: Arc<RwLock<Vec<PolicyException>>>,
}

impl PolicyStore {
    pub fn new() -> Self {
        Self {
            decisions: Arc::new(RwLock::new(Vec::new())),
            approvals: Arc::new(RwLock::new(HashMap::new())),
            exceptions: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn record_decision(&self, eval: PolicyEvaluation) {
        self.decisions.write().await.push(eval);
    }

    pub async fn get_decisions(&self, limit: usize) -> Vec<PolicyEvaluation> {
        let d = self.decisions.read().await;
        d.iter().rev().take(limit).cloned().collect()
    }

    pub async fn put_approval(&self, task: ApprovalTask) {
        self.approvals.write().await.insert(task.approval_id.clone(), task);
    }

    pub async fn get_approval(&self, id: &str) -> Option<ApprovalTask> {
        self.approvals.read().await.get(id).cloned()
    }

    pub async fn list_approvals(&self, status: Option<&ApprovalStatus>, limit: usize) -> Vec<ApprovalTask> {
        let a = self.approvals.read().await;
        a.values()
            .filter(|t| status.is_none_or(|s| &t.status == s))
            .take(limit)
            .cloned()
            .collect()
    }

    pub async fn put_exception(&self, exc: PolicyException) {
        self.exceptions.write().await.push(exc);
    }

    pub async fn list_exceptions(&self) -> Vec<PolicyException> {
        self.exceptions.read().await.clone()
    }
}
