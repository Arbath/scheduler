use sqlx::{PgPool, QueryBuilder};
use crate::models::fetch::{Api, ApiData, ApiExecute, ApiHeader, ApiMembers, CreateApi, CreateApiData, CreateApiExecute, CreateApiHeader, CreateApiMembers, UpdateApi, UpdateApiData, UpdateApiExecute, UpdateApiHeader, UpdateMemberRequest};
pub struct FetchRepository {
    pool: PgPool,
}
pub struct FetchMemberRepository {
    pool: PgPool
}
pub struct FetchExecuteRepository {
    pool: PgPool
}
pub struct FetchHeaderRepository {
    pool: PgPool
}
pub struct FetchDataRepository {
    pool: PgPool
}

impl FetchRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_job_id(&self ,job_id: &str) -> Result<Api, sqlx::Error> {
        sqlx::query_as::<_,Api>(
            r#"SELECT * FROM fetch_api WHERE job_id = $1"#
        )
        .bind(job_id)
        .fetch_one(&self.pool)
        .await
    }
    
    pub async fn get_by_id(&self ,id: &i32) -> Result<Api, sqlx::Error> {
        sqlx::query_as::<_,Api>(
            r#"SELECT * FROM fetch_api WHERE id = $1"#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_all_fetch(&self) -> Result<Vec<Api>, sqlx::Error> {
        sqlx::query_as::<_,Api>(
            r#"SELECT * FROM fetch_api ORDER BY id ASC"#
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create(&self, data: CreateApi) -> Result<Api, sqlx::Error> {
        sqlx::query_as::<_,Api>(
            r#"INSERT INTO fetch_api (name, type, method, topic, job_id, description, execute_id, header_id, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(data.name)
        .bind(data.r#type)
        .bind(data.method)
        .bind(data.topic)
        .bind(data.job_id)
        .bind(data.description)
        .bind(data.execute_id)
        .bind(data.header_id)
        .bind(data.is_active)
        .fetch_one(&self.pool)
        .await
    }
    
    pub async fn update(&self,id: &i32, data: UpdateApi) -> Result<Api, sqlx::Error> {
        sqlx::query_as::<_,Api>(
            r#"UPDATE fetch_api
            SET name = $1, type = $2, method =$3, topic = $4, job_id = $5, description = $6, execute_id = $7, header_id = $8, is_active = $9, secure = $10
            WHERE id = $11
            RETURNING *
            "#
        )
        .bind(data.name)
        .bind(data.r#type)
        .bind(data.method)
        .bind(data.topic)
        .bind(data.job_id)
        .bind(data.description)
        .bind(data.execute_id)
        .bind(data.header_id)
        .bind(data.is_active)
        .bind(data.secure)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(&self, id: &i32) -> Result<Api, sqlx::Error> {
        sqlx::query_as::<_, Api>(
            r#"
            DELETE FROM fetch_api
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }    
}

impl FetchMemberRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {pool}
    }

    pub async fn find_by_id(&self, fetch_id: i32, user_id: i32) -> Result<ApiMembers, sqlx::Error> {
        sqlx::query_as::<_, ApiMembers> (
            r#"SELECT * FROM fetch_api_members WHERE fetch_id = $1 AND user_id = $2 RETURNING *"#
        )
        .bind(fetch_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_members(&self, fetch_id: i32) -> Result<Vec<ApiMembers>, sqlx::Error> {
        sqlx::query_as::<_, ApiMembers> (
            r#"SELECT * FROM fetch_api_members WHERE fetch_id = $1 RETURNING *"#
        )
        .bind(fetch_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create(&self, data: CreateApiMembers) -> Result<CreateApiMembers, sqlx::Error> {
        sqlx::query_as::<_,CreateApiMembers>(
            r#"INSERT INTO fetch_api_members (fetch_id, user_id, role)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(data.fetch_id)
        .bind(data.user_id)
        .bind(data.role)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update(&self, id: i32, payload: UpdateMemberRequest) -> Result<(), sqlx::Error> {
        if payload.fetch_id.is_none() && payload.user_id.is_none() && payload.role.is_none() {
            return Ok(());
        }

        let mut qb = QueryBuilder::new("UPDATE api_members SET ");
        let mut separated = qb.separated(", ");

        if let Some(fetch_id) = payload.fetch_id {
            separated.push("fetch_id = ");
            separated.push_bind_unseparated(fetch_id);
        }

        if let Some(user_id) = payload.user_id {
            separated.push("user_id = ");
            separated.push_bind_unseparated(user_id);
        }

        if let Some(role) = payload.role {
            separated.push("role = ");
            separated.push_bind_unseparated(role);
        }

        qb.push(" WHERE id = ");
        qb.push_bind(id);
        let query = qb.build();
        query.execute(&self.pool).await?;

        Ok(())
    }
}

impl FetchExecuteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {pool}
    }

    pub async fn find_by_id(&self, id: i32) -> Result<ApiExecute, sqlx::Error> {
        sqlx::query_as::<_, ApiExecute> (
            r#"SELECT * FROM fetch_api_execute WHERE id = $1"#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create(&self, data: CreateApiExecute) -> Result<ApiExecute, sqlx::Error> {
        sqlx::query_as::<_,ApiExecute> (
            r#"INSERT INTO fetch_api_execute (user_id, name, is_repeat, type, value)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(data.user_id)
        .bind(data.name)
        .bind(data.is_repeat)
        .bind(data.r#type)
        .bind(data.value)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update(&self, id: i32, data: UpdateApiExecute) -> Result<ApiExecute, sqlx::Error> {
        sqlx::query_as::<_,ApiExecute>(
            r#" UPDATE fetch_api_execute
            SET user_id=$1, name=$2, is_repeat=$3, type=$4, value=$5
            WHERE id = $6
            RETURNING *
            "#
        )
        .bind(data.user_id)
        .bind(data.name)
        .bind(data.is_repeat)
        .bind(data.r#type)
        .bind(data.value)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(&self, id: i32) -> Result<ApiExecute, sqlx::Error> {
        sqlx::query_as::<_,ApiExecute>(
            r#" DELETE FROM fetch_api_execute WHERE id=$1 RETURNING *"#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }
}

impl FetchHeaderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {pool}
    }

    pub async fn find_by_id(&self, id: i32) -> Result<ApiHeader, sqlx::Error> {
        sqlx::query_as::<_, ApiHeader> (
            r#"SELECT * FROM fetch_api_header WHERE id = $1"#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create(&self, data: CreateApiHeader) -> Result<ApiHeader, sqlx::Error>{
        sqlx::query_as::<_,ApiHeader>(
            r#" INSERT INTO fetch_api_header (user_id, name, headers)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(data.user_id)
        .bind(data.name)
        .bind(data.headers)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update(&self, data: UpdateApiHeader) -> Result<ApiHeader, sqlx::Error>{
        sqlx::query_as::<_,ApiHeader>(
            r#"UPDATE fetch_api_header 
            SET user_id = $1, name=$2, headers=$3
            RETURNING *
            "#
        )
        .bind(data.user_id)
        .bind(data.name)
        .bind(data.headers)
        .fetch_one(&self.pool)
        .await
    }
    
    pub async fn delete(&self, id:i32) -> Result<ApiHeader, sqlx::Error> {
        sqlx::query_as::<_,ApiHeader> (
            r#"DELETE FROM fetch_api_header WHERE id=$1 RETURNING *"#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }
}

impl FetchDataRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {pool}
    }

    pub async fn find_by_id(&self, id: i32) -> Result<ApiData, sqlx::Error> {
        sqlx::query_as::<_, ApiData> (
            r#"SELECT * FROM fetch_api_data WHERE id = $1"#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create(&self, data: CreateApiData) -> Result<ApiData, sqlx::Error>{
        sqlx::query_as::<_,ApiData> (
            r#"INSERT INTO fetch_api_data (fetch_id, user_id, name, payload, status_code, response, response_header)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(data.fetch_id)
        .bind(data.user_id)
        .bind(data.name)
        .bind(data.payload)
        .bind(data.status_code)
        .bind(data.response)
        .bind(data.response_headers)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update(&self, id: i32, data: UpdateApiData) -> Result<ApiData, sqlx::Error>{
        sqlx::query_as::<_,ApiData> (
            r#"UPDATE fetch_api_data
            SET fetch_id=$1, user_id=$2, name=$3, payload=$4, status_code=$5, response=$6, response_headers=$7
            WHERE id=$8
            "#
        )
        .bind(data.fetch_id)
        .bind(data.user_id)
        .bind(data.name)
        .bind(data.payload)
        .bind(data.status_code)
        .bind(data.response)
        .bind(data.response_headers)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(&self, id:i32) -> Result<ApiData, sqlx::Error> {
        sqlx::query_as::<_,ApiData> (
            r#"DELETE FROM fetch_api_data WHERE id=$1 RETURNING *"#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }
}
