use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents the identity of some resource that has previously been persisted to the database
#[derive(Debug, PartialEq, Clone)]
pub struct Identity<ID> {
    pub id: ID,
    pub version: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

/// Represents a resource that has previously been persisted to the database
#[derive(Debug, PartialEq, Clone)]
pub struct Entity<ID, DATA> {
    pub identity: Identity<ID>,
    pub data: DATA,
}
