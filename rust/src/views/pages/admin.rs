use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
};
use axum_extra::extract::Multipart;
use maud::{html, Markup};
use sqlx::{Pool, Sqlite};

use crate::{
    controllers::admin::{add_product, get_all_products, get_product_by_id, update_product_by_id},
    models::product::Product,
    utilities::app_error::AppError,
    views::{
        layout::{admin_layout::create_admin_layout, app_layout::create_app_layout},
        ui::{
            icons::{
                align_justify_icon, badge_check_icon, chart_no_axes_combined_icon,
                layout_dashboard_icon, loading_icon, log_out_icon, shopping_basket_icon,
            },
            image_upload::image_upload,
            input_with_label::input_with_label,
            primary_button::primary_button,
            select::select,
            text_area::text_area,
        },
    },
};

//........................................................................
//....CCCCCCC......OOOOOOO.....NNNN...NNNN..FFFFFFFFFFIIIII....GGGGGGG....
//...CCCCCCCCC....OOOOOOOOOO...NNNNN..NNNN..FFFFFFFFFFIIIII..GGGGGGGGGG...
//..CCCCCCCCCCC..OOOOOOOOOOOO..NNNNN..NNNN..FFFFFFFFFFIIIII.GGGGGGGGGGGG..
//..CCCC...CCCCC.OOOOO..OOOOO..NNNNNN.NNNN..FFFF......IIIII.GGGGG..GGGGG..
//.CCCC.....CCC.OOOOO....OOOOO.NNNNNN.NNNN..FFFF......IIIIIGGGGG....GGG...
//.CCCC.........OOOO......OOOO.NNNNNNNNNNN..FFFFFFFFF.IIIIIGGGG...........
//.CCCC.........OOOO......OOOO.NNNNNNNNNNN..FFFFFFFFF.IIIIIGGGG..GGGGGGG..
//.CCCC.........OOOO......OOOO.NNNNNNNNNNN..FFFFFFFFF.IIIIIGGGG..GGGGGGG..
//.CCCC.....CCC.OOOOO....OOOOO.NNNNNNNNNNN..FFFF......IIIIIGGGGG.GGGGGGG..
//..CCCC...CCCCC.OOOOO..OOOOO..NNNN.NNNNNN..FFFF......IIIII.GGGGG....GGG..
//..CCCCCCCCCCC..OOOOOOOOOOOO..NNNN..NNNNN..FFFF......IIIII.GGGGGGGGGGGG..
//...CCCCCCCCCC...OOOOOOOOOO...NNNN..NNNNN..FFFF......IIIII..GGGGGGGGGG...
//....CCCCCCC.......OOOOOO.....NNNN...NNNN..FFFF......IIIII....GGGGGGG....
//........................................................................

pub enum MenuItemIcon {
    Dashboard,
    Products,
    Orders,
}

pub struct MenuItem {
    id: &'static str,
    label: &'static str,
    path: &'static str,
    icon: MenuItemIcon,
    url: &'static str,
}

pub struct SelectOption {
    pub value: &'static str,
    pub label: &'static str,
}

const MENUITEMS: &[MenuItem] = &[
    MenuItem {
        id: "admin-dashboard",
        label: "Dashboard",
        path: "/contents/admin/dashboard",
        icon: MenuItemIcon::Dashboard,
        url: "/admin/dashboard",
    },
    MenuItem {
        id: "admin-products",
        label: "Products",
        path: "/contents/admin/products",
        icon: MenuItemIcon::Products,
        url: "/admin/products",
    },
    MenuItem {
        id: "admin-orders",
        label: "Orders",
        path: "/contents/admin/orders",
        icon: MenuItemIcon::Orders,
        url: "/admin/orders",
    },
];

const CATEGORIES_SELECT_OPTIONS: &[SelectOption] = &[
    SelectOption {
        value: "men",
        label: "Men",
    },
    SelectOption {
        value: "women",
        label: "Women",
    },
    SelectOption {
        value: "kids",
        label: "Kids",
    },
    SelectOption {
        value: "accessories",
        label: "Accessories",
    },
    SelectOption {
        value: "footwear",
        label: "Footwear",
    },
];

