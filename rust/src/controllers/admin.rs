use axum::{
    body::Bytes,
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
};
use axum_extra::extract::Multipart;
use maud::Markup;
use sqlx::{Pool, Sqlite};

use crate::{
    models::{image::Image, product::Product},
    utilities::app_error::AppError,
    views::pages::admin::product_tile,
};

#[derive(Debug)]
pub struct AddProductForm {
    pub title: String,
    pub description: String,
    pub category: String,
    pub brand: String,
    pub price: String,
    pub sale_price: String,
    pub total_stock: String,
    pub image_data: Bytes,
    pub image_name: String,
    pub image_content_type: String,
}

//.........................................
//.....AAAAA.....DDDDDDDDD....DDDDDDDDD....
//.....AAAAA.....DDDDDDDDDD...DDDDDDDDDD...
//....AAAAAA.....DDDDDDDDDDD..DDDDDDDDDDD..
//....AAAAAAA....DDDD...DDDD..DDDD...DDDD..
//...AAAAAAAA....DDDD....DDDD.DDDD....DDD..
//...AAAAAAAA....DDDD....DDDD.DDDD....DDD..
//...AAAA.AAAA...DDDD....DDDD.DDDD....DDD..
//..AAAAAAAAAA...DDDD....DDDD.DDDD....DDD..
//..AAAAAAAAAAA..DDDD....DDDD.DDDD....DDD..
//..AAAAAAAAAAA..DDDD...DDDDD.DDDD...DDDD..
//.AAAA....AAAA..DDDDDDDDDDD..DDDDDDDDDDD..
//.AAAA.....AAAA.DDDDDDDDDD...DDDDDDDDDD...
//.AAAA.....AAAA.DDDDDDDDD....DDDDDDDDD....
//.........................................

pub async fn add_product(
    State(pool): State<Pool<Sqlite>>,
    mut mutipart_form: Multipart,
) -> Result<Markup, AppError> {
    let mut add_product_form = AddProductForm {
        title: String::new(),
        description: String::new(),
        category: String::new(),
        brand: String::new(),
        price: String::new(),
        sale_price: String::new(),
        total_stock: String::new(),
        image_data: Bytes::new(),
        image_name: String::new(),
        image_content_type: String::new(),
    };

    while let Some(field) = mutipart_form.next_field().await.map_err(|error| {
        tracing::error!("Failed to get multipart field: {}", error);
        AppError::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error".to_string(),
        )
    })? {
        let name = field.name().unwrap_or("").to_string();

        if name == "title" {
            add_product_form.title = field.text().await.map_err(|error| {
                tracing::error!("Failed to get title text: {}", error);
                AppError::new(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error".to_string(),
                )
            })?;
        } else if name == "description" {
            add_product_form.description = field.text().await.map_err(|error| {
                tracing::error!("Failed to get description text: {}", error);
                AppError::new(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error".to_string(),
                )
            })?;
        } else if name == "category" {
            add_product_form.category = field.text().await.map_err(|error| {
                tracing::error!("Failed to get category text: {}", error);
                AppError::new(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error".to_string(),
                )
            })?;
        } else if name == "brand" {
            add_product_form.brand = field.text().await.map_err(|error| {
                tracing::error!("Failed to get brand text: {}", error);
                AppError::new(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error".to_string(),
                )
            })?;
        } else if name == "price" {
            add_product_form.price = field.text().await.map_err(|error| {
                tracing::error!("Failed to get price text: {}", error);
                AppError::new(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error".to_string(),
                )
            })?;
        } else if name == "sale_price" {
            add_product_form.sale_price = field.text().await.map_err(|error| {
                tracing::error!("Failed to get sale_price text: {}", error);
                AppError::new(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error".to_string(),
                )
            })?;
        } else if name == "total_stock" {
            add_product_form.total_stock = field.text().await.map_err(|error| {
                tracing::error!("Failed to get total_stock text: {}", error);
                AppError::new(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error".to_string(),
                )
            })?;
        } else if name == "image" {
            if let Some(file_name) = field.file_name() {
                add_product_form.image_name = file_name.to_string();
                add_product_form.image_content_type =
                    if let Some(content_type) = field.content_type() {
                        content_type.to_string()
                    } else {
                        "application/octet-stream".to_string()
                    };
                add_product_form.image_data = field.bytes().await.map_err(|error| {
                    tracing::error!("Failed to get image bytes: {}", error);
                    AppError::new(
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Server Error".to_string(),
                    )
                })?
            }
        }
    }

    let new_product = sqlx::query_as!(
        Product,
        r#"
        INSERT INTO products (title, description, category, brand, price, sale_price, total_stock)
        VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *
        "#,
        add_product_form.title,
        add_product_form.description,
        add_product_form.category,
        add_product_form.brand,
        add_product_form.price,
        add_product_form.sale_price,
        add_product_form.total_stock,
    )
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to insert product: {}", err);
        AppError::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error".to_string(),
        )
    })?;

    let image_blob = add_product_form.image_data.as_ref();

    sqlx::query!(
        r#"
        INSERT INTO images (product_id, name, content_type, image)
        VALUES ($1, $2, $3, $4)
        "#,
        new_product.id,
        add_product_form.image_name,
        add_product_form.image_content_type,
        image_blob,
    )
    .execute(&pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to insert product image: {}", err);
        AppError::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error".to_string(),
        )
    })?;

    Ok(product_tile(new_product))
}

