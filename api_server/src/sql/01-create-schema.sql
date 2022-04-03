CREATE TABLE IF NOT EXISTS Item
(
    id           int PRIMARY KEY,
    name         varchar(32) NOT NULL,
    time_to_cook int         NOT NULL DEFAULT 10
);
CREATE SEQUENCE IF NOT EXISTS item_id_seq INCREMENT 1 START WITH 1000;

CREATE TABLE IF NOT EXISTS Tables
(
    id    int PRIMARY KEY,
    seats int NOT NULL
);

CREATE TABLE IF NOT EXISTS Orders
(
    id           int PRIMARY KEY,
    item_id      int NOT NULL ,
    table_id     int NOT NULL ,
    quantity     int NOT NULL default 1,
    CONSTRAINT fk_item FOREIGN KEY(item_id) REFERENCES Item(id),
    CONSTRAINT fk_table FOREIGN KEY(table_id) REFERENCES Tables(id)
);

CREATE SEQUENCE IF NOT EXISTS order_id_seq INCREMENT 1 START WITH 1000;