#![allow(dead_code)]
use anyhow::{anyhow, Result};
use sqlb::HasFields;
use sqlx::postgres::PgRow;
use sqlx::FromRow;

use crate::model::ModelManager;

pub trait DbBmc {
    const TABLE: &'static str;
}

pub async fn create<MC, E>(mm: &ModelManager, data: E) -> Result<i32>
where
    MC: DbBmc,
    E: HasFields,
{
    let db = mm.db();

    let fields = data.not_none_fields();
    let (id,) = sqlb::insert()
        .table(MC::TABLE)
        .data(fields)
        .returning(&["id"])
        .fetch_one::<_, (i32,)>(db)
        .await?;

    Ok(id)
}

pub async fn get<MC, E>(mm: &ModelManager, id: i32) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    E: HasFields,
{
    let db = mm.db();

    let entity: E = sqlb::select()
        .table(MC::TABLE)
        .columns(E::field_names())
        .and_where("id", "=", id)
        .fetch_optional(db)
        .await?
        .ok_or(anyhow!(
            "Entity not found in table '{}', id: {}",
            MC::TABLE,
            id
        ))?;

    Ok(entity)
}

pub async fn get_many<MC, E>(mm: &ModelManager, ids: &[i32]) -> Result<Vec<E>>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let db = mm.db();

    let query = format!("SELECT * FROM {} WHERE id = ANY($1)", MC::TABLE);

    let entities: Vec<E> = sqlx::query_as(&query).bind(ids).fetch_all(db).await?;

    Ok(entities)
}

pub async fn list<MC, E>(mm: &ModelManager) -> Result<Vec<E>>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    E: HasFields,
{
    let db = mm.db();

    let entities: Vec<E> = sqlb::select()
        .table(MC::TABLE)
        .columns(E::field_names())
        .order_by("id")
        .fetch_all(db)
        .await?;
    Ok(entities)
}

pub async fn update<MC, E>(mm: &ModelManager, id: i32, data: E) -> Result<()>
where
    MC: DbBmc,
    E: HasFields,
{
    let db = mm.db();

    let fields = data.not_none_fields();
    let count = sqlb::update()
        .table(MC::TABLE)
        .and_where("id", "=", id)
        .data(fields)
        .exec(db)
        .await?;

    if count == 0 {
        Err(anyhow!(
            "Entity not found in table '{}', id: {}",
            MC::TABLE,
            id
        ))
    } else {
        Ok(())
    }
}

pub async fn delete<MC>(mm: &ModelManager, id: i32) -> Result<()>
where
    MC: DbBmc,
{
    let db = mm.db();

    let count = sqlb::delete()
        .table(MC::TABLE)
        .and_where("id", "=", id)
        .exec(db)
        .await?;

    if count == 0 {
        Err(anyhow!(
            "Entity not found in table '{}', id: {}",
            MC::TABLE,
            id
        ))?
    } else {
        Ok(())
    }
}
