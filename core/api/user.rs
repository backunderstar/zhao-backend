use common::{EmptyResult, JsonResult, empty_ok, json_ok};
use db::pool;
use dto::{
    IDListRequest, IDRequest, UserAddRequest, UserInfoResponse, UserListRequest, UserListResponse,
    UserUpdateRequest,
};
use entity::{prelude::User, user};
use salvo::prelude::*;

/// 增加user
#[handler]
pub async fn add(req: &mut Request) -> EmptyResult {
    let mut user_add_req: UserAddRequest = req.extract().await?;

    let db = pool();

    user_add_req.encrypt_password()?.insert_into_db(db).await?;

    empty_ok()
}

/// 删除user
#[handler]
pub async fn delete(req: &mut Request) -> EmptyResult {
    let ids: IDListRequest = req.extract().await?;

    let db = pool();

    ids.delete_datas_by_ids::<User>(db).await?;

    empty_ok()
}

/// 更新user
#[handler]
pub async fn update(req: &mut Request) -> EmptyResult {
    let user_update_req: UserUpdateRequest = req.extract().await?;

    let db = pool();

    let user: user::ActiveModel = IDRequest::new(user_update_req.id)
        .get_data_by_id::<User>(db)
        .await?
        .into();

    user_update_req.update_into_db(user, db).await?;

    empty_ok()
}

/// 获取user信息
#[handler]
pub async fn get(req: &mut Request) -> JsonResult<UserInfoResponse> {
    let id: IDRequest = req.extract().await?;

    let db = pool();

    let model = id.get_data_by_id::<User>(db).await?;

    json_ok(model.into())
}

/// 获取user列表
#[handler]
pub async fn list(req: &mut Request) -> JsonResult<UserListResponse> {
    let user_list_req: UserListRequest = req.extract().await?;

    let db = pool();

    let data = user_list_req.get_list_from_db(db).await?;

    json_ok(data)
}
