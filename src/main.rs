#![feature(decl_macro)]
extern crate futures;
extern crate hyper;

use warp::Filter;

mod endpoint_handlers;

#[tokio::main]
async fn main() {
    let ping_chain = warp::path!("v1" / "ping-chain")
        .map(|| endpoint_handlers::ping_chain());

    /* 
      USAGE: http://127.0.0.1:3030/v1/logs/test_file/2021-12-14
      Timestamp (UTC) format example: 2021-12-14
    */
    let get_logs_with_filename_and_date = warp::path!("v1" / "logs" / String / String)
      .and_then(endpoint_handlers::get_file_logs_from_date);
      
    /*
      CURL SAMPLE USAGE: curl -X POST 127.0.0.1:3030/v1/logs -H 'Content-Type: application/json' -d '{"filename":"test_log_file1","title":"test_four_title","content":"content4"}'
     */
    let save_log = warp::post()
      .and(warp::path("v1"))
      .and(warp::path("logs"))
      .and(warp::path::end())
      .and(endpoint_handlers::log_body())
      .and_then(endpoint_handlers::save_log);

    /*
      CURL SAMPLE USAGE: curl -X POST 127.0.0.1:3030/v1/participants -H 'Content-Type: application/json' -d '{"validator_id":"0x16b215a5fd8b8caa03e75313e78c8ee344ba6ee7c6c2d6ce0a0a2e4e3a0d2377f775fc2682db3bb79376a61e773a87cddd1e5e5935cdea8a9ab3a54be011a62b"}'
     */
    let add_validator = warp::post()
      .and(warp::path("v1"))
      .and(warp::path("participants"))
      .and(warp::path::end())
      .and(endpoint_handlers::validator_body())
      .and_then(endpoint_handlers::add_validator);

  let routes = ping_chain
    .or(get_logs_with_filename_and_date)
    .or(save_log)
    .or(add_validator);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
