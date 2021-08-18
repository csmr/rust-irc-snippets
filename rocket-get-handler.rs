// cargo.toml: uuid = { version = "0.8", features = ["serde"] }
// cargo.toml: serde = { version = "1.0", features = ["derive"] }

#[get("/tasks")]
async fn get_tasks(user: User, pool: &State<PgPool>) -> Result<Json<Vec<Task>>, Status> {
    let pool = pool.inner();
    let tasks = sqlx::query_as!(Task, "select * from tasks where user_id = $1", user.user_id)
        .fetch_all(pool)
        .await
        .map_err(|_| Status::InternalServerError)?; 
    Ok(Json(tasks))
}
