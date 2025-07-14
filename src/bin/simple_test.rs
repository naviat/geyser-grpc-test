use geyser_grpc_test::simple_test::simple_geyser_test;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running Simple Geyser Test");
    println!("==========================");
    
    simple_geyser_test().await?;
    
    println!("Simple test completed!");
    Ok(())
} 