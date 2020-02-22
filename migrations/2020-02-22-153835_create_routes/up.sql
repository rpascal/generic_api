-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public."user_endpoints"
(
   key uuid NOT NULL,
   endpoint text NOT NULL,
   response json NOT NULL,
   id serial NOT NULL,
   PRIMARY KEY (id)
);

ALTER TABLE public."user_endpoints"
   OWNER to admin