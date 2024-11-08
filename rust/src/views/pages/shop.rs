use crate::{
    controllers::admin::get_all_products,
    middleware::auth::UserExtention,
    models::product::Product,
    utilities::app_error::AppError,
    views::{
        layout::{app_layout::create_app_layout, shop_layout::create_shop_layout},
        pages::admin::product_tile,
        ui::{
            icons::{
                arrow_up_down_icon, house_plug_icon, log_out_icon_small, shopping_cart_icon,
                user_icon,
            },
            primary_button::outline_button,
        },
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    Extension,
};
use maud::{html, Markup, PreEscaped};
use sqlx::{Pool, Sqlite};

struct MenuItem {
    id: &'static str,
    label: &'static str,
    path: &'static str,
    url: &'static str,
}

struct FilterOption {
    id: &'static str,
    label: &'static str,
}

struct SortOption {
    id: &'static str,
    label: &'static str,
}

//........................................................................
//.....CCCCCC.......OOOOOO.....NNNN....NNN..FFFFFFFFFF.III.....GGGGGG.....
//...CCCCCCCCC....OOOOOOOOOO...NNNN....NNN..FFFFFFFFFF.III...GGGGGGGGGG...
//..CCCCCCCCCCC..OOOOOOOOOOOO..NNNNN...NNN..FFFFFFFFFF.III...GGGGGGGGGGG..
//..CCCC...CCCC..OOOO....OOOO..NNNNN...NNN..FFF........III..GGGG....GGGG..
//..CCC.....CC...OOO......OOO..NNNNNN..NNN..FFF........III..GGG......GG...
//.CCCC.........OOOO......OOOO.NNNNNNN.NNN..FFFFFFFFF..III.GGGG...........
//.CCCC.........OOOO......OOOO.NNN.NNN.NNN..FFFFFFFFF..III.GGGG...GGGGGG..
//.CCCC.........OOOO......OOOO.NNN.NNNNNNN..FFFFFFFFF..III.GGGG...GGGGGG..
//..CCC.....CC...OOO......OOO..NNN..NNNNNN..FFF........III..GGG...GGGGGG..
//..CCCC...CCCC..OOOO....OOOO..NNN..NNNNNN..FFF........III..GGGG.....GGG..
//..CCCCCCCCCCC..OOOOOOOOOOOO..NNN...NNNNN..FFF........III...GGGGGGGGGGG..
//...CCCCCCCCC....OOOOOOOOOO...NNN....NNNN..FFF........III...GGGGGGGGGG...
//.....CCCCCC.......OOOOOO.....NNN....NNNN..FFF........III.....GGGGGG.....
//........................................................................

const MENUITEMS: &[MenuItem] = &[
    MenuItem {
        id: "nav-home",
        label: "Home",
        path: "/contents/shop/home",
        url: "/shop/home",
    },
    MenuItem {
        id: "nav-products",
        label: "Products",
        path: "/contents/shop/listing",
        url: "/shop/listing",
    },
    MenuItem {
        id: "nav-men",
        label: "Men",
        url: "/shop/listing",
        path: "/contents/shop/listing",
    },
    MenuItem {
        id: "nav-women",
        label: "Women",
        url: "/shop/listing",
        path: "/contents/shop/listing",
    },
    MenuItem {
        id: "nav-kids",
        label: "Kids",
        url: "/shop/listing",
        path: "/contents/shop/listing",
    },
    MenuItem {
        id: "nav-footwear",
        label: "Footwear",
        url: "/shop/listing",
        path: "contents/shop/listing",
    },
    MenuItem {
        id: "nav-accessories",
        label: "Accessories",
        url: "/shop/listing",
        path: "/contents/shop/listing",
    },
    MenuItem {
        id: "nav-search",
        label: "Search",
        url: "/shop/search",
        path: "/contents/shop/listing",
    },
];

const FILTER_OPTION_BRAND: &[FilterOption] = &[
    FilterOption {
        id: "men",
        label: "Men",
    },
    FilterOption {
        id: "women",
        label: "Women",
    },
    FilterOption {
        id: "kids",
        label: "Kids",
    },
    FilterOption {
        id: "accessories",
        label: "Accessories",
    },
    FilterOption {
        id: "footwear",
        label: "Footwear",
    },
];

const FILTER_OPTION_CATEGORY: &[FilterOption] = &[
    FilterOption {
        id: "nike",
        label: "Nike",
    },
    FilterOption {
        id: "adidas",
        label: "Adidas",
    },
    FilterOption {
        id: "puma",
        label: "Puma",
    },
    FilterOption {
        id: "levi",
        label: "Levi",
    },
    FilterOption {
        id: "zara",
        label: "Zara",
    },
    FilterOption {
        id: "h&m",
        label: "H&M",
    },
];

const SORT_OPTION: &[SortOption] = &[
    SortOption {
        id: "price-lowtohigh",
        label: "Price: Low to High",
    },
    SortOption {
        id: "price-hightolow",
        label: "Price: High to Low",
    },
    SortOption {
        id: "title-atoz",
        label: "Title: A to Z",
    },
    SortOption {
        id: "title-ztoa",
        label: "Title: Z to A",
    },
];

pub fn shop_header(user_extension: UserExtention) -> Markup {
    let user = user_extension.user;

    html! {
        (PreEscaped(r#"
            <script type="module">
                import {setupUserDropdown} from "/assets/js/shop.js";
                setupUserDropdown();
            </script>
        "#))
        header class="top-0 z-40 sticky bg-background border-b w-full" {
            div class="flex justify-between items-center px-4 md:px-6 h-16" {
                div
                    hx-get="/contents/shop/home"
                    hx-target="#shop-contents"
                    hx-swap="outerHTML"
                    hx-push-url="/shop/home"
                    class="flex items-center gap-2 cursor-pointer" {
                        (house_plug_icon())
                        span class="font-bold" { "Ecommerce" }
                }
                div class="lg:block hidden" {
                    nav class="flex lg:flex-row flex-col lg:items-center gap-6 mb-3 lg:mb-0" {
                        @for item in MENUITEMS {
                            div
                                hx-get=(item.path)
                                hx-target="#shop-contents"
                                hx-swap="outerHTML"
                                hx-push-url=(item.url)
                                id=(item.id)
                                href=(item.path)
                                class="font-medium text-sm hover:text-slate-400 cursor-pointer"
                            {
                                (item.label)
                            }
                        }
                    }
                }
                @if let Some(user) = user {
                    div class="relative flex lg:flex-row flex-col lg:items-center gap-4" {
                        button class="border-slate-300 hover:bg-gray-100 p-2 border border-solid rounded-md" {
                            (shopping_cart_icon())
                        }
                        div id="user-dropdown-container" class="flex bg-black rounded-full w-10 h-10 overflow-hidden shrink-0" {
                           div class="flex justify-center items-center bg-black rounded-full w-full h-full font-extrabold text-white cursor-pointer" {
                            (user.username[0..1].to_uppercase())
                           }
                           div id="user-dropdown" class="right-0 bottom-0 z-50 absolute border-slate-200 hidden bg-white shadow-md border border-solid rounded-md w-max text-sm translate-y-[105%]" {
                            div class="flex flex-col divide-y" {
                                div id="user-logged-in" class="p-3 font-semibold" {"Logged in as " (user.username)}
                                div class="flex gap-2 hover:bg-gray-100 p-3 cursor-pointer" {
                                    (user_icon())
                                    div {"Account"}
                                }
                                div hx-post="/auth/logout" hx-swap="none" class="flex gap-2 hover:bg-gray-100 p-3 cursor-pointer" {
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
    State(pool): State<Pool<Sqlite>>,
) -> Result<Html<String>, AppError> {
    let view = match shop_path.as_str() {
        "home" => Ok(home_view()),
        "listing" => {
            let products = get_all_products(&pool).await?;
            Ok(list_view(products))
        }
        _ => Err(AppError::new(
            StatusCode::NOT_FOUND,
            "Not found".to_string(),
        )),
    }?;

    Ok(Html(
        create_app_layout(create_shop_layout(view, user)).into_string(),
    ))
}

pub async fn shop_content(
    Path(path): Path<String>,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Html<String>, AppError> {
    let view = match path.as_str() {
        "home" => Ok(home_view()),
        "listing" => {
            let products = get_all_products(&pool).await?;
            Ok(list_view(products))
        }
        _ => Err(AppError::new(
            StatusCode::NOT_FOUND,
            "Not found".to_string(),
        )),
    }?;

    Ok(Html(view.into_string()))
}

//.......................................................
//.HHH.....HHH.....OOOOOO.....MMMMM...MMMMM..EEEEEEEEEE..
//.HHH.....HHH...OOOOOOOOOO...MMMMM...MMMMM..EEEEEEEEEE..
//.HHH.....HHH..OOOOOOOOOOOO..MMMMM...MMMMM..EEEEEEEEEE..
//.HHH.....HHH..OOOO....OOOO..MMMMM...MMMMM..EEE.........
//.HHH.....HHH..OOO......OOO..MMMMMM.MMMMMM..EEE.........
//.HHHHHHHHHHH.HOOO......OOOO.MMMMMM.MMMMMM..EEEEEEEEEE..
//.HHHHHHHHHHH.HOOO......OOOO.MMMMMM.MMMMMM..EEEEEEEEEE..
//.HHHHHHHHHHH.HOOO......OOOO.MMMMMMMMMMMMM..EEEEEEEEEE..
//.HHH.....HHH..OOO......OOO..MMM.MMMMM.MMM..EEE.........
//.HHH.....HHH..OOOO....OOOO..MMM.MMMMM.MMM..EEE.........
//.HHH.....HHH..OOOOOOOOOOOO..MMM.MMMMM.MMM..EEEEEEEEEE..
//.HHH.....HHH...OOOOOOOOOO...MMM..MMMM.MMM..EEEEEEEEEE..
//.HHH.....HHH.....OOOOOO.....MMM..MMM..MMM..EEEEEEEEEE..
//.......................................................

pub fn home_view() -> Markup {
    html! {
        div id="shop-contents" {"shopping view home"}
    }
}

//..........................................................................................
//.PPPPPPPPP...RRRRRRRRR.......OOOOOO.....DDDDDDDDD....UUU.....UUU.....CCCCCC....TTTTTTTTT..
//.PPPPPPPPPP..RRRRRRRRRRR...OOOOOOOOOO...DDDDDDDDDD...UUU.....UUU...CCCCCCCCC...TTTTTTTTT..
//.PPPPPPPPPP..RRRRRRRRRRR..OOOOOOOOOOOO..DDDDDDDDDDD..UUU.....UUU..CCCCCCCCCCC..TTTTTTTTT..
//.PPP....PPPP.RRR.....RRR..OOOO....OOOO..DDD....DDDD..UUU.....UUU..CCCC...CCCC......TTT....
//.PPP....PPPP.RRR.....RRR..OOO......OOO..DDD.....DDD..UUU.....UUU..CCC.....CC.......TTT....
//.PPPPPPPPPP..RRRRRRRRRRR.ROOO......OOOO.DDD.....DDDD.UUU.....UUU.UCCC..............TTT....
//.PPPPPPPPPP..RRRRRRRRRR..ROOO......OOOO.DDD.....DDDD.UUU.....UUU.UCCC..............TTT....
//.PPPPPPPPP...RRRRRRRR....ROOO......OOOO.DDD.....DDDD.UUU.....UUU.UCCC..............TTT....
//.PPP.........RRR..RRRR....OOO......OOO..DDD.....DDD..UUU.....UUU..CCC.....CC.......TTT....
//.PPP.........RRR...RRRR...OOOO....OOOO..DDD....DDDD..UUUU...UUUU..CCCC...CCCC......TTT....
//.PPP.........RRR....RRRR..OOOOOOOOOOOO..DDDDDDDDDDD..UUUUUUUUUUU..CCCCCCCCCCC......TTT....
//.PPP.........RRR....RRRR...OOOOOOOOOO...DDDDDDDDDD....UUUUUUUUU....CCCCCCCCC.......TTT....
//.PPP.........RRR.....RRRR....OOOOOO.....DDDDDDDDD......UUUUUUU.......CCCCCC........TTT....
//..........................................................................................

pub fn list_view(products: Vec<Product>) -> Markup {
    html! {
        (PreEscaped(r#"
            <script type="module">
                import {setupSortbyDropdown} from "/assets/js/shop.js";
                setupSortbyDropdown();
            </script>
        "#))
        div id="shop-contents" class="gap-6 grid grid-cols-1 md:grid-cols-[300px_1fr] p-4 md:p-6" {
            (product_filter())
            div class="bg-background shadow-sm rounded-lg w-full" {
                div class="flex justify-between items-center p-4 border-b" {
                    h2 class="font-extrabold text-lg" { "All Products" }
                    div class="relative flex items-center gap-3" {
                        span class="text-muted-foreground" {
                            (products.len()) " Products"
                        }
                        div id="sortby-dropdown-container" {
                            (outline_button(Some("Sort by"), Some("flex items-center gap-1"), Some(arrow_up_down_icon())))
                            div id="sortby-dropdown" class="right-0 bottom-0 z-50 absolute border-slate-200 hidden bg-white shadow-md border border-solid rounded-md w-max text-sm translate-y-[105%]" {
                                div class="flex flex-col gap-2 px-2 py-2" {
                                    @for sort_option in SORT_OPTION {
                                        div id=(sort_option.id) class="hover:bg-gray-100 px-2 py-1 rounded-sm w-full cursor-pointer" {
                                            (sort_option.label)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div class="gap-4 grid sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 grid-col-1 p-4" {
                    @for product in products {
                        (product_tile(product))
                    }
                }
            }
        }
    }
}

pub fn product_filter() -> Markup {
    html! {
        div class="bg-background shadow-sm rounded-lg" {
            div class="p-4 border-b" {
                h2 class="font-extrabold text-lg" {
                    "Filters"
                }
            }
            div class="space-y-4 p-4" {
                div {
                    h3 class="font-bold text-base" {
                        "Category"
                    }
                    div class="gap-2 grid mt-2"{
                        @for brand in FILTER_OPTION_BRAND {
                            div class="flex items-center gap-2" {
                                input class="rounded-sm" type="checkbox" name=(brand.id) value=(brand.label);
                                label for=(brand.id) class="text-sm" {
                                    (brand.label)
                                }
                            }
                        }
                    }
                }
                div class="border-gray-200 my-4 border-t" {}
                div {
                    h3 class="font-bold text-base" {
                        "Category"
                    }
                    div class="gap-2 grid mt-2"{
                        @for category in FILTER_OPTION_CATEGORY {
                            div class="flex items-center gap-2" {
                                input class="rounded-sm" type="checkbox" name=(category.id) value=(category.label);
                                label for=(category.id) class="text-sm" {
                                    (category.label)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
