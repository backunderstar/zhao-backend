use common::{EmptyResult, empty_ok};
use salvo::{
    Router, handler,
    prelude::{StaticDir, StaticFile},
};

use crate::{article, image};

use super::{login, system, user};

/// 路由
pub fn get_all_route() -> Router {
    /* let tera = match Tera::new("theme/simple/**/
*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    }; */

    Router::new()
        // health
        .push(Router::with_path("health").get(health))
        // api
        .push(
            Router::with_path("api")
                // no auth
                // login
                .push(Router::with_path("login").post(login::login_in))
                // auth
                .push(
                    Router::new()
                        // jwt middleware
                        .hoop(hoop::auth_hoop())
                        // system
                        .push(
                            Router::with_path("system")
                                .get(system::get)
                                .put(system::update),
                        )
                        // user
                        .push(
                            Router::with_path("user")
                                .post(user::add)
                                .get(user::list)
                                .delete(user::delete)
                                .push(Router::with_path("{id}").get(user::get).put(user::update)),
                        )
                        // article
                        .push(
                            Router::with_path("article")
                                .post(article::add)
                                .get(article::list)
                                .delete(article::delete)
                                .push(
                                    Router::with_path("{id}")
                                        .get(article::get)
                                        .put(article::update),
                                ),
                        )
                        // image
                        .push(
                            Router::with_path("image")
                                .get(image::list)
                                .post(image::upload)
                                .delete(image::delete),
                        ),
                ),
        )
        // static server
        .push(
            Router::new()
                // favicon.ico图标
                .push(Router::with_path("favicon.ico").get(StaticFile::new("static/favicon.ico")))
                // 默认资源
                .push(Router::with_path("default/{**path}").get(StaticDir::new(["static/default"])))
                // 后台管理页面
                // .push(
                //     Router::with_path("admin/{**path}")
                //         // dev
                //         .get(StaticDir::new(["admin/dist"]).fallback("index.html")),
                //     // prod
                //     //.get(StaticDir::new(["static/admin"]).fallback("index.html")),
                // )
                // 上传的文件
                .push(Router::with_path("upload/{**path}").get(StaticDir::new(["upload"]))),
        )
    // template
    /* .push(
        Router::new()
            .hoop(affix_state::inject(tera))
            .get(template_handler::index)
            .push(Router::with_path("{id}").get(template_handler::index))
            .push(Router::with_path("about").get(template_handler::about)),
    ) */
}

#[handler]
async fn health() -> EmptyResult {
    empty_ok()
}
