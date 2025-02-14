CREATE TABLE IF NOT EXISTS offering
(
    offering_id UUID NOT NULL,
    vendor_id   UUID NOT NULL,
    title       TEXT NOT NULL,
    description TEXT,

    PRIMARY KEY (offering_id),
    FOREIGN KEY (vendor_id) REFERENCES vendor (vendor_id),
    UNIQUE (vendor_id, title)
);
