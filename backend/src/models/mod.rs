pub mod user;
pub mod user_provider;
pub mod provider_model;


#[macro_export]
macro_rules! set_timestamp_before_save {
    ($model:ty) => {
        use sea_orm::{
            ActiveModelBehavior, ConnectionTrait, DbErr, Set,
        };
        use chrono::Utc;

        #[async_trait::async_trait]
        impl ActiveModelBehavior for $model {
            async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
            where
                C: ConnectionTrait,
            {
                let now = Utc::now().into();
                self.updated_at = Set(now);
                if insert {
                    self.created_at = Set(now);
                }
                Ok(self)
            }
        }
    };
}