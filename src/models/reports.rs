use super::ids::Base62Id;
use crate::models::ids::UserId;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "Base62Id")]
#[serde(into = "Base62Id")]
pub struct ReportId(pub u64);

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub id: ReportId,
    pub report_type: String,
    pub item_id: String,
    pub item_type: ItemType,
    pub reporter: UserId,
    pub body: String,
    #[serde(with = "crate::util::time_ser")]
    pub created: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ItemType {
    Project,
    Version,
    User,
    Unknown,
}

impl ItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemType::Project => "project",
            ItemType::Version => "version",
            ItemType::User => "user",
            ItemType::Unknown => "unknown",
        }
    }
}
