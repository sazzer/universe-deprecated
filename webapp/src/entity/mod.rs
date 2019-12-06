use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Base generic type to represent the identity of some entity
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity<ID> {
    pub id: ID,
    pub version: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

/// Base generic type to represent some entity
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity<ID, DATA> {
    pub identity: Identity<ID>,
    pub data: DATA,
}
