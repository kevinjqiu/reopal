Implement a maintenance mode for the CLI:

The maintenance mode accepts a parameter that indicates the disk space quota.
For all the videos in the database that are not deleted, if the sum of their size is over the quota, delete the old video files until the total size of the reolink folder is below the quota.  Then mark the video file as deleted in the database.

Provide a flag dry-run such that it only prints out the files to be deleted and the database entries to be marked as deleted but doesn't actually delete the files or updates the database.
