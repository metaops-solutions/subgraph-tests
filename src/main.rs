use anyhow::*;
use graphql_client::{GraphQLQuery, Response};
use serde::Serialize;
use std::error::Error;

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


async fn perform_my_query(variables: collectors::Variables) -> Result<()> {

    let request_body = Collectors::build_query(variables);

    let indexer = "http://graph-query.sandbox.metaops.solutions/subgraphs/id/QmXuMojFeSY6JdLNapkHQ6BDPRT6WePd9ruA5Jy8sy3nYa";

    let client = reqwest::Client::new();
    let res = client.post(indexer).json(&request_body).send().await?;
    let response_body: Response<collectors::ResponseData> = res.json().await?;
    println!("RESULT: {:#?}", response_body);
    Ok(())
}

fn main() {

    let v = collectors::Variables {
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(perform_my_query(v));

}