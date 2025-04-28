use crate::store::Store;
use crate::types::answer::NewAnswer;
use crate::profanity::check_profanity;

use warp::{Rejection, Reply, http::StatusCode};

pub async fn add_answer(answer: NewAnswer, store: Store) -> Result<impl Reply, Rejection> {
    let content = match check_profanity(answer.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let answer = NewAnswer {
	content,
	question_id: answer.question_id,
    };

    println!("{:?}", &answer);

    match store.add_answer(answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
