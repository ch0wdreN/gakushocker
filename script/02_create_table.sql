\c example

CREATE TABLE users (
    id serial PRIMARY KEY,
    display_name varchar NOT NULL,
    email varchar(256) UNIQUE NOT NULL,
    password varchar(256) NOT NULL,
    point int NOT NULL
);

CREATE TABLE products (
    id serial PRIMARY KEY,
    name varchar(256) NOT NULL,
    price int NOT NULL,
    stock int NOT NULL
);

CREATE TABLE orders (
    id uuid PRIMARY KEY,
    user_id int,
    total int NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    status varchar(32) NOT NULL
);

CREATE TABLE order_items (
    id serial PRIMARY KEY,
    order_id uuid NOT NULL,
    product_id int NOT NULL,
    quantity int NOT NULL,
    UNIQUE (order_id, product_id)
);

ALTER TABLE orders ADD FOREIGN KEY (user_id) REFERENCES users (id);

ALTER TABLE order_items ADD FOREIGN KEY (order_id) REFERENCES orders (id);

ALTER TABLE order_items ADD FOREIGN KEY (product_id) REFERENCES products (id);

CREATE TYPE item AS (name VARCHAR(256), price INT, quantity INT);

CREATE OR REPLACE VIEW order_view AS
    SELECT
        o.id,
        o.user_id,
        o.created_at,
        o.total,
        o.status,
        ARRAY_AGG(ROW(p.name, p.price, i.quantity)::item) AS items
    FROM
        orders AS o
            INNER JOIN
        order_items AS i
        ON i.order_id = o.id
            INNER JOIN
        products AS p
        ON p.id = i.product_id
    GROUP BY o.id;

INSERT INTO products (name, price, stock)
    VALUES
        ('カツ丼', 360, 100),
        ('肉うどん', 300, 100),
        ('カレー', 340, 100);

INSERT INTO users
    VALUES
        (1, 'example_user', 'ex@example.com', 'passw0rd', 100);
