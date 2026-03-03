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
use std::io::Cursor;
use image::ImageReader;
use image::{imageops, DynamicImage, ImageFormat};

use anyhow::Result;

// TODO this can be made much nicer by defining a custom
// openapi type. Can have static type checking all the way thru,
// api user can be even warned when they pass, for example,
// task id as media id.
// but that is difficult and I dont want to do it now.

macro_rules! media_task_id {
    ($id:expr) => {
        format!("bardak-media-task--{}--", $id)
    };
}

macro_rules! saved_media_id {
    ($id:expr) => {
        format!("bardak-saved-media--{:x}--", $id)
    };
}

macro_rules! saved_media_filename {
    ($id:expr) => {
        format!("{}.dat", saved_media_id!($id))
    };
}

macro_rules! thumb_id {
    ($id:expr) => {
        format!("bardak-thumb--{:x}--", $id)
    };
}

macro_rules! thumb_filename {
    ($id:expr) => {
        format!("{}.dat", thumb_id!($id))
    };
}


/// Save media to disk from reader
/// and launch thumbnailer task
pub async fn add_media<R>(
        mut reader: R,
        config: &Config,
    ) -> Result<schema::ResAddMedia>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let id = Uuid::new_v4();
    // TODO collision avoidance? Maybe irrelevant with v4
    // but if switch to v7?

    let media_id = save_media(
        &mut reader,
        config,
        ).await?;
    
    //tokio::spawn(
    //    make_thumbs(&config, media_id.clone())
    //);

    Ok(schema::ResAddMedia {
        task_id: media_task_id!(id).to_string(),
        media_id: media_id,
    })
}

pub async fn make_thumbs(
    config: &Config,
    filename: String,
    ) {
    for counter in 0..5 {
        println!("Background task running... count: {}", counter);
        time::sleep(time::Duration::from_secs(1)).await;
    }
}

/// Generate thumbs with `image` crate
/// (avif, webp, jpeg)
pub async fn thumbs_image(
    filename: String,
    config: &Config,
    ) -> Result<()> {

    let path_in: PathBuf = [
        config.media_save_dir.clone(),
        filename.clone().into(),
        ].iter().collect();

    let path_large: PathBuf = [
        config.media_save_dir.clone(),
        filename.clone().into(),
        ].iter().collect();

    let img = ImageReader::open(path_in)?.decode()?;

    for size in [&config.image_size_large, &config.image_size_medium] {
        let sized = imageops::resize(
            &img, size.0, size.1,
            imageops::FilterType::Lanczos3
            );

        // TODO anyhow context

        sized.save_with_format(
            format!("fubar{}.jpeg", size.0).to_string(),
            ImageFormat::Jpeg
            )?;
    }

    Ok(())

}

/// Consume media reader while writing to disk
/// and calculating hash.
/// Renames saved file to match media id,
/// return media id.
async fn save_media<R>(
        mut reader: R,
        config: &Config,
        ) -> anyhow::Result<String>
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

    let media_id = saved_media_id!(hash);
    

    let path_save: PathBuf = [
        config.media_save_dir.clone(),
        saved_media_filename!(hash).into(),
        ].iter().collect();
    
    // TODO unwrap
    rename(
        path_upload,
        path_save,
        ).await.unwrap();
    
    file.sync_all().await?; // Ensure written to disk

    Ok(media_id)
}

