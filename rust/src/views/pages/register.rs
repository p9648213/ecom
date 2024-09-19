use maud::{html, Markup};

use crate::views::{
    layout::{app_layout::create_app_layout, auth_layout::create_auth_layout},
    ui::{input_with_label::input_with_label, primary_button::primary_button},
};

pub async fn register_view() -> Markup {
    let view = html! {
        (register_form())
    };

    create_app_layout(create_auth_layout(view))
}

pub fn register_form() -> Markup {
    html! {
        div class="mx-auto w-full max-w-md space-y-6" {
            div class="text-center" {
                h1 class="text-3xl font-bold tracking-tight text-foreground" { "Create new account"}
                div class="mt-2 flex justify-center" {
                    "Already have an account?"
                    a id="login-link" href="/auth/login" class="font-medium text-pretty hover:underline ml-2" {
                        span class="button-text" { "Login" }
                        div class="button-indicator w-6 h-6 border-4 text-primary border-t-transparent rounded-full animate-spin flex" {}
                    }
                }
            }
            form
                id="register-form"
                hx-post="/auth/register"
                hx-swap="none"
                hx-on:submit="this.reset()"
                hx-disabled-elt="find button"
            {
                div class="flex flex-col gap-3" {
                    (input_with_label("Username", "text", "username", "username", "Enter your username"))
                    (input_with_label("Email", "email", "email", "email", "Enter your email"))
                    (input_with_label("Password", "password", "password", "password", "Enter your password"))
                }
                (primary_button(Some("Sign Up"), Some("w-full mt-3"), None))
            }
        }
    }
}
