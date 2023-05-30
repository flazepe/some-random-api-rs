use crate::{
    AnimalEndpoint, AnimuEndpoint, CanvasEndpoint, CanvasFilterEndpoint, CanvasMiscEndpoint,
    CanvasOverlayEndpoint, ChatBotEndpoint, FactEndpoint, ImageEndpoint, OthersEndpoint,
    PokemonEndpoint, PremiumEndpoint, Requester, WelcomeEndpoint,
};

/// A struct with functions used for interacting with Some Random API
pub struct Client {
    pub animal: AnimalEndpoint,
    pub animu: AnimuEndpoint,
    pub canvas: CanvasEndpoint,
    pub chatbot: ChatBotEndpoint,
    pub fact: FactEndpoint,
    pub image: ImageEndpoint,
    pub others: OthersEndpoint,
    pub pokemon: PokemonEndpoint,
    pub premium: PremiumEndpoint,
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
