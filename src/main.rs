mod profanity;
mod routes;
mod store;
mod types;

use crate::routes::answer::add_answer;
use crate::routes::question::*;
use store::Store;

use handle_errors::return_error;
//use log::{error, info, warn};
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{Filter, http::Method};

#[tokio::main]
async fn main() {
    // use env_logger for log
    //env_logger::init();
    // use log4rs for log
    //log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "rwd=info,warp=error".to_owned());
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record
        .with_env_filter(log_filter)
        // Record an event each span closes.
        // This can be used to time our routes' durations
        .with_span_events(FmtSpan::CLOSE)
        .init();

    //error!("Something ERROR, handle it now!");
    //info!("This is an info");
    //warn!("Warning info...");

    /*
    let log = warp::log::custom(|info| {
        // Just print to stderr
        //eprintln!(
        // use log crate
        info!(
            "{} {} {} {:?} from {} with {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
        );
    });
    */

    let statics = warp::fs::dir("statics");

    let store = Store::new("postgres://postgres:Asdf123$@localhost:5432/rustwebdev").await;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migration!");

    let store_filter = warp::any().map(move || store.clone());

    // used for log4rs
    // let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        //.allow_header("not-in-the-request")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        //.and(id_filter)
        .and_then(get_questions)
        .with(warp::trace(|info| {
            tracing::info_span!(
            "get_questions request",
            method = %info.method(),
            path = %info.path(),
            id = %uuid::Uuid::new_v4(),
            )
        }));

    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and(store_filter.clone())
        .and_then(add_answer);

    let route = statics
        .or(get_questions)
        .or(get_question)
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .with(cors)
        //.with(log)
        //.with(warp::trace::request())
        .recover(return_error);

    warp::serve(route).run(([0, 0, 0, 0], 3030)).await;
}
