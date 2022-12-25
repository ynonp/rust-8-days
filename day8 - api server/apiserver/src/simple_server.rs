use warp::Filter;

/*
* Part 1 - Basic usage, url path parameters, sending strings, query string params
*/

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"  
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let sum = warp::path!("sum" / u32 / u32)
        .map(|a, b| format!("{}", a + b));

    let api = 
        hello
        .or(sum);

    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
