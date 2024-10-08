mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = infrastructure::openai::openai_sample_request_repository::execute().await?;
    println!("Response content: {}", response);
    Ok(())
}
