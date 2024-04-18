use askama::Template;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginHtmlQuery {
    client_id: String,
    redirect_uri: String,
    scope: Option<String>,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    client_id: String,
    redirect_uri: String,
    scope: String,
}

struct LoginHtmlTemplate<T>(T);

impl<T> IntoResponse for LoginHtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

pub async fn index(q: Query<LoginHtmlQuery>) -> impl IntoResponse {
    let scope = if let Some(scope) = &q.scope {
        scope.to_string()
    } else {
        "".to_string()
    };
    let template = LoginTemplate {
        client_id: q.client_id.to_string(),
        redirect_uri: q.redirect_uri.to_string(),
        scope,
    };
    LoginHtmlTemplate(template)
}
