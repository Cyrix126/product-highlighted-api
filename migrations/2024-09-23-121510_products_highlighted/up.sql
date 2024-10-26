CREATE TABLE products_highlighted (
  id SERIAL PRIMARY KEY,
  product_id SERIAL NOT NULL UNIQUE,
  priority SMALLSERIAL NOT NULL UNIQUE,
  enabled BOOLEAN NOT NULL DEFAULT 't'
);