use crate::{utilities::minify::minify_html, views::layout::app_layout::create_app_layout};
use axum::response::Html;
use maud::html;

pub async fn home_view() -> Html<String> {
    let view = html! {
        div {"shopping view home"}
    };

    Html(minify_html(&create_app_layout(view).into_string()))
}
