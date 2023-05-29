# some-random-api-rs

A crate for interacting with [Some Random API](https://some-random-api.com/).

## Getting Started

```rs
use some_random_api::{Client, WelcomeImage, WelcomeImageBackground};
use std::{error::Error, fs::write};
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new(None::<String>); // Creates a client without an API key

    // Look up definition of a word
    let definition = client.others.dictionary("resuscitate").await?;
    println!("{:#?}", definition);

    // Generate a welcome image
    let welcome_image = client
        .welcome
        .image(
            WelcomeImage::new(
                "Username",
                "0001",
                "https://cdn.discordapp.com/embed/avatars/0.png",
                "Guild Name",
                194,
            )
            .set_background(WelcomeImageBackground::Night),
        )
        .await?;

    // Save the welcome image to a file
    write("welcome.png", welcome_image)?;

    Ok(())
}
```

# More

Read the [documentation](https://docs.rs/some-random-api/latest/some-random-api/) for more information.
