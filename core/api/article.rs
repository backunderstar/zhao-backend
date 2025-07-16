use common::{EmptyResult, JsonResult, empty_ok, json_ok};
use db::pool;
use dto::{
    ArticleAddRequest, ArticleInfoResponse, ArticleListResponse, ArticleUpdateRequest,
    IDListRequest, IDRequest, PaginationRequest,
};
use entity::{article, prelude::Article};
use salvo::prelude::*;
use sea_orm::PaginatorTrait;

#[handler]
pub async fn add(req: &mut Request) -> EmptyResult {
    let article_add_req: ArticleAddRequest = req.extract().await?;

    let db = pool();

    article_add_req.insert_into_db(db).await?;

    empty_ok()
}

#[handler]
pub async fn delete(req: &mut Request) -> EmptyResult {
    let ids: IDListRequest = req.extract().await?;

    let db = pool();

    ids.delete_datas_by_ids::<Article>(db).await?;

    empty_ok()
}

#[handler]
pub async fn update(req: &mut Request) -> EmptyResult {
    let article_update_req: ArticleUpdateRequest = req.extract().await?;

    let db = pool();

    let article: article::ActiveModel = IDRequest::new(article_update_req.id)
        .get_data_by_id::<Article>(db)
        .await?
        .into();

    article_update_req.update_into_db(article, db).await?;

    empty_ok()
}

#[handler]
pub async fn get(req: &mut Request) -> JsonResult<ArticleInfoResponse> {
    let id: IDRequest = req.extract().await?;

    let db = pool();

    let data = id.get_data_by_id::<Article>(db).await?;

    json_ok(data.into())
}

#[handler]
pub async fn list(req: &mut Request) -> JsonResult<ArticleListResponse> {
    let pagination: PaginationRequest = req.extract().await?;

    let db = pool();

    let query = pagination.get_query::<Article>();

    let (page, limit) = pagination.get_page_limit();

    let count = query.clone().count(db).await?;

    let articles = query.paginate(db, limit).fetch_page(page).await?;

    let articles_info = articles.into_iter().map(|article| article.into()).collect();

    json_ok(ArticleListResponse::new(count, articles_info))
}
