use common::dto::PaginationRequest;
use salvo::prelude::*;
use tera::{Context, Tera};

#[handler]
pub async fn index(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let param = req.param("id");

    let page = match param {
        Some(id) => id,
        None => 1 as u32,
    };

    PaginationRequest{
        page: Some(page),
        limit: None,
        order: None,
        keyword: None,
    };

    

    let mut context = Context::new();
    context.insert("message", "Hello World!");
    render_page(depot, res, "pages/index.html", context).await;
}

#[handler]
pub async fn about(depot: &mut Depot, res: &mut Response) {
    let mut context = Context::new();
    context.insert("message", "Hello World!");
    render_page(depot, res, "pages/about.html", context).await;
}

async fn render_page(depot: &mut Depot, res: &mut Response, template: &str, context: Context) {
    let tera = match depot.obtain_mut::<Tera>() {
        Ok(tera) => tera,
        Err(_) => {
            res.status_code(StatusCode::NOT_FOUND);
            return;
        }
    };
    match tera.full_reload() {
        Ok(_) => {}
        Err(_) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
    };
    let page = match tera.render(template, &context) {
        Ok(page) => page,
        Err(_) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
    };
    res.render(Text::Html(page));
}
