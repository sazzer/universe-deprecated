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

impl<ID> Default for Identity<ID>
where
    ID: Default,
{
    fn default() -> Self {
        Identity {
            id: Default::default(),
            version: Uuid::new_v4(),
            created: Utc::now(),
            updated: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    // The sleeps before and after are enough that we can guarantee that the `created` and `updated` times
    // were in a period between them, even if we can't assert the exact values of them.
    #[test]
    fn test_default_identity() {
        let ten_millis = std::time::Duration::from_millis(10);

        let before = Utc::now();
        std::thread::sleep(ten_millis);

        let identity: Identity<u32> = Default::default();

        std::thread::sleep(ten_millis);
        let after = Utc::now();

        assert_that(&identity.id).is_equal_to(0);

        assert_that(&identity.created.timestamp_millis())
            .is_greater_than(before.timestamp_millis());
        assert_that(&identity.created.timestamp_millis()).is_less_than(after.timestamp_millis());

        assert_that(&identity.updated.timestamp_millis())
            .is_greater_than(before.timestamp_millis());
        assert_that(&identity.updated.timestamp_millis()).is_less_than(after.timestamp_millis());
    }
}
