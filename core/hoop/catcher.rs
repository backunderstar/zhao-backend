
use salvo::{handler, http::StatusCode, FlowCtrl, Response};

//==================================================================================
//= 捕获所有错误
//= catch all error
#[handler]
pub async fn catcher_all(res: &mut Response, ctrl: &mut FlowCtrl) {
    if StatusCode::NOT_FOUND == res.status_code.unwrap_or(StatusCode::NOT_FOUND) {
        res.render("Custom 404 Error Page");
        ctrl.skip_rest();
    }
    /* 
    if let Some(StatusCode::UNAUTHORIZED) = res.status_code {
        res.stuff(
            StatusCode::UNAUTHORIZED,
            Json({
                code: 401,
                message: "Unauthorized".to_string(),
                source_error: anyhow!("Unauthorized").into(),
            }),
        );
        ctrl.skip_rest();
    }
    if let Some(StatusCode::FORBIDDEN) = res.status_code {
        res.stuff(
            StatusCode::FORBIDDEN,
            Json(ErrorResponseBuilder {
                code: 403,
                message: "Forbidden".to_string(),
                source_error: anyhow!("Forbidden").into(),
            }),
        );
        ctrl.skip_rest();
    }
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        res.stuff(
            StatusCode::NOT_FOUND,
            Json(ErrorResponseBuilder {
                code: 404,
                message: "Not Found".to_string(),
                source_error: anyhow!("Not Found").into(),
            }),
        );
        ctrl.skip_rest();
    }
    if let Some(StatusCode::METHOD_NOT_ALLOWED) = res.status_code {
        res.stuff(
            StatusCode::METHOD_NOT_ALLOWED,
            Json(ErrorResponseBuilder {
                code: 405,
                message: "Method Not Allowed".to_string(),
                source_error: anyhow!("Method Not Allowed").into(),
            }),
        );
        ctrl.skip_rest();
    }
    if let Some(StatusCode::INTERNAL_SERVER_ERROR) = res.status_code {
        res.stuff(
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponseBuilder {
                code: 500,
                message: "Internal Server Error".to_string(),
                source_error: anyhow!("Internal Server Error").into(),
            }),
        );
        ctrl.skip_rest();
    }
    if let Some(StatusCode::BAD_REQUEST) = res.status_code {
        res.stuff(
            StatusCode::BAD_REQUEST,
            Json(ErrorResponseBuilder {
                code: 400,
                message: "Bad Request".to_string(),
                source_error: anyhow!("Bad Request").into(),
            }),
        );
        ctrl.skip_rest();
    }
    if let Some(StatusCode::GATEWAY_TIMEOUT) = res.status_code {
        res.stuff(
            StatusCode::GATEWAY_TIMEOUT,
            Json(ErrorResponseBuilder {
                code: 504,
                message: "Gateway Timeout".to_string(),
                source_error: anyhow!("Gateway Timeout").into(),
            }),
        );
        ctrl.skip_rest();
    } */
}
