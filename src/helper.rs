pub async fn aws_client_or_default<C>(
    client: Option<C>,
    make_client: impl FnOnce(&aws_types::SdkConfig) -> C,
) -> C {
    match client {
        Some(c) => c,
        None => {
            let config = aws_config::load_from_env().await;
            make_client(&config)
        }
    }
}
