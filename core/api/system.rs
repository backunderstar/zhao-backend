use common::{empty_ok, json_ok, EmptyResult, JsonResult};
use db::pool;
use dto::{IDRequest, SystemInfoResponse, SystemUpdateRequest};
use salvo::prelude::*;
use entity::prelude::System;


#[handler]
pub async fn update(req: &mut Request) -> EmptyResult {
    let system_update_request: SystemUpdateRequest = req.extract().await?;

    system_update_request.check_none()?;

    let id = IDRequest::new(1);

    let db = pool();

    let system = id.get_data_by_id::<System>(db).await?.into();

    system_update_request.update_into_db(system, db).await?;

    empty_ok()
}

#[handler]
pub async fn get() -> JsonResult<SystemInfoResponse> {
    let id = IDRequest::new(1);

    let db = pool();

    let system = id.get_data_by_id::<System>(db).await?;

    json_ok(system.into())
}
