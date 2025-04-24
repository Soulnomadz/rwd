use warp::{
    Rejection, 
    Reply,
    http::StatusCode,
    reject::Reject,
    filters::body::BodyDeserializeError,
    filters::cors::CorsForbidden,
};

use tracing::{event, Level, instrument};
//use sqlx::error::Error as SqlxError;

use std::fmt;

//#[derive(Debug)]
//struct InvalidId;
//
//impl Reject for InvalidId {}

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    DatabaseQueryError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::ParseError(ref err) => write!(f, "cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "missing parameter"),
	    Error::DatabaseQueryError => write!(f, "Query could not be executed"),
        }
    }
}

impl Reject for Error {}

#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQueryError) = r.find() {
        event!(Level::ERROR, "Database query error");

        Ok(warp::reply::with_status(
            Error::DatabaseQueryError.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(err) = r.find::<Error>() {
	event!(Level::ERROR, "{}", err);

        Ok(warp::reply::with_status(
            err.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(err) = r.find::<CorsForbidden>() {
	event!(Level::ERROR, "CORS forbidden error: {}", err);	

        Ok(warp::reply::with_status(
            err.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(err) = r.find::<BodyDeserializeError>() {
	event!(Level::ERROR, "Cannot deserialize request body: {}", err);

        Ok(warp::reply::with_status(
            err.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
	event!(Level::WARN, "Requested route not found");

        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
