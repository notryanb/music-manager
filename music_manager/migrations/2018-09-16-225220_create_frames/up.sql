create table frame_types (
    id serial primary key,
    code text not null,
    description text not null
);

create table frames (
  id serial primary key,
  id3_tag_id integer not null references id3_tags(id),
  frame_type_id integer not null references frame_types(id),
  content text not null
);

insert into frame_types (code, description)
values ('TYER', 'Year'),
  ('TPE1', 'Artist'),
  ('TPE2', 'Album Artist'),
  ('TALB', 'Album'),
  ('TPE1', 'Artist'),
  ('TIT2', 'Title'),
  ('TLEN', 'Duration'),
  ('TCON', 'Genre'),
  ('TPOS', 'Disc'),
  ('TRCK', 'Track');
