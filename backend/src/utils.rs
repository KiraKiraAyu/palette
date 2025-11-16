use chrono::{DateTime, Utc};
use uuid::{NoContext, Timestamp, Uuid};

pub trait ToUuidV7 {
    fn to_uuid_v7(&self) -> Uuid;
}

impl ToUuidV7 for DateTime<Utc> {
    fn to_uuid_v7(&self) -> Uuid {
        let seconds = self.timestamp() as u64;
        let nanoseconds = self.timestamp_subsec_nanos();
        let timestamp = Timestamp::from_unix(NoContext, seconds, nanoseconds);
        Uuid::new_v7(timestamp)
    }
}