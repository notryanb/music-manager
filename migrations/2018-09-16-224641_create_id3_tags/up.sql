create table versions (
    id serial primary key,
    version text not null
);

create table id3_tags (
  id serial primary key,
  version_id integer references versions(id),
  file_id integer references files(id)
);