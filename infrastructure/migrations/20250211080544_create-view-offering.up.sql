CREATE OR REPLACE VIEW offering AS
SELECT tour_id AS offering_id,
       vendor_id,
       title,
       description,
       'tour'  AS kind
FROM tour
UNION
SELECT rental_id AS offering_id,
       vendor_id,
       title,
       description,
       'rental'  AS kind
FROM rental;
