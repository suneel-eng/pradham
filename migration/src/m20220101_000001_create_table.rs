use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager.create_table(
            Table::create()
            .table(Users::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Users::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key()
            )
            .col(
                ColumnDef::new(Users::Name)
                .string().not_null()
            )
            .col(
                ColumnDef::new(Users::Username)
                .string()
                .not_null()
            )
            .col(
                ColumnDef::new(Users::Email)
                .string()
                .not_null()
            )
            .col(
                ColumnDef::new(Users::Gender)
                .string()
                .not_null()
            )
            .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Name,
     // #[sea_orm(iden = "username")] // Renaming the identifier
    Username,
    Email,
    Gender
}
