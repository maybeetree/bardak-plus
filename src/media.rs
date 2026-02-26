use crate::schema;
use uuid::Uuid;
use tokio;
use tokio::time;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use sha2::Sha256;
use sha2::Digest;
use crate::config::Config;
use std::path::PathBuf;
use tokio::fs::rename;

/// Save media to disk from reader
/// and launch thumbnailer task
pub async fn add_media<R>(
        mut reader: R,
        config: &Config,
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
    save_media(
        &mut reader,
        config,
        ).await.unwrap();

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

/// Consume media reader while writing to disk
/// and calculating hash.
/// Renames saved file to match media id,
/// return media id.
async fn save_media<R>(
        mut reader: R,
        config: &Config,
        ) -> anyhow::Result<()>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let path_upload: PathBuf = [
        config.media_upload_dir.clone(),
        format!("{}.dat", uuid::Uuid::new_v4()).into(),
        ].iter().collect();

    let mut file = tokio::fs::File::create(
        path_upload.clone(),
        ).await?;
    
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
    

    let path_save: PathBuf = [
        config.media_save_dir.clone(),
        format!("{:x}.dat", hash).into(),
        ].iter().collect();
    
    // TODO unwrap
    rename(
        path_upload,
        path_save,
        ).await.unwrap();
    
    file.sync_all().await?; // Ensure written to disk
    Ok(())
}

