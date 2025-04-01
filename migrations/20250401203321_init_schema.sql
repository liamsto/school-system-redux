-- USERS
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT UNIQUE NOT NULL,
    hashed_password TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('student', 'admin')),
    created_at TIMESTAMPTZ DEFAULT now()
);

-- DEPARTMENTS
CREATE TABLE departments (
    id INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    code TEXT UNIQUE NOT NULL, -- e.g. CS, MATH
    name TEXT NOT NULL
);

-- COURSES
CREATE TABLE courses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    department_id INT NOT NULL REFERENCES departments(id),
    course_number TEXT NOT NULL, -- e.g. CS101
    title TEXT NOT NULL,
    description TEXT,
    credits INT NOT NULL CHECK (credits > 0),
    UNIQUE (department_id, course_number)
);

-- COURSE PREREQUISITES
CREATE TABLE course_prerequisites (
    course_id UUID REFERENCES courses(id) ON DELETE CASCADE,
    prerequisite_id UUID REFERENCES courses(id) ON DELETE CASCADE,
    PRIMARY KEY (course_id, prerequisite_id),
    CHECK (course_id <> prerequisite_id)
);

-- TERMS (e.g. Fall 2025)
CREATE TABLE terms (
    id INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL, -- e.g. Fall 2025
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    CHECK (start_date < end_date)
);

-- COURSE OFFERINGS (specific to a term)
CREATE TABLE course_offerings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id UUID REFERENCES courses(id) ON DELETE CASCADE,
    term_id INT REFERENCES terms(id),
    instructor_id UUID REFERENCES users(id),
    capacity INT NOT NULL CHECK (capacity > 0),
    location TEXT,
    UNIQUE (course_id, term_id, instructor_id)
);

-- COURSE SCHEDULE (recurrence info)
CREATE TABLE course_meeting_times (
    id INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    offering_id UUID REFERENCES course_offerings(id) ON DELETE CASCADE,
    day_of_week TEXT CHECK (day_of_week IN ('Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday')),
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    CHECK (start_time < end_time)
);

-- STUDENT PROFILE
CREATE TABLE student_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id),
    student_id TEXT UNIQUE NOT NULL, -- university-assigned
    enrollment_year INT NOT NULL,
    major TEXT
);

-- COURSE REGISTRATION
CREATE TABLE registrations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID REFERENCES users(id) ON DELETE CASCADE,
    offering_id UUID REFERENCES course_offerings(id) ON DELETE CASCADE,
    registered_at TIMESTAMPTZ DEFAULT now(),
    status TEXT NOT NULL CHECK (status IN ('registered', 'dropped', 'waitlisted')),
    grade TEXT, -- can be null until assigned
    UNIQUE (student_id, offering_id)
);