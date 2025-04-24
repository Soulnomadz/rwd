use crate::store::Store;
use crate::types::answer::NewAnswer;

use warp::{Rejection, Reply, http::StatusCode};

pub async fn add_answer(answer: NewAnswer, store: Store) -> Result<impl Reply, Rejection> {
    match store.add_answer(answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
