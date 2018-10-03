Music Manager
---
Notes on designs for the database.

PostgreSQL table design

## files
- id [int]: PK
- path [text]: File path to .mp3 file

## id3_tags
- id [int]: PK
- version_id [byte or something similar]: FK to `versions` table
- file_id [int]: diesFK to file that this tag is associated with

## frames
- id [int]: PK
- id3_tag_id [int]: FK to id3_tags table
- frame_type_id [int]: FK to frame type lookup table
- content [text]: The value that is stored for the frame

## frame_types
- id [int]: PK
- code [text]: The 4 letter code identifing the frame via the ID3 standards.
- description: Description of the frame.
