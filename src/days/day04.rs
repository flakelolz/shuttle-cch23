use axum::{extract, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReindeerStrength {
    name: String,
    strength: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reindeer {
    name: String,
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Winners {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub async fn calc_strength(extract::Json(payload): extract::Json<Vec<ReindeerStrength>>) -> String {
    payload
        .iter()
        .fold(0_i32, |a, b| a + b.strength)
        .to_string()
}

pub async fn contest(extract::Json(payload): extract::Json<Vec<Reindeer>>) -> Json<Winners> {
    let fastest = payload
        .iter()
        .max_by(|x, y| x.speed.abs().total_cmp(&y.speed.abs()))
        .unwrap();

    let tallest = payload.iter().max_by_key(|r| r.height.abs()).unwrap();

    let magician = payload
        .iter()
        .max_by_key(|r| r.snow_magic_power.abs())
        .unwrap();

    let consumer = payload
        .iter()
        .max_by_key(|r| r.candies_eaten_yesterday.abs())
        .unwrap();

    Json(Winners {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert_strength(value: serde_json::Value) -> Vec<ReindeerStrength> {
        let mut v: Vec<ReindeerStrength> = Vec::new();
        let length = value.as_array().unwrap().len();

        for i in 0..length {
            v.push(ReindeerStrength {
                name: value[i]["name"].as_str().unwrap().to_string(),
                strength: value[i]["strength"].as_i64().unwrap() as i32,
            })
        }
        v
    }
    fn convert_contest(value: serde_json::Value) -> Vec<Reindeer> {
        let mut v: Vec<Reindeer> = Vec::new();
        let length = value.as_array().unwrap().len();

        for i in 0..length {
            v.push(Reindeer {
                name: value[i]["name"].as_str().unwrap().to_string(),
                strength: value[i]["strength"].as_i64().unwrap() as i32,
                speed: value[i]["speed"].as_f64().unwrap() as f32,
                height: value[i]["height"].as_i64().unwrap() as i32,
                antler_width: value[i]["antler_width"].as_i64().unwrap() as i32,
                snow_magic_power: value[i]["snow_magic_power"].as_i64().unwrap() as i32,
                favorite_food: value[i]["favorite_food"].as_str().unwrap().to_string(),
                candies_eaten_yesterday: value[i]["cAnD13s_3ATeN-yesT3rdAy"].as_i64().unwrap()
                    as i32,
            })
        }
        v
    }


    #[tokio::test]
    async fn test_calc_strength() {
        let value = serde_json::json!([
        { "name": "Dasher", "strength": 5 },
        { "name": "Dancer", "strength": 6 },
        { "name": "Prancer", "strength": 4 },
        { "name": "Vixen", "strength": 7 }
        ]);

        let input = convert_strength(value);

        assert_eq!(calc_strength(Json(input)).await, "22".to_string());
    }

    #[tokio::test]
    async fn test_contest() {
        let value = serde_json::json!([
            {
                "name": "Dasher",
                "strength": 5,
                "speed": 50.4,
                "height": 80,
                "antler_width": 36,
                "snow_magic_power": 9001,
                "favorite_food": "hay",
                "cAnD13s_3ATeN-yesT3rdAy": 2
            },
            {
                "name": "Dancer",
                "strength": 6,
                "speed": 48.2,
                "height": 65,
                "antler_width": 37,
                "snow_magic_power": 4004,
                "favorite_food": "grass",
                "cAnD13s_3ATeN-yesT3rdAy": 5
            }
        ]);

        let input = convert_contest(value);

        let output = Winners {
            fastest: "Speeding past the finish line with a strength of 5 is Dasher".to_string(),
            tallest: "Dasher is standing tall with his 36 cm wide antlers".to_string(),
            magician: "Dasher could blast you away with a snow magic power of 9001".to_string(),
            consumer: "Dancer ate lots of candies, but also some grass".to_string(),
        };

        assert_eq!(contest(Json(input)).await.0, output);
    }
}
