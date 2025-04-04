-- Add migration script here
ALTER TABLE student_profiles
    ALTER COLUMN major SET NOT NULL;
