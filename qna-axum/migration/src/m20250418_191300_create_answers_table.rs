use sea_orm_migration::{prelude::*, schema::*};
use crate::m20250417_233200_create_questions_table::Question;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Answer::Table)
                    .if_not_exists()
                    .col(pk_auto(Answer::Id))
                    .col(string(Answer::Content))
                    .col(
                        ColumnDef::new(Answer::QuestionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-answer-question_id")
                            .from(Answer::Table, Answer::QuestionId)
                            .to(Question::Table, Question::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Answer::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Answer::UpdatedAt)
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
            .drop_table(Table::drop().table(Answer::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
#[iden(rename = "answers")]
pub enum Answer {
    Table,
    Id,
    Content,
    QuestionId,
    CreatedAt,
    UpdatedAt,
}
