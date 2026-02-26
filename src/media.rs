use crate::schema;
use sqlx::Error;
use uuid::Uuid;
use tokio;
use tokio::time;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use sha2::Sha256;
use sha2::Digest;


pub async fn add_media<R>(
    mut reader: R
    ) -> Result<schema::ResAddMedia, sqlx::Error>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let id = Uuid::new_v4();
    // TODO collision avoidance? Maybe irrelevant with v4
    // but if switch to v7?
    
    tokio::spawn(
        mytask()
    );

    // TODO
    save_media(&mut reader).await.unwrap();

    Ok(schema::ResAddMedia {
        media_id: id.to_string()
    })
}

pub async fn mytask() {
    for counter in 0..5 {
        println!("Background task running... count: {}", counter);
        time::sleep(time::Duration::from_secs(1)).await;
    }
}

async fn save_media<R>(mut reader: R) -> anyhow::Result<()>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let filename = format!("upload_{}.dat", uuid::Uuid::new_v4());
    let mut file = tokio::fs::File::create(&filename).await?;
    
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 1_048_576]; // 1MB chunks
                                           // TODO hardcoded
    
    loop {
        let bytes_read = reader.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        
        // Update hash incrementally
        hasher.update(&buffer[..bytes_read]);
        
        // Write chunk to disk
        file.write_all(&buffer[..bytes_read]).await?;
    }
    
    let hash = hasher.finalize();
    println!("SHA256: {:x}", hash); // Or return/store hash
    
    file.sync_all().await?; // Ensure written to disk
    Ok(())
}

