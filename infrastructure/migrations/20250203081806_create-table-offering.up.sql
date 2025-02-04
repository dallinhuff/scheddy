create table if not exists offering
(
    id        uuid primary key,
    vendor_id uuid not null references vendor (id),
    name      text not null
);

create table if not exists rental
(
    id uuid primary key references offering (id)
);

create table if not exists tour_style
(
    id   serial primary key,
    name text not null unique
);

insert into tour_style (id, name)
values (1, 'Guided'),
       (2, 'Self-Guided')
on conflict do nothing;

create table if not exists tour
(
    id    uuid primary key references offering (id),
    style int4 not null references tour_style (id)
);

create table if not exists tour_rental
(
    tour_id   uuid not null references tour (id),
    rental_id uuid not null references rental (id)
);
