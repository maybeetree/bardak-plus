-- Add migration script here

create table thumbs (
	original text not null,
	spec text not null,
	thumb text,  -- null if not ready
	ready boolean not null,
	primary key (original, spec)
);

