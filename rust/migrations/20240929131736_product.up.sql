-- Add up migration script here
CREATE TABLE products (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL,
    brand TEXT NOT NULL,
    price INTEGER NOT NULL,
    sale_price INTEGER NOT NULL,
    total_stock INTEGER NOT NULL
);

CREATE TABLE images (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    image BLOB NOT NULL,
    name TEXT NOT NULL,
    content_type TEXT NOT NULL,
    product_id INTEGER NOT NULL,
    FOREIGN KEY (product_id) REFERENCES products(id)
);