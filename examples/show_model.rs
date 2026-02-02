use ollama_rs::OllamaClient;
use ollama_rs::types::show::ShowModelRequest;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = OllamaClient::default();
    let model = "qwen3:8b";
    
    println!("Requesting info for model: {}", model);
    let request = ShowModelRequest::new(model.to_string());
    let response = client.show_model(request).await?;
    
    println!("Model Info:");
    println!("  License: {}", response.license.as_deref().unwrap_or("N/A").lines().next().unwrap_or("")); // First line only
    println!("  Modelfile: {} bytes", response.modelfile.as_ref().map(|s| s.len()).unwrap_or(0));
    println!("  Parameters: {}", response.parameters.as_ref().map(|s| s.len()).unwrap_or(0));
    println!("  Template: {:?}", response.template.is_some());
    println!("  Details: {:?}", response.details);
    
    Ok(())
}
