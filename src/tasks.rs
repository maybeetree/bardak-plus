use crate::media;
use crate::db;
use crate::config::Config;
use crate::config::LoadedConfig;
use sqlx::sqlite::SqlitePool;
use anyhow::Result;
use uuid::Uuid;

use crate::schema;

/// Save media to disk from reader
/// and launch thumbnailer task
pub async fn add_media<R>(
        mut reader: R,
        config: &'static Config,
        lconfig: &'static LoadedConfig,
        pool: &'static SqlitePool,
    ) -> Result<schema::ResAddMedia>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let task_id = media::media_task_id(Uuid::new_v4());
    // TODO collision avoidance? Maybe irrelevant with v4
    // but if switch to v7?

    let media_id = media::save_media(
        &config.media_upload_dir,
        &config.media_save_dir,
        &mut reader,
        &task_id,
        ).await?;

    // here we clone arc to config because this goes in a different
    // thread (not necesarily os thread -- remember tokio)
    // and outlives this function
    
    db::start_thumbs(&pool, &media_id, lconfig.thumbspecs.specs.keys()).await?;
    
    tokio::spawn(
        error_logger(
            media::make_thumbs(&config, &lconfig, &pool, media_id.clone())
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


