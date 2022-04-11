use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
            .table(Post::Table)
            .col(ColumnDef::new(Post::Id).integer().primary_key().auto_increment().not_null())
            .col(ColumnDef::new(Post::Title).string().not_null())
            .col(ColumnDef::new(Post::Text).string().not_null())
            .to_owned()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop()
        .table(Post::Table)
        .to_owned()).await
    }
}
#[derive(Iden)]
pub enum Post{
    Table,
    Id,
    Title,
    Text,
}

