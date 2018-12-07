//!
//! Player lib
//! Handle Player and system
//!
extern crate lightql;

#[macro_use]
extern crate serde_derive;

use lightql::{
    Request,
    QlType,
    QlError
};

#[derive(Deserialize, Serialize, Debug)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
    dim: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct Player {
    uuid: String,
    position: Option<Position>
}

#[derive(Deserialize, Serialize, Debug)]
struct PlayersRequest {
    players: Vec<Player>
}

#[derive(Deserialize, Serialize, Debug)]
struct PlayerRequest {
    player: Player
}

/// Fetch all players from @uri
fn fetch_players(uri: &str) -> Result<Vec<Player>, QlError> {
    let query = "{ players { uuid, position { x, y, z, dim } } }";

    let players = Request::new(query, QlType::Query)
        .send::<PlayersRequest>(uri)?;
    Ok(players.players)
}

/// Fetch a player from the @uri with @uuid as index.
fn fetch_player(uuid: &str, uri: &str) -> Result<Player, QlError> {
    let mut hashmap = std::collections::HashMap::new();
    hashmap.insert(
        String::from("_uuid"),
        String::from(uuid),
    );
    let player_req = Request::from_path("ql/request_player.query")
        .unwrap()
        .prepare(Some(hashmap))?
        .send::<PlayerRequest>(uri)?;
    Ok(player_req.player)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_all_players() {
        let players = super::fetch_players("https://api.utopia-server.com/").unwrap();
        println!("{:#?}", players);
    }

    #[test]
    fn test_all_players_filter() {
        let players: Vec<super::Player> = super::fetch_players("https://api.utopia-server.com/")
            .unwrap()
            .into_iter()
            .filter(|x| x.position.is_some())
            .collect();
        println!("{:#?}", players);
    }

    #[test]
    fn test_one_player() {
        let player = super::fetch_player("ddd122ed-232f-4d72-a8a7-2a722193de84", "https://api.utopia-server.com/").unwrap();
        println!("{:#?}", player);
    }
    
}