const BRAND_SELECT_OPTIONS: &[SelectOption] = &[
    SelectOption {
        value: "adidas",
        label: "Adidas",
    },
    SelectOption {
        value: "nike",
        label: "Nike",
    },
    SelectOption {
        value: "puma",
        label: "Puma",
    },
    SelectOption {
        value: "levi",
        label: "Levi's",
    },
    SelectOption {
        value: "zara",
        label: "Zara",
    },
    SelectOption {
        value: "h&m",
        label: "H&M",
    },
];

//.............................................................
//.....AAAAA.....DDDDDDDDD...MMMMMM...MMMMMMIIIII.NNNN...NNNN..
//.....AAAAA.....DDDDDDDDDD..MMMMMM...MMMMMMIIIII.NNNNN..NNNN..
//....AAAAAA.....DDDDDDDDDDD.MMMMMM...MMMMMMIIIII.NNNNN..NNNN..
//....AAAAAAA....DDDD...DDDD.MMMMMMM.MMMMMMMIIIII.NNNNNN.NNNN..
//...AAAAAAAA....DDDD....DDDDMMMMMMM.MMMMMMMIIIII.NNNNNN.NNNN..
//...AAAAAAAA....DDDD....DDDDMMMMMMM.MMMMMMMIIIII.NNNNNNNNNNN..
//...AAAA.AAAA...DDDD....DDDDMMMMMMMMMMMMMMMIIIII.NNNNNNNNNNN..
//..AAAAAAAAAA...DDDD....DDDDMMMMMMMMMMMMMMMIIIII.NNNNNNNNNNN..
//..AAAAAAAAAAA..DDDD....DDDDMMMMMMMMMMMMMMMIIIII.NNNNNNNNNNN..
//..AAAAAAAAAAA..DDDD...DDDDDMMMM.MMMMM.MMMMIIIII.NNNN.NNNNNN..
//.AAAA....AAAA..DDDDDDDDDDD.MMMM.MMMMM.MMMMIIIII.NNNN..NNNNN..
//.AAAA.....AAAA.DDDDDDDDDD..MMMM.MMMMM.MMMMIIIII.NNNN..NNNNN..
//.AAAA.....AAAA.DDDDDDDDD...MMMM.MMMMM.MMMMIIIII.NNNN...NNNN..
//.............................................................

pub async fn admin_view(Path(admin_path): Path<String>) -> Result<Html<String>, AppError> {
    let view = match admin_path.as_str() {
        "dashboard" => Ok(admin_dashboard()),
        "products" => Ok(admin_products()),
        "orders" => Ok(admin_orders()),
        _ => Err(AppError::new(
            StatusCode::NOT_FOUND,
            "Not found".to_string(),
        )),
    }?;

    Ok(Html(
        create_app_layout(create_admin_layout(view)).into_string(),
    ))
}

pub async fn admin_contents(Path(path): Path<String>) -> Result<Html<String>, AppError> {
    let view = match path.as_str() {
        "dashboard" => Ok(admin_dashboard()),
        "products" => Ok(admin_products()),
        "orders" => Ok(admin_orders()),
        _ => Err(AppError::new(
            StatusCode::NOT_FOUND,
            "Contents not found".to_string(),
        )),
    }?;

    Ok(Html(view.into_string()))
}

pub fn admin_sidebar() -> Markup {
    html! {
        aside class="lg:flex flex-col hidden bg-background p-6 border-r w-64" {
            div class="flex items-center gap-2 cursor-pointer" {
                (chart_no_axes_combined_icon())
                h1 class="font-extrabold text-2xl" { "Admin Panel" }
            }
            (menuitems())
        }
    }
}

fn menuitem_icon(item: &MenuItemIcon) -> Markup {
    match item {
        MenuItemIcon::Dashboard => layout_dashboard_icon(),
        MenuItemIcon::Products => shopping_basket_icon(),
        MenuItemIcon::Orders => badge_check_icon(),
    }
}

