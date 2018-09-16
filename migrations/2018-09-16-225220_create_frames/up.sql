create table frame_types (
    id serial primary key,
    code text not null,
    description text not null
);

create table frames (
  id serial primary key,
  id3_tag_id integer references id3_tags(id),
  frame_type_id integer references frame_types(id),
  content text not null
);