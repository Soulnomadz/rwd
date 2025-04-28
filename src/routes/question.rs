use crate::store::Store;
use crate::types::pagination::{Pagination, extract_pagination};
use crate::types::question::{NewQuestion, Question};
use crate::profanity::check_profanity;

//use handle_errors::Error;

use std::collections::HashMap;

use warp::{Rejection, Reply, http::StatusCode};

//use log::info;
use tracing::{Level, event, instrument};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    event!(target: "rwd", Level::INFO, "querying questions");

    let mut page = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        page = extract_pagination(params)?;
    }

    match store.get_questions(page.limit, page.offset).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn get_question(id: i32, store: Store) -> Result<impl Reply, Rejection> {
    event!(target: "rwd", Level::INFO, "querying question with id: {}", id);

    match store.get_question(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn add_question(store: Store, question: NewQuestion) -> Result<impl Reply, Rejection> {
    let title = match check_profanity(question.title).await {
	Ok(res) => res,
	Err(e) => return Err(warp::reject::custom(e)),
    };

    let content = match check_profanity(question.content).await {
	Ok(res) => res,
	Err(e) => return Err(warp::reject::custom(e)),
    };

    let question = NewQuestion {
	title, 
	content,
	tags: question.tags,
    };

    match store.add_question(question).await {
        Ok(_) => Ok(warp::reply::with_status("Question added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    let title = match check_profanity(question.title).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let content = match check_profanity(question.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let question = Question {
	id: question.id,
        title,
        content,
        tags: question.tags,
    };

    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl Reply, Rejection> {
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
