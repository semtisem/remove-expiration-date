use std::sync::Arc;

use dco3::{
    auth::Connected,
    nodes::{Node, NodesFilter},
    Dracoon, ListAllParams, Nodes, RangedItems,
};
use futures::StreamExt;
use futures_util::stream;
use tokio::sync::Mutex;
use tracing::{debug, error};

use crate::cmd::errors::AppError;

pub async fn get_all_nodes(
    dracoon: Dracoon<Connected>,
    parent_id: Option<u64>,
) -> Result<RangedItems<Node>, AppError> {
    // Initial request
    let params = ListAllParams::builder()
        .with_offset(0)
        .with_limit(500)
        .with_filter(NodesFilter::is_room())
        .build();
    let results = dracoon
        .nodes()
        .get_nodes(parent_id, None, Some(params))
        .await?;
    let total = results.range.total;
    let shared_results = Arc::new(Mutex::new(results.clone()));

    // Subsequent requests
    let reqs = (500..=total)
        .step_by(500)
        .map(|offset| {
            let params = ListAllParams::builder()
                .with_offset(offset)
                .with_limit(500)
                .with_filter(NodesFilter::is_room())
                .build();

            dracoon.nodes().get_nodes(parent_id, None, Some(params))
        })
        .collect::<Vec<_>>();

    stream::iter(reqs)
        .for_each_concurrent(5, |f| {
            let shared_results_clone = Arc::clone(&shared_results);
            async move {
                match f.await {
                    Ok(mut nodes) => {
                        let mut shared_results = shared_results_clone.lock().await;
                        shared_results.items.append(&mut nodes.items);
                    }
                    Err(e) => {
                        error!("Failed to fetch nodes: {}", e);
                    }
                }
            }
        })
        .await;

    let results = shared_results.lock().await.clone();
    debug!(
        "Fetched {} child nodes from parent {}",
        results.items.len(),
        parent_id.unwrap_or(0)
    );
    Ok(results)
}
