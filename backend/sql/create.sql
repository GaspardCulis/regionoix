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
    description text,
    category_parent integer references category(id)
);

CREATE TABLE IF NOT EXISTS TAG(
    id serial primary key,
    name citext not null unique
);

CREATE TYPE roles AS ENUM ('CLIENT','ADMIN');

CREATE TABLE IF NOT EXISTS USER_(
    id serial primary key,
    lastname varchar,
    firstname varchar,
    email citext unique not null,
    password varchar not null,
    role roles not null default 'CLIENT'
);

CREATE TABLE IF NOT EXISTS PRODUCT (
    id serial primary key,
    name varchar(50) not null unique,
    description text,
    weight float,
    price real CHECK(price > 0) NOT NULL,
    image varchar,
    stock integer not null CHECK(stock >= 0) default 0,
    region_id integer references region(id) ON DELETE SET NULL,
    brand_id integer references brand(id) ON DELETE SET NULL,
    category_id integer references category(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS PRODUCT_TAG(
    id serial primary key,
    product_id integer references product(id),
    tag_id integer references tag(id)
);

CREATE TABLE IF NOT EXISTS ADDRESS(
    id serial primary key,
    lastname varchar not null,
    firstname varchar not null,
    city varchar not null,
    country varchar not null,
    street varchar not null,
    postal_code varchar not null
);

CREATE TYPE order_status AS ENUM ('PENDING_PAYMENT','PAYED', 'IN_DELIVERY', 'DELIVERED','CANCELED', 'ABORTED');

CREATE TABLE IF NOT EXISTS ORDER_(
    id serial primary key,
    total_price real not null,
    status_ order_status not null default 'PENDING_PAYMENT',
    arrival_date date CHECK (arrival_date > creation_date),
    creation_date date default now(),
    user_id integer references user_(id) ON DELETE SET NULL,
    adress_id integer references adress(id)
);


CREATE TABLE IF NOT EXISTS ORDER_LINE(
    id serial primary key,
    quantity integer not null CHECK(quantity >= 0) default 1,
    unit_price real not null CHECK(unit_price >= 0),
    product_id integer references product(id) ON DELETE CASCADE,
    order_id integer references order_(id) ON DELETE CASCADE,
    UNIQUE(product_id, order_id)
);

CREATE TABLE IF NOT EXISTS CART(
    id serial primary key,
    user_id integer UNIQUE references user_(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS CART_LINE (
    id serial primary key,
    cart_id integer references cart(id) ON DELETE CASCADE,
    quantity integer not null CHECK(quantity >= 0) DEFAULT 1,
    product_id integer not null references product(id) ON DELETE CASCADE,
    UNIQUE(cart_id, product_id)
);

CREATE TABLE IF NOT EXISTS DISCOUNT(
    id serial primary key,
    percentage_off integer NOT NULL check(percentage_off > 0),
    start_date date not null,
    end_date date not null,
    check (end_date > start_date),
    product_id integer references product(id) ON DELETE CASCADE
);