//....................................................
//.....GGGGGGG....EEEEEEEEEEE.TTTTTTTTTTT.SSSSSSS.....
//...GGGGGGGGGG...EEEEEEEEEEE.TTTTTTTTTTTSSSSSSSSS....
//..GGGGGGGGGGGG..EEEEEEEEEEE.TTTTTTTTTTTSSSSSSSSSS...
//..GGGGG..GGGGG..EEEE...........TTTT...TSSSS..SSSS...
//.GGGGG....GGG...EEEE...........TTTT...TSSSS.........
//.GGGG...........EEEEEEEEEE.....TTTT....SSSSSSS......
//.GGGG..GGGGGGGG.EEEEEEEEEE.....TTTT.....SSSSSSSSS...
//.GGGG..GGGGGGGG.EEEEEEEEEE.....TTTT.......SSSSSSS...
//.GGGGG.GGGGGGGG.EEEE...........TTTT..........SSSSS..
//..GGGGG....GGGG.EEEE...........TTTT...TSSS....SSSS..
//..GGGGGGGGGGGG..EEEEEEEEEEE....TTTT...TSSSSSSSSSSS..
//...GGGGGGGGGG...EEEEEEEEEEE....TTTT....SSSSSSSSSS...
//.....GGGGGGG....EEEEEEEEEEE....TTTT.....SSSSSSSS....
//....................................................

pub async fn get_all_products(pool: &Pool<Sqlite>) -> Result<Vec<Product>, AppError> {
    let products = sqlx::query_as!(
        Product,
        r#"
        SELECT * FROM products
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to get all products: {}", err);
        AppError::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error".to_string(),
        )
    })?;

    Ok(products)
}

pub async fn get_image_by_product_id(
    State(pool): State<Pool<Sqlite>>,
    Path(product_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let image = sqlx::query_as!(
        Image,
        r#"
        SELECT * FROM images WHERE product_id = $1
        "#,
        product_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to get image by product id: {}", err);
        AppError::new(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error".to_string(),
        )
    })?;

    let mut headers = HeaderMap::new();

    headers.insert(
        "Content-Type",
        image.content_type.parse().map_err(|error| {
            tracing::error!("Failed to parse content type: {}", error);
            AppError::new(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error".to_string(),
            )
        })?,
    );

    headers.insert(
        "Cache-Control",
        "max-age=86400".parse().map_err(|error| {
            tracing::error!("Failed to parse cache control: {}", error);
            AppError::new(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error".to_string(),
            )
        })?,
    );

    Ok((headers, image.image))
}
