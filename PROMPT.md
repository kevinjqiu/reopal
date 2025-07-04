Create a CLI command such that it can read the ReoLink video directory.

The ReoLink video directory has the following format:
- The folders are in the format of MMDDYYYY
- inside each folder, there are multiple files. They're in the format of <camera name>-00-<start time>-<end time>.mp4.
- <start time> and <end time> are in the format of HHMMSS
- both times are in the local timezone, subject to time saving

Store each file in a sqlite database with each part extracted:
- Camera name
- Date
- Start time
- End time
- File path
- File size
- Deleted (boolean)

