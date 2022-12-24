CREATE TABLE public.telemetry_no_orm (
        id serial4 NOT NULL,
        request_id int4 NOT NULL,
        code varchar NOT NULL,
        property varchar NOT NULL,
        value varchar NULL,
        created_at timestamp NOT NULL,
        CONSTRAINT telemetry_pkey PRIMARY KEY (id)
    );