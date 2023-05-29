use crate::{
    AnimalEndpoint, AnimuEndpoint, CanvasEndpoint, CanvasFilterEndpoint, CanvasMiscEndpoint,
    CanvasOverlayEndpoint, ChatBotEndpoint, FactEndpoint, ImageEndpoint, OthersEndpoint,
    PokemonEndpoint, PremiumEndpoint, Requester, WelcomeEndpoint,
};

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
    /// Canvas-related endpoints
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
    ///         .jail("url").await?,
    /// )?;
    /// ```
    pub canvas: CanvasEndpoint,
    /// Canvas-related endpoints
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
    /// An endpoint that sends random Pokemon information
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::Client;
    ///
    /// Client::new(None::<String>).pokemon.pokedex("pikachu").await?;
    /// ```
    pub pokemon: PokemonEndpoint,
    /// Premium-related endpoints
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
    ///         .premium
    ///         .petpet("url").await?,
    /// )?;
    /// ```
    pub premium: PremiumEndpoint,
    /// Welcome message (free)
    ///
    /// # Examples
    ///
    /// ```
    /// use some_random_api::Client;
    /// use std::fs::write;
    ///
    /// write(
    ///     "welcome.png",
    ///     Client::new(None::<String>)
    ///         .welcome
    ///         .image(
    ///             ...
    ///         ).await?,
    /// )?;
    /// ```
    pub welcome: WelcomeEndpoint,
}

impl Client {
    /// Constructs a new `Client`
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
