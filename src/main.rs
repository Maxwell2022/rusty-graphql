use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::StatusCode;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};

#[derive(SimpleObject)]
struct User {
    name: String,
    id: i32,
}

struct Query;

#[Object(extends)]
impl Query {
    async fn get_users(&self) -> Vec<User> {
        vec![
            User {
                id: 1,
                name: String::from("name"),
            },
            User {
                id: 2,
                name: String::from("name 2"),
            },
        ]
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    // 1. Setup Route to handle Graphql Requests
    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    // 2. Setup Route for Graphql Playground
    let playground = warp::path("graphql")
        .and(warp::path::end())
        .and(warp::get())
        .map(|| {
            HttpResponse::builder()
                .header("content-type", "text/html")
                .body(playground_source(GraphQLPlaygroundConfig::new("/")))
        });

    // 3. Apply Variables As Routes
    let routes = playground
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            println!("Error {:?}", err);

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    // 4. Create And Start The Server
    warp::serve(routes).run(([0, 0, 0, 0], 4000)).await;
}
