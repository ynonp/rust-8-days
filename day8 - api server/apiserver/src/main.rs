use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Sum2QueryParams {
    a: u32,
    b: u32,
}

/*
* Part 1 - Basic usage, url path parameters, sending strings, query string params
*/

/*
#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"  
    let _hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let _sum = warp::path!("sum" / u32 / u32)
        .map(|a, b| format!("{}", a + b));

    let _sum2 = warp::path!("sum2")
        .and(warp::query::<Sum2QueryParams>())
        .map(|params: Sum2QueryParams| format!("{}", params.a + params.b));

    let api = 
        _hello
        .or(_sum)
        .or(_sum2);

    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
 */

mod db {
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;
    use tokio::sync::Mutex;


    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct Note {
        id: String,
        title: String,
        text: String
    }
    
    pub fn init() -> Arc<Mutex<Vec<Note>>> {
        return Arc::new(Mutex::new(vec![
            Note { id: String::from("1"), title: String::from("yay"), text: String::from("it's a first note") },
            Note { id: String::from("2"), title: String::from("yay"), text: String::from("it's a second note") },
            Note { id: String::from("3"), title: String::from("yay"), text: String::from("it's a third note") },
        ]));
    }
}

mod handlers {
    use std::convert::Infallible;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use warp::http::StatusCode;
    use super::db::Note;
    
    pub async fn list_notes(notes: Arc<Mutex<Vec<Note>>>) -> Result<impl warp::Reply, Infallible> {
        let notes = notes.lock().await;
        let notes: Vec<Note> = notes.clone().into_iter().collect();
        Ok(warp::reply::json(&notes))
    }

    pub async fn create_note(new_note: Note, notes: Arc<Mutex<Vec<Note>>>) -> Result<impl warp::Reply, Infallible> {
        let mut notes = notes.lock().await;
        notes.push(new_note);

        Ok(StatusCode::CREATED)
    }
}

mod api {
    use super::db::Note;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use warp::Filter;
    use super::handlers;

    pub fn routes(notes: Arc<Mutex<Vec<Note>>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        list_notes(notes.clone())
        .or(create_note(notes.clone()))
    }

    pub fn list_notes(notes: Arc<Mutex<Vec<Note>>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("notes")
        .and(warp::get())
        .and(warp::any().map(move || notes.clone()))
        .and_then(handlers::list_notes)
    }

    pub fn create_note(notes: Arc<Mutex<Vec<Note>>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("notes")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
        .and(warp::any().map(move || notes.clone()))
        .and_then(handlers::create_note)        
    }
}

#[tokio::main]
async fn main() {
    let notes = db::init();

    /*

    let n1 = notes.clone();
    let n2 = notes.clone();

    let api = 
    warp::path!("notes")
        .and(warp::get())    
        .and(warp::any().map(move || n1.clone()))
        .and_then(handlers::list_notes)
    .or(
        warp::path!("notes")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
        .and(warp::any().map(move || n2.clone()))
        .and_then(handlers::create_note)
    );
     */

    let api = api::routes(notes);

    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}