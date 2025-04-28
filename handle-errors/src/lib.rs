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
use reqwest::Error as ReqwestError;

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
    ExternalAPIError(ReqwestError),
    ClientError(APILayerError),
    ServerError(APILayerError),
}

#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::ParseError(ref err) => write!(f, "cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "missing parameter"),
	    Error::DatabaseQueryError => write!(f, "Query could not be executed"),
	    Error::ExternalAPIError(ref err) => write!(f, "Cannot execute: {}", err), 
	    Error::ClientError(ref err) => write!(f, "External Client error: {}", err),
	    Error::ServerError(ref err) => write!(f, "External Server error: {}", err),
        }
    }
}

impl Reject for Error {}
impl Reject for APILayerError {}

//impl From<SqlxError> for Error {
//    fn from(err: SqlxError) -> Self {
//	Error::DatabaseQueryError(err)
//    }
//}

#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQueryError) = r.find() {
        event!(Level::ERROR, "Database query error");

        Ok(warp::reply::with_status(
            Error::DatabaseQueryError.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::ExternalAPIError(err)) = r.find() {
	event!(Level::ERROR, "{}", err);

	Ok(warp::reply::with_status(
	    "Internal Server Error".to_string(),
	    StatusCode::INTERNAL_SERVER_ERROR,
	))
    } else if let Some(Error::ClientError(err)) = r.find() {
	event!(Level::ERROR, "{}", err);

	Ok(warp::reply::with_status(
	    "Internal Server Error".to_string(),
	    StatusCode::INTERNAL_SERVER_ERROR,
	))
    } else if let Some(Error::ServerError(err)) = r.find() {
	event!(Level::ERROR, "{}", err);

	Ok(warp::reply::with_status(
	    "Internal Server Error".to_string(),
	    StatusCode::INTERNAL_SERVER_ERROR,
	))
    } else if let Some(err) = r.find::<Error>() {
    //if let Some(err) = r.find::<Error>() {
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
