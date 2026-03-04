use crate::schema;
use uuid::Uuid;
use tokio;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use sha2::Sha256;
use sha2::Digest;
use crate::config::Config;
use std::path::PathBuf;
use tokio::fs::rename;
use std::sync::Arc;
use image::ImageReader;
use image::{imageops, ImageFormat};
use image::GenericImageView;

use anyhow::Result;
use anyhow::Context;

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

macro_rules! upload_filename {
    ($id:expr) => {
        format!("{}.dat", $id)
    };
}

macro_rules! saved_media_filename {
    ($id:expr) => {
        format!("{}.dat", $id)
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
        config: Arc<Config>,
    ) -> Result<schema::ResAddMedia>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let task_id = media_task_id!(Uuid::new_v4()).to_string();
    // TODO collision avoidance? Maybe irrelevant with v4
    // but if switch to v7?

    let media_id = save_media(
        &config,
        &mut reader,
        &task_id,
        ).await?;

    // here we clone arc to config because this goes in a different
    // thread (not necesarily os thread -- remember tokio)
    // and outlives this function
    
    tokio::spawn(
        error_logger(
            make_thumbs(config.clone(), media_id.clone())
        )
    );

    Ok(schema::ResAddMedia {
        task_id: task_id,
        media_id: media_id,
    })
}

pub async fn error_logger<Fut>(f: Fut)
where
    Fut: std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'static,
{
    eprintln!("pre");
    if let Err(e) = f.await {
        eprintln!("Task failed: {}", e);
    }
    eprintln!("post");
}

pub async fn make_thumbs(
    config: Arc<Config>,
    media_id: String,
    ) -> Result<()> {

    thumbs_image(&config, media_id).await
}

/// Generate thumbs with `image` crate
/// (avif, webp, jpeg)
pub async fn thumbs_image(
    config: &Config,
    media_id: String,
    ) -> Result<()> {

    println!("barfoo!");

    let path_in: PathBuf = [
        config.media_save_dir.clone(),
        saved_media_filename!(media_id).into(),
        ].iter().collect();

    // TODO figure out how to use anyhow context properly
    let img = ImageReader::open(&path_in)
        .context(format!("cant open path: {}", &path_in.display()))?
        .with_guessed_format()?
        .decode()?
        .to_rgb8()  // convert to rgb8 (i.e. strip alpha channel)
                    // because jpg doesn't support alpha channel
                    // and would fail
        ;

    for size in [&config.image_size_large, &config.image_size_medium, &config.image_size_thumb] {
        let (new_w, new_h) = zoom_to_fit(
            img.width(),
            img.height(),
            size.0,
            size.1
            );
        let sized = imageops::thumbnail(&img, new_w, new_h);

        // TODO anyhow context

        sized.save_with_format(
            format!("fubar{}.jpeg", size.0).to_string(),
            ImageFormat::Jpeg
            )?;
    }

    println!("fubar!");

    Ok(())

}

pub fn zoom_to_fit(
    original_width: u32,
    original_height: u32,
    target_width: u32,
    target_height: u32,
    ) -> (u32, u32) {

    let scale_x = target_width as f32 / original_width as f32;
    let scale_y = target_height as f32 / original_height as f32;
    let scale = scale_x.min(scale_y);
    
    let new_w = (original_width as f32 * scale).round() as u32;
    let new_h = (original_height as f32 * scale).round() as u32;

    (new_w, new_h)
}


/// Consume media reader while writing to disk
/// and calculating hash.
/// Renames saved file to match media id,
/// return media id.
async fn save_media<R>(
        config: &Config,
        mut reader: R,
        task_id: &String,
        ) -> anyhow::Result<String>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let path_upload: PathBuf = [
        config.media_upload_dir.clone(),
        upload_filename!(task_id).into(),
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
        saved_media_filename!(media_id).into(),
        ].iter().collect();
    
    // TODO unwrap
    rename(
        path_upload,
        path_save,
        ).await.unwrap();
    
    file.sync_all().await?; // Ensure written to disk

    Ok(media_id)
}

