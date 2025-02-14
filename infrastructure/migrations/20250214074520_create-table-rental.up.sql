CREATE TABLE IF NOT EXISTS rental
(
    rental_id UUID NOT NULL,

    PRIMARY KEY (rental_id),
    FOREIGN KEY (rental_id)
        REFERENCES offering (offering_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE OR REPLACE VIEW rental_offering AS
SELECT *
FROM rental
         JOIN offering ON rental_id = offering_id;