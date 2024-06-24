use anyhow::Result;
use std::collections::HashMap;
use tokio::fs;

pub async fn load_html(html_dir: &str) -> Result<HashMap<String, String>> {
    let mut html_file_map = HashMap::new();

    let mut files = fs::read_dir(html_dir).await?;

    while let Some(file) = files.next_entry().await? {
        let html_body = fs::read_to_string(file.path()).await?;
        let file_name = file.file_name().into_string().unwrap();

        // Cut off the ".html" so it can be called by the file name without the ext
        let file_name = file_name.split(".").nth(0).unwrap().to_string();

        html_file_map.insert(file_name, html_body);
    }

    println!("{:?}", html_file_map);
    Ok(html_file_map)
}
