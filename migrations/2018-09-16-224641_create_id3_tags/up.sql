create table versions (
    id serial primary key,
    version text not null
);

insert into versions (version)
values
  ('Id3v2.2'), 
  ('Id3v2.3'),
  ('Id3v2.4');

create table id3_tags (
  id serial primary key,
  version_id integer not null references versions(id),
  file_id integer not null references files(id)
);