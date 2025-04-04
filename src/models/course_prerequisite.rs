use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct CoursePrerequisite {
    pub course_id: Uuid,
    pub prerequisite_id: Uuid,
}

impl CoursePrerequisite {
    pub fn new(course_id: Uuid, prerequisite_id: Uuid) -> CoursePrerequisite {
        CoursePrerequisite {
            course_id,
            prerequisite_id,
        }
    }
}
