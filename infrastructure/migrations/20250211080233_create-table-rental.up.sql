CREATE TABLE IF NOT EXISTS rental
(
    rental_id   UUID NOT NULL,
    vendor_id   UUID NOT NULL,
    title       TEXT NOT NULL,
    description TEXT,

    PRIMARY KEY (rental_id),
    FOREIGN KEY (vendor_id) REFERENCES vendor (vendor_id),
    UNIQUE (vendor_id, title)
);
