use crate::schema;
use sqlx::Error;
use uuid::Uuid;


pub async fn add_media(
        payload: Vec<u8>,
        ) -> Result<schema::ResAddMedia, sqlx::Error>
{
    let id = Uuid::new_v4();
    // TODO collision avoidance? Maybe irrelevant with v4
    // but if switch to v7?

    Ok(schema::ResAddMedia {
        media_id: id.to_string()
    })
}

