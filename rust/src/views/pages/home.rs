use maud::{html, Markup};
use crate::views::layout::app_layout::create_app_layout;

pub async fn home_view() -> Markup {
    let view = html! {
        div {"shopping view home"}
    };

    create_app_layout(view)
}
