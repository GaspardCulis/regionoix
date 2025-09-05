CREATE TABLE IF NOT EXISTS BRAND(
    id serial primary key,
    name varchar(100) not null unique,
    description text
);

CREATE TABLE IF NOT EXISTS REGION (
    id serial primary key,
    name varchar(100) unique not null,
    description text
);

CREATE TABLE IF NOT EXISTS CATEGORY (
    id serial primary key,
    name varchar(100) not null unique,
    description text
);


CREATE TABLE IF NOT EXISTS SUB_CATEGORY(
    id serial primary key,
    main_category_id integer references category(id) ON DELETE CASCADE,
    sub_category_id integer references category(id) ON DELETE CASCADE,
    unique(main_category_id, sub_category_id)
);

CREATE TABLE IF NOT EXISTS USER_(
    id serial primary key,
    email citext unique not null,
    password varchar not null
);

CREATE TABLE IF NOT EXISTS PRODUCT (
    id serial primary key,
    name varchar(50) not null unique,
    description text,
    weight float,
    price numeric(10,2) CHECK(price > 0),
    brand_id integer references brand(id),
    image varchar,
    stock integer not null CHECK(stock >= 0) default 0,
    region_id integer references region(id)
);

CREATE TABLE IF NOT EXISTS PRODUCT_CATEGORY(
    id serial primary key,
    product_id integer references product(id) ON DELETE CASCADE,
    category_id integer references category(id) ON DELETE CASCADE,
    unique (category_id, product_id)
);


CREATE TABLE IF NOT EXISTS ORDER_(
    id serial primary key,
    client_lastname varchar not null,
    client_firstname varchar not null,
    total_price numeric(10,2) not null,
    status_ varchar CHECK (status_ in ('PAYED', 'IN_DELIVERY', 'DELIVERED','CANCELED' )),
    arrival_date date,
    creation_date date,
    user_id integer references user_(id) ON DELETE CASCADE,
    city varchar not null,
    country varchar not null,
    adress varchar not null,
    postal_code numeric not null
);


CREATE TABLE IF NOT EXISTS ORDER_LINE(
    id serial primary key,
    quantity integer not null CHECK(quantity >= 0),
    unit_price numeric(10,2) not null CHECK(unit_price >= 0),
    product_id integer references product(id),
    order_id integer references order_(id) ON DELETE CASCADE)
;

CREATE TABLE IF NOT EXISTS CART(
    id serial primary key,
    user_id integer references user_(id) UNIQUE
);

CREATE TABLE IF NOT EXISTS CART_LINE (
    id serial primary key,
    cart_id integer references cart(id),
    quantity integer not null CHECK(quantity >= 0),
    product_id integer not null references product(id)
);

CREATE TABLE IF NOT EXISTS DISCOUNT(
    id serial primary key,
    percentage_off integer NOT NULL check(percentage_off > 0),
    start_date date not null,
    end_date date not null,
    check (end_date > start_date),
    product_id integer references product(id)
);

