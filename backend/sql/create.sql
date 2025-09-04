CREATE TABLE IF NOT EXISTS BRAND(
    id serial primary key,
    name varchar(100) not null unique,
    description text
);

CREATE TABLE IF NOT EXISTS STOCK(
    id serial primary key,
    quantity integer not null CHECK(quantity >= 0)
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
    main_category_id integer references category(id) ON DELETE CASCADE,
    sub_category_id integer references category(id) ON DELETE CASCADE,
    primary key(main_category_id, sub_category_id)
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
    weight integer,
    price numeric(10,2) CHECK(price > 0),
    brand_id integer references brand(id),
    image varchar,
    stock_id integer references stock(id),
    region_id integer references region(id)
);

CREATE TABLE IF NOT EXISTS PRODUCT_CATEGORY(
    product_id integer references product(id) ON DELETE CASCADE,
    category_id integer references category(id) ON DELETE CASCADE,
    primary key (category_id, product_id)
);


CREATE TABLE IF NOT EXISTS ORDER_(
    id serial primary key,
    client_lastname varchar not null,
    client_firstname varchar not null,
    total_price numeric(10,2) not null,
    status_ varchar CHECK (status_ in ('basket', 'validated', 'payed','in_delivery','delivered','canceled' )),
    arrival_date date,
    user_id integer references user_(id) ON DELETE CASCADE,
    city varchar not null,
    country varchar not null,
    postal_code numeric not null
);


CREATE TABLE IF NOT EXISTS ORDER_LINE(
    id serial primary key,
    quantity integer not null CHECK(quantity >= 0),
    unit_price numeric(10,2) not null CHECK(unit_price >= 0),
    product_id integer references product(id),
    order_id integer references order_(id) ON DELETE CASCADE)
;

CREATE TABLE IF NOT EXISTS DISCOUNT(
    id serial primary key,
    percentage_off integer NOT NULL check(percentage_off > 0),
    start_date date not null,
    end_date date not null,
    check (end_date > start_date),
    product_id integer references product(id)
);

