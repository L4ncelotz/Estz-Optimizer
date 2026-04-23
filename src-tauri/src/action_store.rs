use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionRecord {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub action_id: String,
    pub action_name: String,
    pub category: String,
    pub previous_value: Option<String>,
    pub new_value: Option<String>,
    pub registry_path: Option<String>,
    pub status: ActionStatus,
    pub reverted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ActionStatus {
    Applied,
    Failed,
    Reverted,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionStore {
    pub actions: Vec<ActionRecord>,
}

impl ActionStore {
    fn store_path() -> PathBuf {
        let app_data = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Estz");
        fs::create_dir_all(&app_data).ok();
        app_data.join("history.json")
    }

    pub fn load() -> Self {
        let path = Self::store_path();
        if path.exists() {
            let data = fs::read_to_string(&path).unwrap_or_else(|_| "{}".to_string());
            serde_json::from_str(&data).unwrap_or(ActionStore {
                actions: Vec::new(),
            })
        } else {
            ActionStore {
                actions: Vec::new(),
            }
        }
    }

    pub fn save(&self) {
        let path = Self::store_path();
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, data);
        }
    }

    pub fn log_action(
        &mut self,
        action_id: &str,
        action_name: &str,
        category: &str,
        previous_value: Option<String>,
        new_value: Option<String>,
        registry_path: Option<String>,
        status: ActionStatus,
    ) -> String {
        let id = Uuid::new_v4().to_string();
        let record = ActionRecord {
            id: id.clone(),
            timestamp: Utc::now(),
            action_id: action_id.to_string(),
            action_name: action_name.to_string(),
            category: category.to_string(),
            previous_value,
            new_value,
            registry_path,
            status,
            reverted: false,
        };
        self.actions.push(record);
        self.save();
        id
    }

    pub fn mark_reverted(&mut self, record_id: &str) {
        if let Some(record) = self.actions.iter_mut().find(|r| r.id == record_id) {
            record.reverted = true;
            record.status = ActionStatus::Reverted;
        }
        self.save();
    }

    pub fn get_history(&self) -> Vec<ActionRecord> {
        let mut sorted = self.actions.clone();
        sorted.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        sorted
    }

#[allow(dead_code)]
    pub fn get_unrevertable_actions(&self) -> Vec<ActionRecord> {
        self.actions
            .iter()
            .filter(|a| !a.reverted && a.status == ActionStatus::Applied && a.previous_value.is_some())
            .cloned()
            .collect()
    }

    pub fn clear_history(&mut self) {
        self.actions.clear();
        self.save();
    }
}
