use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        crate::util::create_table(
            m,
            "enrollments",
            &[("id", ColType::PkAuto), ("until", ColType::DateTime)],
            &[("user", ""), ("course", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "enrollments").await
    }
}
