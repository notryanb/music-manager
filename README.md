Music Manager
---
Notes on designs for the database.


## CLI TODO:
- Import files (for given directory, add all `.mp3` files recursively)
- Clean database (go through each File and check if Path exists)
- Get full Id3 tag of file (file path as arg)
- Get full Id3 tags of all `.mp3` files for a file path (given as arg)
- List all available Id3 frames (stick to Id32.4, as that's what `rust-id3` automatically converts everything to)
- Write frame(s) for given mp3 file (takes file, frame, frame content as args)
- Write frames for given all `.mp3` files in given path (file path, frames, frame content) 
- Search for Tracks by given frame & content (Postgres Full Text Search?)
- Various statistics (how many tracks in music library, most tracks by artist, longest / shortest tracks?...)

## Notes:
Think about database denormalization (Do I really need to store all these lookup tables?).

## PostgreSQL table design
### files
- id [int]: PK
- path [text]: File path to .mp3 file

### id3_tags
- id [int]: PK
- version_id [byte or something similar]: FK to `versions` table
- file_id [int]: diesFK to file that this tag is associated with

### frames
- id [int]: PK
- id3_tag_id [int]: FK to id3_tags table
- frame_type_id [int]: FK to frame type lookup table
- content [text]: The value that is stored for the frame

### frame_types
- id [int]: PK
- code [text]: The 4 letter code identifing the frame via the ID3 standards.
- description: Description of the frame.
