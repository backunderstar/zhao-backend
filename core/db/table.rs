use common::AppResult;
use entity::prelude::*;
use sea_orm::{ConnectionTrait, DatabaseConnection, Schema};

//==================================================================================
//= 创建数据库表
//= create tables
pub async fn create_tables(db: &DatabaseConnection) -> AppResult<()> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let tables = vec![
        schema.create_table_from_entity(User).if_not_exists().take(),
        schema
            .create_table_from_entity(System)
            .if_not_exists()
            .take(),
        schema
            .create_table_from_entity(Article)
            .if_not_exists()
            .take(),
        schema
            .create_table_from_entity(UserArticle)
            .if_not_exists()
            .take(),
        schema.create_table_from_entity(Menu).if_not_exists().take(),
        schema
            .create_table_from_entity(UserMenu)
            .if_not_exists()
            .take(),
        schema
            .create_table_from_entity(Image)
            .if_not_exists()
            .take(),
    ];

    for table in tables {
        db.execute(builder.build(&table)).await?;
    }

    Ok(())
}
