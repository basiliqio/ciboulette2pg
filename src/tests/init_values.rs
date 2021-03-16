use super::*;
use uuid::Uuid;

struct InitValueRes {
    id: Uuid,
}

const FAVORITE_COLOR_INIT_FILES: [&str; 3] = [
    include_str!("init_values_sql/init_favorite_color_1.sql"),
    include_str!("init_values_sql/init_favorite_color_2.sql"),
    include_str!("init_values_sql/init_favorite_color_3.sql"),
];

const PEOPLES_INIT_FILES: [&str; 3] = [
    include_str!("init_values_sql/init_peoples_1.sql"),
    include_str!("init_values_sql/init_peoples_2.sql"),
    include_str!("init_values_sql/init_peoples_3.sql"),
];

const ARTICLE_INIT_FILES: [&str; 3] = [
    include_str!("init_values_sql/init_article_1.sql"),
    include_str!("init_values_sql/init_article_2.sql"),
    include_str!("init_values_sql/init_article_3.sql"),
];

const COMMENT_INIT_FILES: [&str; 3] = [
    include_str!("init_values_sql/init_comment_1.sql"),
    include_str!("init_values_sql/init_comment_2.sql"),
    include_str!("init_values_sql/init_comment_3.sql"),
];

const PEOPLE_ARTICLE: &str = include_str!("init_values_sql/init_article_author.sql");

async fn init_favorite_colors(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>
) -> Vec<Uuid> {
    let mut favorite_color_id: Vec<Uuid> = Vec::with_capacity(3);
    for files in FAVORITE_COLOR_INIT_FILES.iter() {
        let id: (Uuid,) = sqlx::query_as(files)
            .fetch_one(&mut *transaction)
            .await
            .unwrap();
        favorite_color_id.push(id.0);
    }
    favorite_color_id
}

async fn init_peoples(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    favorite_colors: &Vec<Uuid>,
) -> Vec<Uuid> {
    let mut peoples_id: Vec<Uuid> = Vec::with_capacity(3);
    for (files, favorite_color_id) in PEOPLES_INIT_FILES.iter().zip(favorite_colors) {
        let id: (Uuid,) = sqlx::query_as(files)
            .bind(favorite_color_id)
            .fetch_one(&mut *transaction)
            .await
            .unwrap();
        peoples_id.push(id.0);
    }
    peoples_id
}

async fn init_articles(transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Vec<Uuid> {
    let mut article_id: Vec<Uuid> = Vec::with_capacity(3);
    for files in ARTICLE_INIT_FILES.iter() {
        let id: (Uuid,) = sqlx::query_as(files)
            .fetch_one(&mut *transaction)
            .await
            .unwrap();
        article_id.push(id.0);
    }
    article_id
}

async fn init_comments(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    peoples_id: &Vec<Uuid>,
    articles_id: &Vec<Uuid>,
) -> Vec<Uuid> {
    let mut comment_id: Vec<Uuid> = Vec::with_capacity(3);
    for (i, (files, article_id)) in COMMENT_INIT_FILES.iter().zip(articles_id).enumerate() {
        let id: (Uuid,) = sqlx::query_as(files)
            .bind(peoples_id[i % 2])
            .bind(article_id)
            .fetch_one(&mut *transaction)
            .await
            .unwrap();
        comment_id.push(id.0);
    }
    comment_id
}

async fn init_link_article_author(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    peoples_id: &Vec<Uuid>,
    articles_id: &Vec<Uuid>,
) -> Vec<Uuid> {
    let mut article_author_id: Vec<Uuid> = Vec::with_capacity(3);
    for (i, article_id) in articles_id.iter().enumerate() {
        let id: (Uuid,) = sqlx::query_as(PEOPLE_ARTICLE)
            .bind(peoples_id[i % 2])
            .bind(article_id)
            .fetch_one(&mut *transaction)
            .await
            .unwrap();
        article_author_id.push(id.0);
    }
    article_author_id
}

pub async fn init_values(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>
) -> BTreeMap<String, Vec<Uuid>> {
    let mut res: BTreeMap<String, Vec<Uuid>> = BTreeMap::new();
    let favorite_color_id = init_favorite_colors(&mut *transaction).await;
    let article_id = init_articles(&mut *transaction).await;
    let peoples_id = init_peoples(&mut *transaction, &&favorite_color_id).await;
    let comment_id = init_comments(&mut *transaction, &peoples_id, &article_id).await;
    let people_article = init_link_article_author(transaction, &peoples_id, &article_id).await;

    res.insert("favorite_colors".to_string(), favorite_color_id);
    res.insert("peoples".to_string(), peoples_id);
    res.insert("articles".to_string(), article_id);
    res.insert("comments".to_string(), comment_id);
    res.insert("people-article".to_string(), people_article);
    res
}