use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        crate::util::create_table(
            m,
            "assignments",
            &[
                ("id", ColType::PkAuto),
                ("name", ColType::String),
                ("description", ColType::StringNull),
                ("due", ColType::DateTimeNull),
                ("lock_at", ColType::DateTimeNull),
                ("lock_until", ColType::DateTimeNull),
                ("max_score", ColType::IntegerNull),
                ("is_quiz", ColType::BooleanNull),
            ],
            &[],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "assignments").await
    }
}