fn menuitems() -> Markup {
    html! {
        nav class="flex flex-col gap-2 mt-8" {
            @for item in MENUITEMS {
                div
                    hx-get=(item.path)
                    hx-target="#admin-contents"
                    hx-swap="outerHTML"
                    hx-push-url=(item.url)
                    id=(item.id)
                    class="flex items-center gap-2 hover:bg-muted px-3 py-2 rounded-md text-muted-foreground text-xl hover:text-foreground cursor-pointer group" {
                        (menuitem_icon(&item.icon))
                        span { (item.label) }
                }
            }
        }
    }
}

fn admin_drawer() -> Markup {
    html! {
        section class="drawer--left drawer" id="admin-drawer" data-drawer-target {
            div class="drawer__overlay" data-drawer-close tabindex="-1" {}
            div class="drawer__wrapper" {
                div class="drawer__header" {
                    div class="flex gap-3" {
                        (chart_no_axes_combined_icon())
                        h1 class="font-extrabold text-2xl" {
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

fn menuitems_drawers() -> Markup {
    html! {
        nav class="flex flex-col gap-2 mt-8" {
            @for item in MENUITEMS {
                div
                    hx-get=(item.path)
                    hx-target="#admin-contents"
                    hx-swap="outerHTML"
                    hx-push-url=(item.url)
                    id=(format!("drawer-{}", item.id))
                    data-drawer-close
                    class="flex items-center gap-2 hover:bg-muted px-3 py-2 rounded-md text-muted-foreground text-xl hover:text-foreground cursor-pointer group" {
                        (menuitem_icon(&item.icon))
                        span { (item.label) }
                }
            }
        }
    }
}

pub fn admin_header() -> Markup {
    html! {
        header class="flex justify-between items-center bg-background px-4 py-3 border-b" {
            a href="#" data-drawer-trigger aria-controls="admin-drawer" aria-expanded="false" {
                (primary_button(None, Some("lg:hidden sm:block"),Some(align_justify_icon())))
            }
            (admin_drawer())
            div hx-post="/auth/logout" hx-swap="none" class="flex flex-1 justify-end" {
                (primary_button(Some("Logout"), Some("inline-flex gap-2 items-center rounded-md px-4 py-2 text-sm font-medium shadow mt-0"), Some(log_out_icon())))
            }
        }
    }
}

//................................................................................
//.DDDDDDDDD....BBBBBBBBBB.....OOOOOOO........AAAAA.....RRRRRRRRRR...DDDDDDDDD....
//.DDDDDDDDDD...BBBBBBBBBBB...OOOOOOOOOO......AAAAA.....RRRRRRRRRRR..DDDDDDDDDD...
//.DDDDDDDDDDD..BBBBBBBBBBB..OOOOOOOOOOOO....AAAAAA.....RRRRRRRRRRR..DDDDDDDDDDD..
//.DDDD...DDDD..BBBB...BBBB..OOOOO..OOOOO....AAAAAAA....RRRR...RRRRR.DDDD...DDDD..
//.DDDD....DDDD.BBBB...BBBB.BOOOO....OOOOO..AAAAAAAA....RRRR...RRRRR.DDDD....DDD..
//.DDDD....DDDD.BBBBBBBBBBB.BOOO......OOOO..AAAAAAAA....RRRRRRRRRRR..DDDD....DDD..
//.DDDD....DDDD.BBBBBBBBBB..BOOO......OOOO..AAAA.AAAA...RRRRRRRRRRR..DDDD....DDD..
//.DDDD....DDDD.BBBBBBBBBBB.BOOO......OOOO.AAAAAAAAAA...RRRRRRRR.....DDDD....DDD..
//.DDDD....DDDD.BBBB....BBBBBOOOO....OOOOO.AAAAAAAAAAA..RRRR.RRRR....DDDD....DDD..
//.DDDD...DDDDD.BBBB....BBBB.OOOOO..OOOOO..AAAAAAAAAAA..RRRR..RRRR...DDDD...DDDD..
//.DDDDDDDDDDD..BBBBBBBBBBBB.OOOOOOOOOOOO.OAAA....AAAA..RRRR..RRRRR..DDDDDDDDDDD..
//.DDDDDDDDDD...BBBBBBBBBBB...OOOOOOOOOO..OAAA.....AAAA.RRRR...RRRRR.DDDDDDDDDD...
//.DDDDDDDDD....BBBBBBBBBB......OOOOOO...OOAAA.....AAAA.RRRR....RRRR.DDDDDDDDD....
//................................................................................

fn admin_dashboard() -> Markup {
    html! {
        div id="admin-contents" class="flex mb-5 w-full" { "admin dashboard"}
    }
}

//..................................................................
//.PPPPPPPPP...PRRRRRRRRR.....OOOOOOO.....ODDDDDDDD.....SSSSSSS.....
//.PPPPPPPPPP..PRRRRRRRRRR...OOOOOOOOOO...ODDDDDDDDD...DSSSSSSSS....
//.PPPPPPPPPPP.PRRRRRRRRRR..ROOOOOOOOOOO..ODDDDDDDDDD..DSSSSSSSSS...
//.PPPP...PPPP.PRRR...RRRRR.ROOOO..OOOOO..ODDD...DDDD.DDSSS..SSSS...
//.PPPP...PPPP.PRRR...RRRRRRROOO....OOOOO.ODDD....DDDDDDSSS.........
//.PPPPPPPPPPP.PRRRRRRRRRR.RROO......OOOO.ODDD....DDDD.DSSSSSS......
//.PPPPPPPPPP..PRRRRRRRRRR.RROO......OOOO.ODDD....DDDD..SSSSSSSSS...
//.PPPPPPPPP...PRRRRRRR....RROO......OOOO.ODDD....DDDD....SSSSSSS...
//.PPPP........PRRR.RRRR...RROOO....OOOOO.ODDD....DDDD.......SSSSS..
//.PPPP........PRRR..RRRR...ROOOO..OOOOO..ODDD...DDDDDDDSS....SSSS..
//.PPPP........PRRR..RRRRR..ROOOOOOOOOOO..ODDDDDDDDDD.DDSSSSSSSSSS..
//.PPPP........PRRR...RRRRR..OOOOOOOOOO...ODDDDDDDDD...DSSSSSSSSS...
//.PPPP........PRRR....RRRR....OOOOOO.....ODDDDDDDD.....SSSSSSSS....
//..................................................................

fn admin_products() -> Markup {
    html! {
        div id="admin-contents" {
            div class="flex justify-end mb-5 w-full" {
                div hx-on:click=r#"window.resetAddProductForm()"# data-drawer-trigger aria-controls="products-drawer" {
                    (primary_button(Some("Add New Product"), None, None))
                }
            }
            div
                hx-get="/admin/products/all"
                hx-trigger="load"
                hx-target="#admin-product-list"
                hx-swap="outerHTML"
                class="h-full"
                id="admin-product-list" {
                    div class="flex justify-center items-center h-full" {
                        (loading_icon())
                    }
            }
            (products_drawer())
        }
    }
}

fn products_drawer() -> Markup {
    html! {
        section class="drawer--right drawer" id="products-drawer" data-drawer-target {
            div class="drawer__overlay" data-drawer-close tabindex="-1" {}
            div class="drawer__wrapper" style="width: 26rem" {
                div class="drawer__header" {
                    div id="products-drawer-header" class="flex gap-3" {
                        "Add New Product"
                    }
                    button class="drawer__close" data-drawer-close aria-label="Close Drawer" {}
                }
                div class="drawer__content" {
                    div class="mt-3" {
                        (add_product_form())
                    }
                }
            }
        }
    }
}

fn add_product_form() -> Markup {
    html! {
        form
            id="add-product-form"
            hx-post="/admin/products/add"
            hx-target="#admin-product-list"
            hx-swap="afterbegin"
            hx-on--after-request="if(event.detail.successful === true) {document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }))}"
            enctype="multipart/form-data"
        {
            div class="flex flex-col gap-3" {
                (image_upload("image"))
                (input_with_label("Title", "text", "title", "add-product-title", "Enter product title", ""))
                (text_area("Description", "description", "add-product-description", "Enter product description", ""))
                (select("Category", "category", "add-product-category", CATEGORIES_SELECT_OPTIONS, CATEGORIES_SELECT_OPTIONS[0].value))
                (select("Brand", "brand", "add-product-brand", BRAND_SELECT_OPTIONS, BRAND_SELECT_OPTIONS[0].value))
                (input_with_label("Price", "number", "price", "add-product-price", "Enter product price", ""))
                (input_with_label("Sale Price", "number", "sale_price", "add-product-sale-price", "Enter sale price (optional)", ""))
                (input_with_label("Total Stock", "number", "total_stock", "add-product-total-stock", "Enter total stock", ""))
                (primary_button(Some("Add"), Some("mt-2 w-full"), None))
            }
        }
    }
}

pub async fn edit_product_form(
    State(pool): State<Pool<Sqlite>>,
    Path(product_id): Path<i32>,
) -> Result<Html<String>, AppError> {
    let product = get_product_by_id(pool, product_id).await?;
    let put_link = format!("/admin/products/{}", product.id);
    let target_id = format!("#product-{}", product.id);

    let view = html! {
        form
            id="add-product-form"
            hx-put=(put_link)
            hx-target=(target_id)
            hx-swap="outerHTML"
            hx-on--after-request="if(event.detail.successful === true) {document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }))}"
            enctype="multipart/form-data"
        {
            div class="flex flex-col gap-3" {
                (image_upload("image"))
                (input_with_label("Title", "text", "title", "edit-product-title", "Enter product title", product.title.as_str()))
                (text_area("Description", "description", "edit-product-description", "Enter product description", product.description.as_str()))
                (select("Category", "category", "edit-product-category", CATEGORIES_SELECT_OPTIONS, product.category.as_str()))
                (select("Brand", "brand", "edit-product-brand", BRAND_SELECT_OPTIONS, product.brand.as_str()))
                (input_with_label("Price", "number", "price", "edit-product-price", "Enter product price", product.price.to_string().as_str()))
                (input_with_label("Sale Price", "number", "sale_price", "edit-product-sale-price", "Enter sale price (optional)", product.sale_price.to_string().as_str()))
                (input_with_label("Total Stock", "number", "total_stock", "edit-product-total-stock", "Enter total stock", product.total_stock.to_string().as_str()))
                (primary_button(Some("Edit"), Some("mt-2 w-full"), None))
            }
        }
        div type="submit" hx-swap-oob="true" id="products-drawer-header" class="flex gap-3" {
            "Edit Product"
        }
    };

    Ok(Html(view.into_string()))
}

pub async fn admin_product_list(
    State(pool): State<Pool<Sqlite>>,
) -> Result<Html<String>, AppError> {
    let products = get_all_products(&pool).await?;

    let view = html! {
        @match products.len() {
            0 => {
                div class="flex flex-col gap-3 p-4 font-bold text-center" {
                    "No products found"
                }
            }
            _ => {
                div id="admin-product-list" class="gap-4 grid md:grid-cols-3 lg:grid-cols-4" {
                    @for product in products {
                        (product_tile(product))
                    }
                }
            }
        }
    };

    Ok(Html(view.into_string()))
}

pub fn product_tile(product: Product) -> Markup {
    let edit_product_link = format!("/admin/products/{}/edit", product.id);
    let delete_product_link = format!("/admin/products/{}", product.id);
    let product_div_id = format!("product-{}", product.id);
    let delete_target_id = format!("#product-{}", product.id);

    html! {
        div id=(product_div_id) class="bg-card shadow-sm mx-auto border rounded-lg w-full max-w-sm text-card-foreground" {
            div {
                div  {
                    (product_detail(product))
                }
                div class="flex justify-between items-center p-6 pt-0" {
                    div
                        hx-get=(edit_product_link)
                        hx-swap="outerHTML"
                        hx-target="#add-product-form"
                        data-drawer-trigger
                        aria-controls="products-drawer" {
                            (primary_button(Some("Edit"), None, None))
                    }
                    div
                        hx-delete=(delete_product_link)
                        hx-target=(delete_target_id)
                        hx-swap="outerHTML" {
                            (primary_button(Some("Delete"), None, None))
                    }
                }
            }
        }
    }
}

pub fn product_detail(product: Product) -> Markup {
    html! {
        div class="relative" {
            img class="rounded-t-lg w-full h-[300px] object-contain" onerror=r#"this.onerror=null;this.src="/assets/images/noimage.jpg""# src=(format!("/admin/products/{}/image", product.id)) alt=(product.title);
        }
        div class="p-6 pt-0" {
            h2 class="mt-2 mb-2 font-bold text-xl" {
                (product.title)
            }
            div class="flex justify-between items-center mb-2" {
                @if product.sale_price > 0 {
                    span class="line-through" {
                        (product.price)
                    }
                }@else {
                    span class="font-semibold text-lg text-primary" {
                        (product.price)
                    }
                }

                @if product.sale_price > 0 {
                    span class="font-bold text-lg" {
                        (product.sale_price)
                    }
                }
            }
        }
    }
}

pub async fn new_product(
    State(pool): State<Pool<Sqlite>>,
    mutipart_form: Multipart,
) -> Result<Html<String>, AppError> {
    let new_product = add_product(pool, mutipart_form).await?;
    Ok(Html(product_tile(new_product).into_string()))
}

pub async fn edit_product(
    Path(product_id): Path<i32>,
    State(pool): State<Pool<Sqlite>>,
    mutipart_form: Multipart,
) -> Result<Html<String>, AppError> {
    let edit_product = update_product_by_id(product_id, pool, mutipart_form).await?;
    Ok(Html(product_tile(edit_product).into_string()))
}

//................................................................................
//....OOOOOOO.....RRRRRRRRRR...DDDDDDDDD....EEEEEEEEEEE.RRRRRRRRRR....SSSSSSS.....
//...OOOOOOOOOO...RRRRRRRRRRR..DDDDDDDDDD...EEEEEEEEEEE.RRRRRRRRRRR..SSSSSSSSS....
//..OOOOOOOOOOOO..RRRRRRRRRRR..DDDDDDDDDDD..EEEEEEEEEEE.RRRRRRRRRRR..SSSSSSSSSS...
//..OOOOO..OOOOO..RRRR...RRRRR.DDDD...DDDD..EEEE........RRRR...RRRRRRSSSS..SSSS...
//.OOOOO....OOOOO.RRRR...RRRRR.DDDD....DDDD.EEEE........RRRR...RRRRRRSSSS.........
//.OOOO......OOOO.RRRRRRRRRRR..DDDD....DDDD.EEEEEEEEEE..RRRRRRRRRRR..SSSSSSS......
//.OOOO......OOOO.RRRRRRRRRRR..DDDD....DDDD.EEEEEEEEEE..RRRRRRRRRRR...SSSSSSSSS...
//.OOOO......OOOO.RRRRRRRR.....DDDD....DDDD.EEEEEEEEEE..RRRRRRRR........SSSSSSS...
//.OOOOO....OOOOO.RRRR.RRRR....DDDD....DDDD.EEEE........RRRR.RRRR..........SSSSS..
//..OOOOO..OOOOO..RRRR..RRRR...DDDD...DDDDD.EEEE........RRRR..RRRR..RSSS....SSSS..
//..OOOOOOOOOOOO..RRRR..RRRRR..DDDDDDDDDDD..EEEEEEEEEEE.RRRR..RRRRR.RSSSSSSSSSSS..
//...OOOOOOOOOO...RRRR...RRRRR.DDDDDDDDDD...EEEEEEEEEEE.RRRR...RRRRR.SSSSSSSSSS...
//.....OOOOOO.....RRRR....RRRR.DDDDDDDDD....EEEEEEEEEEE.RRRR....RRRR..SSSSSSSS....
//................................................................................

fn admin_orders() -> Markup {
    html! {
        div id="admin-contents" class="flex mb-5 w-full" {"admin orders"}
    }
}
