-- Your SQL goes here

CREATE TABLE public.api_keys
(
    api_key uuid NOT NULL,
    PRIMARY KEY (api_key)
);

ALTER TABLE public.api_keys
    OWNER to admin;

-- Get Requests Table

CREATE TABLE public.get_requests
(
    api_key uuid NOT NULL,
    route text NOT NULL,
    response jsonb NOT NULL,
    PRIMARY KEY (api_key, route),
    CONSTRAINT api_key FOREIGN KEY (api_key)
        REFERENCES public.api_keys (api_key) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
);

ALTER TABLE public.get_requests
    OWNER to admin;