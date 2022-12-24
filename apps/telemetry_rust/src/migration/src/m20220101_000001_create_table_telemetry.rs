use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TelemetryRust::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TelemetryRust::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TelemetryRust::RequestId).integer().not_null())
                    .col(ColumnDef::new(TelemetryRust::Code).string().not_null())
                    .col(ColumnDef::new(TelemetryRust::Property).string().not_null())
                    .col(ColumnDef::new(TelemetryRust::Value).string())
                    .col(ColumnDef::new(TelemetryRust::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TelemetryRust::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum TelemetryRust {
    Table,
    Id,
    RequestId,
    Code,
    Property,
    Value,
    CreatedAt,
}
