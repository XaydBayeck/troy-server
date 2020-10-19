#[cfg(test)]
mod test {
    use crate::article::ArticleCard;
    use futures::executor::block_on;
    use futures::TryStreamExt;
    use sqlx::sqlite::{SqlitePoolOptions, SqliteRow};
    use sqlx::Row;

    #[test]
    fn test_orm() -> Result<(), sqlx::Error> {
        block_on(test_orm_sub())
    }

    async fn test_orm_sub() -> Result<(), sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite:database/troy.db")
            .await?;

        let sql = "SELECT tbl_name FROM sqlite_master WHERE type = 'table'";

        let mut stream = sqlx::query(sql)
            .try_map(|row: SqliteRow| {
                let table: String = row.get("tbl_name");
                Ok(table)
            })
            .fetch(&pool);

        while let Some(table) = stream.try_next().await? {
            println!("table: {:?}", table);
        }

        let article_card = ArticleCard {
            title: String::from("测试文章"),
            time: String::from("2020-10-04"),
            category: String::from("test"),
            tags: vec![
                String::from("test"),
                String::from("rust"),
                String::from("json"),
            ],
            part: String::from("这是用来测试的文章。"),
        };

        let insert_sql = article_card.to_sql();
        println!("{}", insert_sql);

        let affect_rows = sqlx::query(&*insert_sql).execute(&pool).await?;
        println!("{:?}", affect_rows);

        Ok(())
    }
}
