use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Question::Table)
                    .if_not_exists()
                    .col(pk_auto(Question::Id))
                    .col(string(Question::Title))
                    .col(string(Question::Content))
                    .col(
                        ColumnDef::new(Question::Tags)
                            .text()
                            .default("[]")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Question::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Question::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Question::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
#[iden(rename = "questions")]
pub enum Question {
    Table,
    Id,
    Title,
    Content,
    Tags,
    CreatedAt,
    UpdatedAt,
}
