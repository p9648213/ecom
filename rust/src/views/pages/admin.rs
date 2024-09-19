use axum::{extract::Path, http::StatusCode};
use maud::{html, Markup};

use crate::{
    utilities::app_error::AppError,
    views::{
        layout::{admin_layout::create_admin_layout, app_layout::create_app_layout},
        ui::primary_button::primary_button,
    },
};

pub struct MenuItem {
    id: &'static str,
    label: &'static str,
    path: &'static str,
    icon: &'static str,
}

const MENUITEMS: &[MenuItem] = &[
    MenuItem {
        id: "dashboard",
        label: "Dashboard",
        path: "/admin/contents/dashboard",
        icon: "/assets/images/layout-dashboard.svg",
    },
    MenuItem {
        id: "products",
        label: "Products",
        path: "/admin/contents/products",
        icon: "/assets/images/shopping-basket.svg",
    },
    MenuItem {
        id: "orders",
        label: "Orders",
        path: "/admin/contents/orders",
        icon: "/assets/images/badge-check.svg",
    },
];

pub async fn admin_view(Path(admin_path): Path<String>) -> Result<Markup, AppError> {
    let view = match admin_path.as_str() {
        "dashboard" => Ok(admin_dashboard()),
        "products" => Ok(admin_products()),
        "orders" => Ok(admin_orders()),
        _ => Err(AppError::new(
            StatusCode::NOT_FOUND,
            "Contents not found".to_string(),
        )),
    }?;

    Ok(create_app_layout(create_admin_layout(view)))
}

pub async fn admin_contents(Path(path): Path<String>) -> Result<Markup, AppError> {
    let view = match path.as_str() {
        "dashboard" => Ok(admin_dashboard()),
        "products" => Ok(admin_products()),
        "orders" => Ok(admin_orders()),
        _ => Err(AppError::new(
            StatusCode::NOT_FOUND,
            "Contents not found".to_string(),
        )),
    }?;

    Ok(view)
}

fn admin_dashboard() -> Markup {
    html! {
        div id="admin-contents" { "admin dashboard"}
    }
}

fn admin_products() -> Markup {
    html! {
        div id="admin-contents" { "admin products"}
    }
}

fn admin_orders() -> Markup {
    html! {
        div id="admin-contents" { "admin orders"}
    }
}

pub fn admin_sidebar() -> Markup {
    html! {
        aside class="hidden w-64 flex-col border-r bg-background p-6 lg:flex" {
            div class="flex cursor-pointer items-center gap-2" {
                img src="/assets/images/chart-no-axes-combined.svg" alt="admin panel" {}
                h1 class="text-2xl font-extrabold" { "Admin Panel" }
            }
            (menuitems())
        }
    }
}

pub fn admin_header() -> Markup {
    html! {
        header class="flex items-center justify-between px-4 py-3 bg-background border-b" {
            a href="#" data-drawer-trigger aria-controls="admin-drawer" aria-expanded="false" {
                (primary_button(None, Some("lg:hidden sm:block"),Some("align-justify.svg")))
            }
            (drawer())
            div class="flex flex-1 justify-end" {
                (primary_button(Some("Logout"), Some("inline-flex gap-2 items-center rounded-md px-4 py-2 text-sm font-medium shadow mt-0"), Some("log-out.svg")))
            }
        }
    }
}

fn drawer() -> Markup {
    html! {
        section class="drawer drawer--left" id="admin-drawer" data-drawer-target {
            div class="drawer__overlay" data-drawer-close tabindex="-1" {}
            div class="drawer__wrapper" {
                div class="drawer__header" {
                    div class="flex gap-3" {
                        img src="/assets/images/chart-no-axes-combined.svg" alt="admin panel"{}
                        h1 class="text-2xl font-extrabold" {
                            "Admin Panel"
                        }
                    }
                    button class="drawer__close" data-drawer-close aria-label="Close Drawer" {}
                }
                div class="drawer__content" {
                    (menuitems_drawers())
                }
            }
        }
    }
}

fn menuitems() -> Markup {
    html! {
        nav class="mt-8 flex-col flex gap-2" {
            @for item in MENUITEMS {
                div hx-get=(item.path) hx-target="#admin-contents" id=(item.id) class="group flex cursor-pointer text-xl items-center gap-2 rounded-md px-3 py-2 text-muted-foreground hover:bg-muted hover:text-foreground" {
                    img class="filter contrast-0 group-hover:contrast-100" src=(item.icon) alt=(item.label) {}
                    span { (item.label) }
                }
            }
        }
    }
}


fn menuitems_drawers() -> Markup {
    html! {
        nav class="mt-8 flex-col flex gap-2" {
            @for item in MENUITEMS {
                div hx-get=(item.path) hx-target="#admin-contents" id=(item.id) data-drawer-close class="group flex cursor-pointer text-xl items-center gap-2 rounded-md px-3 py-2 text-muted-foreground hover:bg-muted hover:text-foreground" {
                    img class="filter contrast-0 group-hover:contrast-100" src=(item.icon) alt=(item.label) {}
                    span { (item.label) }
                }
            }
        }
    }
}
