use std::io;

use tokio::{fs, io::AsyncWriteExt as _};

pub async fn write_clip_to_file(filename: String, content: &Vec<u8>) {
    let destination: String = format!(
        "/home/otto/Videos/Clips/{}", // Replace harcoded value when config module implemented
        filename
    );
    let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
    let _ = saved_file.write_all(content).await.unwrap();
}

pub async fn remove_clip_file(filename: String) -> io::Result<()> {
    let path = format!("/home/otto/Videos/Clips/{}", filename);
    fs::remove_file(path).await
}
