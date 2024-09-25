use maud::{html, Markup, DOCTYPE};

pub fn create_app_layout(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="stylesheet" href="/assets/css/main.css";
                script src="/assets/js/lib/htmx.js" defer {};
                script src="/assets/js/lib/nprogress.js" defer {};
                script src="/assets/js/main.js" defer type="module" {};
                link rel="icon" type="image/x-icon" href="/assets/images/favicon.ico" {};
            }
            body hx-boost="true" hx-history="false" {
                (content)
                div id="toast" {}
            }

        }
    }
}
