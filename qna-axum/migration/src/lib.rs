pub use sea_orm_migration::prelude::*;

mod m20250417_233200_create_questions_table;
mod m20250418_191300_create_answers_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250417_233200_create_questions_table::Migration),
            Box::new(m20250418_191300_create_answers_table::Migration),
        ]
    }
}
