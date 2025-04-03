-- Add migration script here
-- Apply NOT NULL constraints
ALTER TABLE course_meeting_times
    ALTER COLUMN offering_id SET NOT NULL,
    ALTER COLUMN day_of_week SET NOT NULL;