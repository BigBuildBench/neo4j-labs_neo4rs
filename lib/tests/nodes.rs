use neo4rs::*;

mod container;

#[tokio::test]
async fn nodes() {
    let neo4j = container::Neo4jContainer::new().await;
    let graph = neo4j.graph();

    include!("../include/nodes.rs");
}
