use anyhow::*;
use graphql_client::{GraphQLQuery, Response};
use serde::Serialize;
use std::thread;
use std::time::Instant;


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
type BigInt = String;

enum DestinationType {
    MetaOps,
    TheGraph,
}

fn perform_my_query(variables: collectors::Variables, indexer: &str) -> Result<Response<collectors::ResponseData>> {

    let request_body = Collectors::build_query(variables);

    let client = reqwest::blocking::Client::new();
    let res = client.post(indexer).json(&request_body).send()?;
    let response_body: Response<collectors::ResponseData> = res.json()?;
    Ok(response_body)
}

fn main() {

    println!("Spawning 100 requests");
    let now = Instant::now();
    spawn_thread(&DestinationType::TheGraph).join().expect("The thread was interrupted");
    let elapsed = now.elapsed();
    println!("TheGraph indexer took: {:.2?}", elapsed);

    let now = Instant::now();
    spawn_thread(&DestinationType::MetaOps).join().expect("The thread was interrupted");
    let elapsed = now.elapsed();
    println!("Metaops Indexer took: {:.2?}", elapsed);
}

fn spawn_thread(d: &DestinationType) -> std::thread::JoinHandle<()> {

    let indexer = match d {
        DestinationType::MetaOps => "http://graph-query.sandbox.metaops.solutions/subgraphs/name/known_origin2",
        DestinationType::TheGraph => "https://api.thegraph.com/subgraphs/name/knownorigin/known-origin"
    };

    let handle = thread::spawn(||
        for _ in 1..100 {
            let v = collectors::Variables {};
            let result = perform_my_query(v, indexer);
            assert!(result.unwrap().data.is_some());
        });
    handle
}