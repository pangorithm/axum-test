use crate::sea_orm::DatabaseBackend;
use crate::sea_orm::Statement;
use sea_orm::{EnumIter, Iterable};

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    #[sea_orm(iden = "text")] // Renaming the identifier
    Text,
    Category,
}

#[derive(Iden, EnumIter)]
pub enum Category {
    #[iden = "Feed"]
    Feed,
    #[iden = "Story"]
    Story,
}

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the category type
        manager
            .get_connection()
            .execute(Statement::from_string(
                DatabaseBackend::Postgres,
                "CREATE TYPE category AS ENUM ('Feed', 'Story')".to_owned(),
            ))
            .await?;

        // Create the Post table
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())
                    .col(
                        ColumnDef::new(Post::Category)
                            .enumeration(Alias::new("category"), Category::iter()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-post_title")
                    .table(Post::Table)
                    .col(Post::Title)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;

        // Drop the category type
        manager
            .get_connection()
            .execute(Statement::from_string(
                DatabaseBackend::Postgres,
                "DROP TYPE category".to_owned(),
            ))
            .await?;

        Ok(())
    }
}
