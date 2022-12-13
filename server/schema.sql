create table orders (
  id serial not null primary key
  , menu varchar(16) not null
  , price int not null
  , ordered_at timestamp not null
);

create table menu (
  id serial not null primary key
  , menu varchar(16) not null
  , price int not null
  , stock int not null
);
