use serde::{Serialize, Deserialize, de::DeserializeOwned};

// static BACKEND_URL: &'static str = "backend.simrace.club";
static BACKEND_URL: &'static str = "http://localhost:8000";

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: String,
    pub series: Series,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Series {
    BeamTouringcarMasters,
    CovetCup
}

fn get<S: Into<String>, R: DeserializeOwned>(s: S) -> anyhow::Result<R> {
    Ok(reqwest::blocking::get(format!("{}/{}", BACKEND_URL, s.into()))?.json()?)
}

pub fn get_upcoming_races() -> Vec<Event> {
    get("/upcoming_events").unwrap_or(Vec::new())
}

#[derive(Serialize, Deserialize)]
struct DiscordAccData {
    id: String,
}

pub fn get_discord_id(token: &ezoauth::Token) -> anyhow::Result<String> {
    let client = reqwest::blocking::Client::new();
    let request = client.request(reqwest::Method::GET, "https://discordapp.com/api/users/@me").header("Authorization", format!("Bearer {}", token.access_token()));
    let data: DiscordAccData = request.send()?.json()?;
    Ok(data.id)
}
