use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(ServedRequestsCount::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServedRequestsCount::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ServedRequestsCount::ApiName).string().not_null())
                    .col(ColumnDef::new(ServedRequestsCount::RequestsCount).big_unsigned().default(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(ServedRequestsCount::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ServedRequestsCount {
    Table,
    Id,
    ApiName,
    RequestsCount,
}
