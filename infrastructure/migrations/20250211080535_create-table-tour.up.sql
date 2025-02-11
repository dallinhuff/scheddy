CREATE TABLE IF NOT EXISTS tour
(
    tour_id     UUID NOT NULL,
    vendor_id   UUID NOT NULL,
    title       TEXT NOT NULL,
    description TEXT,

    PRIMARY KEY (tour_id),
    FOREIGN KEY (vendor_id) REFERENCES vendor (vendor_id) ON DELETE RESTRICT,
    UNIQUE (vendor_id, title)
);

CREATE TABLE IF NOT EXISTS tour_rental
(
    tour_id   UUID NOT NULL,
    rental_id UUID NOT NULL,

    PRIMARY KEY (tour_id, rental_id)
);

CREATE OR REPLACE VIEW tour_with_rentals AS
WITH t_rentals AS
         (SELECT tour_id, json_agg(json_build_object('rental_id', rental_id)) AS rentals
          FROM tour_rental
          GROUP BY tour_id)
SELECT tour_id,
       vendor_id,
       title,
       description,
       coalesce(rentals, '[]'::JSON) AS rentals
FROM tour
         LEFT JOIN t_rentals USING (tour_id)