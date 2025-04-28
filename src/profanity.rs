use serde::{Deserialize, Serialize};

use handle_errors::{Error, APILayerError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIResponse(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BadWord {
    original: String,
    word: String,
    deviations: i64,
    info: i64,
    #[serde(rename = "replacedLen")]
    replaced_len: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BadWordsResponse {
    content: String,
    bad_words_total: i64,
    bad_words_list: Vec<BadWord>,
    censored_content: String,
}

pub async fn check_profanity(content: String) -> Result<String, Error> {
    let client = reqwest::Client::new();

    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character={censor_character}")
        .header("apikey", "lymENveEPyMegkYCYipGPWNxGYMU4gYO")
        .body(content)
        .send()
        .await
        .map_err(|e| Error::ExternalAPIError(e))?;

    if !res.status().is_success() {
        let status = res.status().as_u16();
        let message = res.json::<APIResponse>().await.unwrap();

        let err = APILayerError {
            status,
            message: message.0,
        };

        if status < 500 {
            return Err(Error::ClientError(err));
        } else {
            return Err(Error::ServerError(err));
        }
    }

    match res.json::<BadWordsResponse>().await {
        Ok(res) => Ok(res.censored_content),
        Err(e) => Err(Error::ExternalAPIError(e)),
    }
}
