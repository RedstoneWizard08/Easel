use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        crate::util::create_table(
            m,
            "submissions",
            &[("id", ColType::PkAuto), ("s3_file_id", ColType::String)],
            &[("user", ""), ("assignment", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "submissions").await
    }
}
