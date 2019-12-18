use chrono::{DateTime, Utc};

/// Represents the identity of some resource that has previously been persisted to the database
pub struct Identity<ID> {
    pub id: ID,
    pub version: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

/// Represents a resource that has previously been persisted to the database
pub struct Entity<ID, DATA> {
    pub identity: Identity<ID>,
    pub data: DATA,
}
