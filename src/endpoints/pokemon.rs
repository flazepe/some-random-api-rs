use crate::{Pokedex, PokemonAbility, PokemonItem, PokemonMove, Requester};
use anyhow::Result;

pub struct PokemonEndpoint(pub(crate) Requester);

impl PokemonEndpoint {
    /// Look up pokemon abilities
    pub async fn abilities<T: ToString>(&self, ability: T) -> Result<PokemonAbility> {
        self.0
            .request_with_query("pokemon/abilities", &[("ability", ability.to_string())])
            .await
    }

    /// Look up pokemon items
    pub async fn items<T: ToString>(&self, item: T) -> Result<PokemonItem> {
        self.0
            .request_with_query("pokemon/items", &[("item", item.to_string())])
            .await
    }

    /// Look up pokemon moves
    pub async fn moves<T: ToString>(&self, pokemon_move: T) -> Result<PokemonMove> {
        self.0
            .request_with_query("pokemon/moves", &[("move", pokemon_move.to_string())])
            .await
    }

    /// Look up pokemons
    pub async fn pokedex<T: ToString>(&self, pokemon: T) -> Result<Pokedex> {
        self.0
            .request_with_query("pokemon/pokedex", &[("pokemon", pokemon.to_string())])
            .await
    }
}