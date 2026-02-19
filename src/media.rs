use crate::schema;
use sqlx::Error;
use uuid::Uuid;
use tokio::time;


pub async fn add_media(
        payload: Vec<u8>,
        ) -> Result<schema::ResAddMedia, sqlx::Error>
{
    let id = Uuid::new_v4();
    // TODO collision avoidance? Maybe irrelevant with v4
    // but if switch to v7?
    
    tokio::spawn(
        mytask()
    );

    Ok(schema::ResAddMedia {
        media_id: id.to_string()
    })
}

pub async fn mytask() {
    let mut counter = 0;
    loop {
        println!("Background task running... count: {}", counter);
        counter += 1;
        time::sleep(time::Duration::from_secs(5)).await;
    }
}


