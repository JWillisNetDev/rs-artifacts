use rs_artifacts::ArtifactClient;

const API_KEY: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6Impvc2h1YS5tLndpbGxpczhAZ21haWwuY29tIiwicGFzc3dvcmRfY2hhbmdlZCI6IiJ9.hCP2ktTddN4lesYdWGuQlUBXHIzIGwZt_3h9wGxCcbo";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = ArtifactClient::new(API_KEY);

    let moved = client.character("Freakazoid").mov(1, 0).await?;
    println!("Moved: {:?}", moved);

    Ok(())
}
