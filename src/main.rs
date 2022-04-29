use anyhow::*;
use graphql_client::{GraphQLQuery, Response};
use serde::Serialize;
use std::thread;


// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(Serialize)]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/ko_subgraph_schema.graphql",
    query_path = "graphql/ko_subgraph_query.graphql",
    response_derives = "Debug"
)]
pub struct Collectors;


fn perform_my_query(variables: collectors::Variables) -> Result<Response<collectors::ResponseData>> {

    let request_body = Collectors::build_query(variables);

    let indexer = "http://graph-query.sandbox.metaops.solutions/subgraphs/id/QmXuMojFeSY6JdLNapkHQ6BDPRT6WePd9ruA5Jy8sy3nYa";

    let client = reqwest::blocking::Client::new();
    let res = client.post(indexer).json(&request_body).send()?;
    let response_body: Response<collectors::ResponseData> = res.json()?;
    Ok(response_body)
}

fn main() {

    let handle = thread::spawn(||
        for i in 1..100 {
            println!("Performing query number: {}", i);
            let v = collectors::Variables {};
            let result = perform_my_query(v);
            assert!(result.unwrap().data.is_some());
        });

    handle.join().expect("The thread was interrupted");
}