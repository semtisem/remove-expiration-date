use console::Term;
use dco3::nodes::RoomPoliciesRequest;
use dco3::Rooms;
use dco3::{auth::Connected, Dracoon};
use nodes::get_all_nodes;
use tracing::info;

mod nodes;

use super::{errors::AppError, init_dracoon, models::PasswordAuth};

use futures::stream::StreamExt; // for handling concurrent streams // for join_all

pub async fn handle_remove_expiration(
    _term: Term,
    base_url: String,
    data_room_id: u64,
    auth: Option<PasswordAuth>,
) -> Result<(), AppError> {
    let dracoon: Dracoon<Connected> = init_dracoon(&base_url, auth.clone(), true).await?;

    let room_ids = collect_home_room_and_sub_room_ids(dracoon.clone(), data_room_id).await?;

    revert_expiration_date(dracoon, room_ids).await?;

    Ok(())
}

async fn collect_home_room_and_sub_room_ids(
    dracoon: Dracoon<Connected>,
    data_room_id: u64,
) -> Result<Vec<u64>, AppError> {
    let mut room_ids = vec![];

    let home_rooms = get_all_nodes(dracoon.clone(), Some(data_room_id)).await?;

    home_rooms.items.iter().for_each(|room| {
        room_ids.push(room.id);
    });

    let fetch_futures = home_rooms.into_iter().map(|room| {
        let dracoon_clone = dracoon.clone();
        async move {
            let sub_rooms = get_all_nodes(dracoon_clone, Some(room.id)).await?;
            Ok(sub_rooms)
        }
    });

    let mut fetch_stream = futures::stream::iter(fetch_futures).buffer_unordered(10);

    while let Some(result) = fetch_stream.next().await {
        match result {
            Ok(sub_rooms) => {
                for sub_room in sub_rooms {
                    room_ids.push(sub_room.id);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(room_ids)
}

async fn revert_expiration_date(
    dracoon: Dracoon<Connected>,
    room_ids: Vec<u64>,
) -> Result<(), AppError> {
    let revert_futures = room_ids.into_iter().map(|room_id| {
        let dracoon_clone = dracoon.clone();
        async move {
            let new_policy = RoomPoliciesRequest::builder()
                .with_default_expiration_period(0)
                .build();

            dracoon_clone
                .nodes()
                .update_room_policies(room_id, new_policy)
                .await?;

            Ok(((), room_id))
        }
    });

    let mut revert_stream = futures::stream::iter(revert_futures).buffer_unordered(10);

    while let Some(result) = revert_stream.next().await {
        match result {
            Ok((_, room_id)) => {
                info!(
                    "Expiration date reverted successfully for room with id {}",
                    room_id
                );
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}
