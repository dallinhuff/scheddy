create table if not exists waiver
(
    id      uuid primary key,
    content text not null
);

create table if not exists participant
(
    id   uuid primary key,
    name text not null,
    dob  date not null
);

create table if not exists participant_waiver
(
    id             uuid primary key,
    waiver_id      uuid not null references waiver (id),
    participant_id uuid not null references participant (id),
    date_signed    date not null
);

create table if not exists customer
(
    id           uuid primary key,
    name         text not null,
    email        text not null,
    phone_number text not null
);

create table if not exists customer_preferences
(
    id             uuid primary key references customer (id),
    contact_method text not null
);

create table if not exists employee
(
    id           uuid primary key,
    name         text not null,
    email        text not null,
    phone_number text not null
);

create table if not exists employee_role
(
    id          uuid primary key,
    name        text not null,
    description text not null
);

create table if not exists employee_assignable_role
(
    employee_id uuid not null references employee (id),
    role_id     uuid not null references employee_role (id),

    primary key (employee_id, role_id)
);

create table if not exists trip_kind
(
    id          uuid primary key,
    name        text not null,
    description text not null
);

create table if not exists location
(
    id          uuid primary key,
    name        text not null,
    description text not null
);

create table if not exists trip
(
    id           uuid primary key,
    trip_kind_id uuid        not null references trip_kind (id),
    location_id  uuid        not null references location (id),
    max_capacity int4        not null,
    start_time   timestamptz not null
);

create table if not exists booking
(
    id          uuid primary key,
    customer_id uuid not null references customer (id),
    trip_id     uuid not null references trip (id)
);

create table if not exists booking_participant
(
    booking_id     uuid not null references booking (id),
    participant_id uuid not null references participant (id),

    primary key (booking_id, participant_id)
);

create table if not exists equipment
(
    id          uuid primary key,
    name        text not null,
    description text not null,
    inventory   int4 not null
);

create table if not exists rental (
    booking_id uuid not null references booking(id),
    equipment_id uuid not null references equipment(id),
    quantity int4 not null,

    primary key (booking_id, equipment_id)
);
