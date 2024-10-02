use mpris::{Mpris, MprisError};

#[async_std::main]
async fn main() -> Result<(), MprisError> {
    let mpris = Mpris::new().await?;
    for player in mpris.players().await? {
        println!("{:?}", player);
    }
    Ok(())
}
