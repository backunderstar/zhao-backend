use common::{json_ok, JsonResult};
use db::pool;
use dto::{LoginInRequest, LoginResponse};
use salvo::prelude::*;



/// 登录
#[handler]
pub async fn login_in(req: &mut Request) -> JsonResult<LoginResponse> {
    let login: LoginInRequest = req.extract().await?;

    let db = pool();

    let user = login.is_user_exist(db).await?;

    let data = login
        .check_password(&user.password)?
        .generate_token(user.id)?;

    json_ok(data)
}
