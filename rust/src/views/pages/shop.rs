use crate::{
    middleware::auth::UserExtention,
    utilities::{app_error::AppError, minify::minify_html},
    views::{
        layout::{app_layout::create_app_layout, shop_layout::create_shop_layout},
        ui::icons::{house_plug_icon, log_out_icon_small, shopping_cart_icon, user_icon},
    },
};
use axum::{extract::Path, http::StatusCode, response::Html, Extension};
use maud::{html, Markup};

struct MenuItem {
    id: &'static str,
    label: &'static str,
    path: &'static str,
}

const MENUITEMS: &[MenuItem] = &[
    MenuItem {
        id: "nav-home",
        label: "Home",
        path: "/shop/home",
    },
    MenuItem {
        id: "nav-products",
        label: "Products",
        path: "/shop/listing",
    },
    MenuItem {
        id: "nav-men",
        label: "Men",
        path: "/shop/listing",
    },
    MenuItem {
        id: "nav-women",
        label: "Women",
        path: "/shop/listing",
    },
    MenuItem {
        id: "nav-kids",
        label: "Kids",
        path: "/shop/listing",
    },
    MenuItem {
        id: "nav-footwear",
        label: "Footwear",
        path: "/shop/listing",
    },
    MenuItem {
        id: "nav-accessories",
        label: "Accessories",
        path: "/shop/listing",
    },
    MenuItem {
        id: "nav-search",
        label: "Search",
        path: "/shop/search",
    },
];

pub fn shop_header(user_extension: UserExtention) -> Markup {
    let user = user_extension.user;

    html! {
        header class="sticky top-0 z-40 w-full border-b bg-background" {
            div class="flex h-16 items-center justify-between px-4 md:px-6" {
                a
                    href="/shop/home"
                    class="flex items-center gap-2 cursor-pointer" {
                        (house_plug_icon())
                        span class="font-bold" { "Ecommerce" }
                }
                div class="hidden lg:block" {
                    nav class="flex flex-col mb-3 lg:mb-0 lg:items-center gap-6 lg:flex-row" {
                        @for item in MENUITEMS {
                            a id=(item.id) href=(item.path) class="text-sm font-medium hover:text-slate-400" {
                                (item.label)
                            }
                        }
                    }
                }
                @if let Some(user) = user {
                    div class="relative flex lg:items-center lg:flex-row flex-col gap-4" {
                        button class="border border-solid border-slate-300 p-2 rounded-md hover:bg-gray-100" {
                            (shopping_cart_icon())
                        }
                        div id="user-dropdown-container" class="flex h-10 w-10 shrink-0 overflow-hidden rounded-full bg-black" {
                           div class="cursor-pointer flex h-full w-full items-center justify-center rounded-full bg-black text-white font-extrabold" {
                            (user.username[0..1].to_uppercase())
                           }
                           div id="user-dropdown" class="hidden absolute translate-y-[105%] border border-solid border-slate-200 bottom-0 right-0 shadow-md rounded-md z-50 bg-white text-sm w-max" {
                            div class="flex flex-col divide-y" {
                                div id="user-logged-in" class="font-semibold p-3" {"Logged in as " (user.username)}
                                div class="flex p-3 gap-2 hover:bg-gray-100 cursor-pointer" {
                                    (user_icon())
                                    div {"Account"}
                                }
                                div class="flex p-3 gap-2 hover:bg-gray-100 cursor-pointer" {
                                    (log_out_icon_small())
                                    div {"Logout"}
                                }
                            }
                           }
                        }
                    }
                } @else {
                    div class="flex gap-4" {
                        a class="hover:text-slate-400" href="/auth/login" {"Login"}
                        a class="hover:text-slate-400" href="/auth/register" {"Register"}
                    }
                }
            }
        }
    }
}

pub async fn shop_view(
    Path(shop_path): Path<String>,
    Extension(user): Extension<UserExtention>,
) -> Result<Html<String>, AppError> {
    let view = match shop_path.as_str() {
        "home" => Ok(home_view()),
        "listing" => Ok(list_view()),
        _ => Err(AppError::new(
            StatusCode::NOT_FOUND,
            "Not found".to_string(),
        )),
    }?;

    Ok(Html(minify_html(
        &create_app_layout(create_shop_layout(view, user)).into_string(),
    )))
}

pub fn home_view() -> Markup {
    html! {
        div id="shop-contents" {"shopping view home"}
    }
}

pub fn list_view() -> Markup {
    html! {
        div id="shop-contents" {"shopping view list"}
    }
}
