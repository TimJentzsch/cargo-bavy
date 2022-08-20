use actix_files as fs;
use actix_web::{rt, App, HttpServer};
use anyhow::Result;

pub fn launch_game() -> Result<()> {
    rt::System::new().block_on(
        HttpServer::new(|| {
            App::new().service(fs::Files::new("/", "./wasm").index_file("index.html"))
        })
        .bind(("127.0.0.1", 4000))?
        .run(),
    )?;

    Ok(())
}
