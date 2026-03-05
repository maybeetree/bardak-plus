-- Add migration script here

create table thumbs (
	original text,
	thumb text,
	primary key (original, thumb)
);

