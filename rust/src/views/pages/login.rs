use maud::{html, Markup};

use crate::views::{
    layout::{app_layout::create_app_layout, auth_layout::create_auth_layout},
    ui::{input_with_label::input_with_label, primary_button::primary_button},
};

pub async fn login_view() -> Markup {
    let view = html! {
        (login_form())
    };

    create_app_layout(create_auth_layout(view))
}

pub fn login_form() -> Markup {
    html! {
        div class="mx-auto w-full max-w-md space-y-6" {
            div class="text-center" {
                h1 class="text-3xl font-bold tracking-tight text-foreground" { "Sign in to your account" }
                div class="mt-2 flex justify-center" {
                    "Don't have an account?"
                    a id="register-link" href="/auth/register" class="font-medium text-pretty hover:underline ml-2" {
                        span { "Register" }
                    }
                }
            }
            form
                id="login-form"
                hx-post="/auth/login"
                hx-swap="none"
                "hx-on::after-request"="if(event.detail.successful) this.reset()"
                hx-disabled-elt="find button"
            {
                div class="flex flex-col gap-3" {
                    (input_with_label("Email", "email", "email", "email", "Enter your email"))
                    (input_with_label("Password", "password", "password", "password", "Enter your password"))
                }
                (primary_button(Some("Sign In"), Some("w-full mt-3"), None))
            }
        }
    }
}
