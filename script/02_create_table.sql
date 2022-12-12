\c example

CREATE TABLE menu (
    id serial not null primary key
    , menu varchar(16) not null unique
    , price int not null
    , stock int not null
);

CREATE TABLE orders (
    id serial not null primary key
    , menu varchar(256) not null
    , price int not null
    , ordered_at timestamp not null
);

GRANT ALL PRIVILEGES ON menu TO hoge;

INSERT INTO menu (menu, price, stock) VALUES ('カツカレー', 360, 100);
INSERT INTO menu (menu, price, stock) VALUES ('カツ丼', 360, 100);
INSERT INTO menu (menu, price, stock) VALUES ('親子丼', 360, 100);
