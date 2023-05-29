use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PokemonAbility {
    pub id: u64,
    pub name: String,
    pub generation: u64,
    pub effects: String,
    pub description: String,
    pub pokemons: Vec<PokemonAbilityPokemon>,
    pub descriptions: Vec<PokemonAbilityDescription>,
}

#[derive(Debug, Deserialize)]
pub struct PokemonAbilityPokemon {
    pub pokemon: String,
    pub hidden: bool,
}

#[derive(Debug, Deserialize)]
pub struct PokemonAbilityDescription {
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct PokemonItem {
    pub id: u64,
    pub name: String,
    pub effects: String,
    pub cost: u64,
    pub attributes: Vec<String>,
    pub category: String,
    pub sprite: String,
    pub descriptions: Vec<PokemonDescription>,
}

#[derive(Debug, Deserialize)]
pub struct PokemonMove {
    pub id: u64,
    pub name: String,
    pub generation: u64,
    pub effects: String,

    #[serde(rename = "type")]
    pub move_type: String,

    pub category: String,
    pub contest: String,
    pub pp: u64,
    pub power: u64,
    pub accuracy: u64,
    pub priority: u64,
    pub pokemon: Vec<String>,
    pub descriptions: Vec<PokemonDescription>,
}

#[derive(Debug, Deserialize)]
pub struct PokemonDescription {
    pub description: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Pokedex {
    pub name: String,
    pub id: String,

    #[serde(rename = "type")]
    pub pokemon_type: Vec<String>,

    pub species: Vec<String>,
    pub abilities: Vec<String>,
    pub height: String,
    pub weight: String,
    pub base_experience: String,
    pub gender: Vec<String>,
    pub egg_groups: Vec<String>,
    pub stats: PokedexStats,
    pub family: PokedexFamily,
    pub sprites: PokedexSprites,
    pub description: String,
    pub generation: String,
}

#[derive(Debug, Deserialize)]
pub struct PokedexStats {
    pub hp: String,
    pub attack: String,
    pub defense: String,
    pub sp_atk: String,
    pub sp_def: String,
    pub speed: String,
    pub total: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokedexFamily {
    pub evolution_stage: u64,
    pub evolution_line: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PokedexSprites {
    pub normal: String,
    pub animated: String,
}
