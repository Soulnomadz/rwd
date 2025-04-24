use crate::store::Store;
use crate::types::pagination::{Pagination, extract_pagination};
use crate::types::question::{NewQuestion, Question};

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

    let res: Vec<Question> = match store.get_questions(page.limit, page.offset).await {
        Ok(res) => res,
        //Err(_) => return Err(warp::reject::custom(Error::DatabaseQueryError)),
	Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn get_question(
  id: i32, 
  store: Store,
) -> Result<impl Reply, Rejection> {
    event!(target: "rwd", Level::INFO, "querying question with id: {}", id);

    let res = match store.get_question(id).await {
        Ok(res) => res,
        //Err(_) => return Err(warp::reject::custom(Error::DatabaseQueryError)),
	Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn add_question(store: Store, question: NewQuestion) -> Result<impl Reply, Rejection> {
    match store.add_question(question).await {
        Ok(_) => Ok(warp::reply::with_status("Question added", StatusCode::OK)),
        //Err(_) => Err(warp::reject::custom(Error::DatabaseQueryError)),
	Err(e) => return Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    let res = match store.update_question(question, id).await {
        Ok(res) => res,
        //Err(_) => return Err(warp::reject::custom(Error::DatabaseQueryError)),
	Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl Reply, Rejection> {
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        //Err(_) => Err(warp::reject::custom(Error::DatabaseQueryError)),
	Err(e) => return Err(warp::reject::custom(e)),
    }
}
