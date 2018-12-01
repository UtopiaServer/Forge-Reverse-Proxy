//!
//! Player lib
//! Handle Player and system
//!
#![feature(custom_attribute)]
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

/// Structure for the player position
//pub struct Player {
//    position: Position
//}

struct Position {
    x: f64,
    y: f64,
    z: f64,
    dimension: i32,
}

impl Position {
    fn new(x: f64, y: f64, z: f64, dimension: i32) -> Self {
        Self {
            x,
            y,
            z,
            dimension
        }
    }
}

#[derive(Deserialize, Debug)]
struct Data {
    data: Player
}

#[derive(Deserialize, Debug)]
struct Player {
    players: Uuid
}

#[derive(Deserialize, Debug)]
struct Uuid {
    uuid: std::collections::HashMap<String, String>
}

fn request_player(uuid: String) {
    let client = reqwest::Client::new();

    let query = format!("
    {{
        players(uuid:\"{}\") {{
            uuid
        }}
    }}", uuid);

    println!("{}", query);

    let mut res = client
        .post("https://api.utopia-server.com/")
        .body(query)
        .header("content-type", "application/graphql")
        .send()
        .unwrap();

    //println!("{}", res.text().unwrap());

    let response: Data = res
        .json()
        .unwrap();

    println!("{:#?}", response);
}

#[cfg(test)]
mod test {

    const UUID: [&'static str; 3] = [
        "8603f518-c2f1-407d-b019-90aa022e22bf",
        "d28e5f13-4b84-44ce-9d32-51f1b569adda",
        "ddd122ed-232f-4d72-a8a7-2a722193de84"
    ];

    #[test]
    fn req_player() {
        super::request_player(UUID[0].to_string());
    }

}




