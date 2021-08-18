#[get("/tasks")]
async fn get_tasks(user: User, pool: &State<PgPool>) -> Result<Json<Vec::<Task>>, Status> {
    let pool = pool.inner();

    let tasks = sqlx::query_as!(Task, "select * from tasks where user_id = $1", user.user_id).fetch_all(pool).await.unwrap();

    Err(Status::InternalServerError)
}


// structs


#[derive(sqlx::FromRow, Serialize)]
pub struct Task {
    #[serde(with = "crate::serde_uuid")]
    task_id: Uuid,
    #[serde(with = "crate::serde_uuid")]
    user_id: Uuid,
    #[serde(default)]
    tag_id: Option<MyUuid>,
    task: String,
    created: NaiveDate
}

// to get Option working with serde
#[derive(sqlx::Type)]
#[sqlx(transparent)]
#[sqlx(type_name = "Uuid")]
pub struct MyUuid(Uuid);


impl Serialize for MyUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
            S: Serializer {
        let uuid = self.0.to_hyphenated().to_string();
        serializer.serialize_str(&uuid)
    }
}

impl<'de> Deserialize<'de> for MyUuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
            D: Deserializer<'de> {
        let uuid_string = String::deserialize(deserializer)?;
        let uuid = Uuid::parse_str(&uuid_string).map_err(D::Error::custom)?;
        Ok(Self(uuid))
    }
}
