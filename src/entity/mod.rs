use std::time::Instant;
use uuid::Uuid;

/// Base generic type to represent the identity of some entity
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity<ID> {
    pub id: ID,
    pub version: Uuid,
    pub created: Instant,
    pub updated: Instant,
}

/// Base generic type to represent some entity
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity<ID, DATA> {
    pub identity: Identity<ID>,
    pub data: DATA,
}
