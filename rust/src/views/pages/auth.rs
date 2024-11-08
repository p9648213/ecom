use axum::response::Html;
use maud::{html, Markup};

use crate::views::{
    layout::{app_layout::create_app_layout, auth_layout::create_auth_layout},
    ui::{
        icons::chevron_left_icon, input_with_label::input_with_label,
        primary_button::primary_button,
    },
};

//..........................................................
//.LLLL.........OOOOOOO........GGGGGGG...GIIII.NNNN...NNNN..
//.LLLL........OOOOOOOOOO....GGGGGGGGGG..GIIII.NNNNN..NNNN..
//.LLLL.......OOOOOOOOOOOO..GGGGGGGGGGGG.GIIII.NNNNN..NNNN..
//.LLLL.......OOOOO..OOOOO..GGGGG..GGGGG.GIIII.NNNNNN.NNNN..
//.LLLL......LOOOO....OOOOOOGGGG....GGG..GIIII.NNNNNN.NNNN..
//.LLLL......LOOO......OOOOOGGG..........GIIII.NNNNNNNNNNN..
//.LLLL......LOOO......OOOOOGGG..GGGGGGGGGIIII.NNNNNNNNNNN..
//.LLLL......LOOO......OOOOOGGG..GGGGGGGGGIIII.NNNNNNNNNNN..
//.LLLL......LOOOO....OOOOOOGGGG.GGGGGGGGGIIII.NNNNNNNNNNN..
//.LLLL.......OOOOO..OOOOO..GGGGG....GGGGGIIII.NNNN.NNNNNN..
//.LLLLLLLLLL.OOOOOOOOOOOO..GGGGGGGGGGGG.GIIII.NNNN..NNNNN..
//.LLLLLLLLLL..OOOOOOOOOO....GGGGGGGGGG..GIIII.NNNN..NNNNN..
//.LLLLLLLLLL....OOOOOO........GGGGGGG...GIIII.NNNN...NNNN..
//..........................................................

pub async fn login_view() -> Html<String> {
    let view = html! {
        (login_form())
    };

    Html(create_app_layout(create_auth_layout(view)).into_string())
}

pub fn login_form() -> Markup {
    html! {
        div class="relative space-y-6 mx-auto w-full max-w-md" {
            div class="text-center" {
                h1 class="font-bold text-3xl text-foreground tracking-tight" { "Sign in to your account" }
                div class="flex justify-center mt-2" {
                    "Don't have an account?"
                    a id="register-link" href="/auth/register" class="ml-2 font-medium text-pretty hover:underline" {
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
                    (input_with_label("Email", "email", "email", "email", "Enter your email", ""))
                    (input_with_label("Password", "password", "password", "password", "Enter your password", ""))
                }
                (primary_button(Some("Sign In"), Some("w-full mt-3"), None))
            }
            a href="/shop/home" class="absolute flex items-center font-semibold hover:text-gray-500" {
                div class="flex translate-y-[1px]" {
                    (chevron_left_icon())
                }
                "Back To Home"
            }
        }
    }
}

//..............................................................................................
//.RRRRRRRRRR...EEEEEEEEEEE....GGGGGGG...GGIII..SSSSSSS....STTTTTTTTTTTEEEEEEEEEE.ERRRRRRRRR....
//.RRRRRRRRRRR..EEEEEEEEEEE..GGGGGGGGGG..GGIII.ISSSSSSSS...STTTTTTTTTTTEEEEEEEEEE.ERRRRRRRRRR...
//.RRRRRRRRRRR..EEEEEEEEEEE.EGGGGGGGGGGG.GGIII.ISSSSSSSSS..STTTTTTTTTTTEEEEEEEEEE.ERRRRRRRRRR...
//.RRRR...RRRRR.EEEE........EGGGG..GGGGG.GGIIIIISSS..SSSS.....TTTT....TEEE........ERRR...RRRRR..
//.RRRR...RRRRR.EEEE.......EEGGG....GGG..GGIIIIISSS...........TTTT....TEEE........ERRR...RRRRR..
//.RRRRRRRRRRR..EEEEEEEEEE.EEGG..........GGIII.ISSSSSS........TTTT....TEEEEEEEEE..ERRRRRRRRRR...
//.RRRRRRRRRRR..EEEEEEEEEE.EEGG..GGGGGGGGGGIII..SSSSSSSSS.....TTTT....TEEEEEEEEE..ERRRRRRRRRR...
//.RRRRRRRR.....EEEEEEEEEE.EEGG..GGGGGGGGGGIII....SSSSSSS.....TTTT....TEEEEEEEEE..ERRRRRRR......
//.RRRR.RRRR....EEEE.......EEGGG.GGGGGGGGGGIII.......SSSSS....TTTT....TEEE........ERRR.RRRR.....
//.RRRR..RRRR...EEEE........EGGGG....GGGGGGIIIIISS....SSSS....TTTT....TEEE........ERRR..RRRR....
//.RRRR..RRRRR..EEEEEEEEEEE.EGGGGGGGGGGG.GGIIIIISSSSSSSSSS....TTTT....TEEEEEEEEEE.ERRR..RRRRR...
//.RRRR...RRRRR.EEEEEEEEEEE..GGGGGGGGGG..GGIII.ISSSSSSSSS.....TTTT....TEEEEEEEEEE.ERRR...RRRRR..
//.RRRR....RRRR.EEEEEEEEEEE....GGGGGGG...GGIII..SSSSSSSS......TTTT....TEEEEEEEEEE.ERRR....RRRR..
//..............................................................................................

pub async fn register_view() -> Html<String> {
    let view = html! {
        (register_form())
    };

    Html(create_app_layout(create_auth_layout(view)).into_string())
}

pub fn register_form() -> Markup {
    html! {
        div class="relative space-y-6 mx-auto w-full max-w-md" {
            div class="text-center" {
                h1 class="font-bold text-3xl text-foreground tracking-tight" { "Create new account"}
                div class="flex justify-center mt-2" {
                    "Already have an account?"
                    a id="login-link" href="/auth/login" class="ml-2 font-medium text-pretty hover:underline" {
                        span { "Login" }
                    }
                }
            }
            form
                id="register-form"
                hx-post="/auth/register"
                hx-swap="none"
                "hx-on::after-request"="if(event.detail.successful) this.reset()"
                hx-disabled-elt="find button"
            {
                div class="flex flex-col gap-3" {
                    (input_with_label("Username", "text", "username", "username", "Enter your username", ""))
                    (input_with_label("Email", "email", "email", "email", "Enter your email", ""))
                    (input_with_label("Password", "password", "password", "password", "Enter your password", ""))
                }
                (primary_button(Some("Sign Up"), Some("w-full mt-3"), None))
            }
            a href="/shop/home" class="absolute flex items-center font-semibold hover:text-gray-500" {
                div class="flex translate-y-[1px]" {
                    (chevron_left_icon())
                }
                "Back To Home"
            }
        }
    }
}
