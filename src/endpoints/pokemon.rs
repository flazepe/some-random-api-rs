use crate::{Pokedex, PokemonAbility, PokemonItem, PokemonMove, Requester};
use anyhow::Result;

pub struct PokemonEndpoint(pub(crate) Requester);

impl PokemonEndpoint {
    /// Look up Pokemon abilities
    pub async fn abilities<T: ToString>(&self, ability: T) -> Result<PokemonAbility> {
        self.0
            .request(
                "pokemon/abilities",
                Some(&[("ability", ability.to_string())]),
            )
            .await
    }

    /// Look up Pokemon items
    pub async fn items<T: ToString>(&self, item: T) -> Result<PokemonItem> {
        self.0
            .request("pokemon/items", Some(&[("item", item.to_string())]))
            .await
    }

    /// Look up Pokemon moves
    pub async fn moves<T: ToString>(&self, pokemon_move: T) -> Result<PokemonMove> {
        self.0
            .request("pokemon/moves", Some(&[("move", pokemon_move.to_string())]))
            .await
    }

    /// Look up Pokemons
    pub async fn pokedex<T: ToString>(&self, pokemon: T) -> Result<Pokedex> {
        self.0
            .request("pokemon/pokedex", Some(&[("pokemon", pokemon.to_string())]))
            .await
    }
}
