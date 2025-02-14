CREATE TABLE IF NOT EXISTS tour
(
    tour_id UUID NOT NULL,

    PRIMARY KEY (tour_id),
    FOREIGN KEY (tour_id)
        REFERENCES offering (offering_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE OR REPLACE VIEW tour_offering AS
SELECT *
FROM tour
         JOIN offering ON tour_id = offering_id;

CREATE TABLE IF NOT EXISTS tour_rental
(
    tour_id   UUID NOT NULL,
    rental_id UUID NOT NULL,

    PRIMARY KEY (tour_id, rental_id),
    FOREIGN KEY (tour_id)
        REFERENCES tour (tour_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (rental_id)
        REFERENCES rental (rental_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);