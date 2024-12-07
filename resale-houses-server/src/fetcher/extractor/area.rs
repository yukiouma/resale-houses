async fn extract_area_pages() -> anyhow::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use bollard::{image::ListImagesOptions, Docker};
    #[tokio::test]
    async fn test_extract_area_pages() -> anyhow::Result<()> {
        let docker = Docker::connect_with_local_defaults()?;
        let images = &docker
            .list_images(Some(ListImagesOptions::<String> {
                all: true,
                ..Default::default()
            }))
            .await
            .unwrap();

        for image in images {
            println!("-> {:?}", image);
        }
        Ok(())
    }
}
