use crate::{
    AnimalEndpoint, AnimuEndpoint, CanvasEndpoint, CanvasFilterEndpoint, CanvasMiscEndpoint,
    CanvasOverlayEndpoint, ChatBotEndpoint, FactEndpoint, ImageEndpoint, OthersEndpoint,
    PokemonEndpoint, PremiumEndpoint, Requester, WelcomeEndpoint,
};

/// A struct with functions used for interacting with Some Random API
pub struct Client {
    /// An endpoint that sends a random image and fact of an animal
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::Client;
    ///
    /// Client::new(None::<String>).animal.bird().await?;
    /// ```
    pub animal: AnimalEndpoint,
    /// An endpoint that sends random things related to anime
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::Client;
    ///
    /// Client::new(None::<String>).animu.wink().await?;
    /// ```
    pub animu: AnimuEndpoint,
    /// An endpoint for generating images manipulated using Canvas
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::Client;
    /// use std::fs::write;
    ///
    /// write(
    ///     "jail.png",
    ///     Client::new(None::<String>)
    ///         .canvas
    ///         .overlay
    ///         .jail("avatar url").await?,
    /// )?;
    /// ```
    pub canvas: CanvasEndpoint,
    /// An endpoint for interactiong with the ChatBot
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::Client;
    /// use std::fs::write;
    ///
    /// Client::new(Some("xxxxxxxxxx")).chatbot.chatbot("Hello there").await?;
    /// ```
    pub chatbot: ChatBotEndpoint,
    /// An endpoint that sends a random fact of an animal
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::Client;
    ///
    /// Client::new(None::<String>).fact.bird().await?;
    /// ```
    pub fact: FactEndpoint,
    /// An endpoint that sends a random image of an animal
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::Client;
    ///
    /// Client::new(None::<String>).image.bird().await?;
    /// ```
    pub image: ImageEndpoint,
    /// An endpoint that sends other random stuff
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::Client;
    ///
    /// Client::new(None::<String>).others.joke().await?;
    /// ```
    pub others: OthersEndpoint,
    /// An endpoint that sends Pokemon information
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::Client;
    ///
    /// Client::new(None::<String>).pokemon.pokedex("pikachu").await?;
    /// ```
    pub pokemon: PokemonEndpoint,
    /// An endpoint for premium users
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::Client;
    /// use std::fs::write;
    ///
    /// write(
    ///     "petpet.gif",
    ///     Client::new(None::<String>)
    ///         .premium
    ///         .petpet("avatar url").await?,
    /// )?;
    /// ```
    pub premium: PremiumEndpoint,
    /// An endpoint for generating free welcome images
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::{Client, WelcomeImage, WelcomeImageBackground, WelcomeImageTextColor};
    /// use std::fs::write;
    ///
    /// write(
    ///     "welcome.png",
    ///     Client::new(None::<String>)
    ///         .welcome
    ///         .image(
    ///             WelcomeImage::new(
    ///                 "username",
    ///                 "discriminator"
    ///                 "avatar url",
    ///                 "guild name",
    ///                 100, // Member count
    ///             )
    ///             .set_background(WelcomeImageBackground::Stars)
    ///             .set_text_color(WelcomeImageTextColor::Red)
    ///             .set_leave(true);
    ///         ).await?,
    /// )?;
    /// ```
    pub welcome: WelcomeEndpoint,
}

impl Client {
    /// Constructs a new [`Client`]
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::Client;
    ///
    /// // A client without an API key
    /// Client::new(None::<String>);
    ///
    /// // A client with an API key
    /// Client::new(Some("xxxxxxxxxx"));
    /// ```
    pub fn new<T: ToString>(api_key: Option<T>) -> Self {
        let requester = Requester::new(api_key);

        Self {
            animal: AnimalEndpoint(requester.clone()),
            animu: AnimuEndpoint(requester.clone()),
            canvas: CanvasEndpoint {
                filter: CanvasFilterEndpoint(requester.clone()),
                misc: CanvasMiscEndpoint(requester.clone()),
                overlay: CanvasOverlayEndpoint(requester.clone()),
            },
            chatbot: ChatBotEndpoint(requester.clone()),
            fact: FactEndpoint(requester.clone()),
            image: ImageEndpoint(requester.clone()),
            others: OthersEndpoint(requester.clone()),
            pokemon: PokemonEndpoint(requester.clone()),
            premium: PremiumEndpoint(requester.clone()),
            welcome: WelcomeEndpoint(requester),
        }
    }
}
