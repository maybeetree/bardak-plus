drop table if exists item;
drop table if exists attr;
drop table if exists item_attr;

create table item (
	id integer primary key autoincrement
);

create table attr (
	name text,
	val text,
	primary key (name, val)
);

create table item_attr (
	item_id,
	attr_name,
	attr_val,
	foreign key (item_id) references item (id),
	foreign key (attr_name) references attr (name),
	foreign key (attr_val) references attr (val)
);

insert into item (id) values (0);
insert into item (id) values (1);

insert into attr (name, val) values ('desc', 'this is a banana');
insert into attr (name, val) values ('desc', 'this is an apple');
insert into attr (name, val) values ('desc', 'this is tasty banana');
insert into attr (name, val) values ('color', 'red');
insert into attr (name, val) values ('color', 'green');
insert into attr (name, val) values ('color', 'yellow');

insert into item_attr (item_id, attr_name, attr_val) values (
	0, 'desc', 'this is a banana');
insert into item_attr (item_id, attr_name, attr_val) values (
	0, 'desc', 'this is tasty banana');
insert into item_attr (item_id, attr_name, attr_val) values (
	0, 'color', 'yellow');
insert into item_attr (item_id, attr_name, attr_val) values (
	1, 'desc', 'this is an apple');
insert into item_attr (item_id, attr_name, attr_val) values (
	1, 'color', 'green');
insert into item_attr (item_id, attr_name, attr_val) values (
	1, 'color', 'red');

select item.id as id, attr.val as desc
from item
inner join item_attr on item.id = item_attr.item_id
inner join attr on attr.name = item_attr.attr_name
	and attr.val = item_attr.attr_val
where desc like '%banana%'
;


