#[cfg(test)]
use sqlx::PgPool;
#[sqlx::test(migrations = "./migrations_test")]
async fn test_insert_and_delete_course(pool: PgPool) -> Result<(), sqlx::Error> {
    use crate::models::course::create_course;
    use uuid::Uuid;
    dotenvy::from_path("test.env").expect("Failed to load test.env");
    let course = create_course(
        Uuid::new_v4(),
        1, 
        "COSC311".to_string(), 
        "Software Testing".to_string(),
        Some("Introduction to software testing methodologies.".to_string()),
        3,
    );

    course.insert(&pool).await?;
    
    let inserted = sqlx::query!(
        "SELECT * FROM courses WHERE id = $1",
        course.id
    )
    .fetch_optional(&pool)
    .await?;
    assert!(inserted.is_some());

    course.delete(&pool).await?;

    let after_delete = sqlx::query!(
        "SELECT * FROM courses WHERE id = $1",
        course.id
    )
    .fetch_optional(&pool)
    .await?;
    assert!(after_delete.is_none());

    Ok(())
}

#[sqlx::test(migrations = "./migrations_test")]
async fn test_add_and_get_prerequisite(pool: PgPool) -> Result<(), sqlx::Error> {
    dotenvy::from_path("test.env").expect("Failed to load test.env");
    use crate::models::course::create_course;
    use uuid::Uuid;
    use crate::models::course::create_prerequisite;

    let main_course = create_course(
        Uuid::new_v4(),
        1,
        "CS220".to_string(), 
        "Advanced Programming".to_string(),
        None,
        4,
    );

    let prereq_course = create_course(
        Uuid::new_v4(),
        1,
        "CS110".to_string(), 
        "Foundations of CS".to_string(),
        None,
        3,
    );

    main_course.insert(&pool).await?;
    prereq_course.insert(&pool).await?;

    let prereq = create_prerequisite(main_course.id, prereq_course.id);
    main_course.add_prerequisite(&pool, &prereq).await?;

    let prerequisites = main_course.get_prerequisites(&pool).await?;
    assert_eq!(prerequisites.len(), 1);
    assert_eq!(prerequisites[0].id, prereq_course.id);

    Ok(())
}

#[sqlx::test(migrations = "./migrations_test")]
async fn test_remove_prerequisite(pool: PgPool) -> Result<(), sqlx::Error> {
    use crate::models::course::create_course;
    use uuid::Uuid;
    use crate::models::course::create_prerequisite;
    dotenvy::from_path("test.env").expect("Failed to load test.env");

    let course = create_course(
        Uuid::new_v4(),
        1,
        "CS301".to_string(),
        "Algorithms".to_string(),
        None,
        4,
    );

    let prereq = create_course(
        Uuid::new_v4(),
        1,
        "CS201".to_string(),
        "Data Structures".to_string(),
        None,
        4,
    );

    course.insert(&pool).await?;
    prereq.insert(&pool).await?;

    let link = create_prerequisite(course.id, prereq.id);
    course.add_prerequisite(&pool, &link).await?;

    let before_removal = course.get_prerequisites(&pool).await?;
    assert_eq!(before_removal.len(), 1);

    
    course.remove_prerequisite(&pool, &link).await?;

    let after_removal = course.get_prerequisites(&pool).await?;
    assert_eq!(after_removal.len(), 0);

    Ok(())
}
