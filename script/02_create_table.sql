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
    id serial PRIMARY KEY,
    user_id int,
    total int NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    status varchar(32) NOT NULL
);

CREATE TABLE order_items (
    id serial PRIMARY KEY,
    order_id int NOT NULL,
    items int NOT NULL,
    quantity int NOT NULL
);

ALTER TABLE orders ADD FOREIGN KEY (user_id) REFERENCES users (id);

ALTER TABLE order_items ADD FOREIGN KEY (order_id) REFERENCES orders (id);

ALTER TABLE order_items ADD FOREIGN KEY (items) REFERENCES products (id);

INSERT INTO products (name, price, stock)
    VALUES
        ('カツ丼', 360, 100),
        ('肉うどん', 300, 100),
        ('カレー', 340, 100);

INSERT INTO users
    VALUES
        (1, 'example_user', 'ex@example.com', 'passw0rd', 100);
