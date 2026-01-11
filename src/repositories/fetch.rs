use sqlx::{PgPool, QueryBuilder};
use crate::models::fetch::{Api, ApiData, ApiExecute, ApiHeader, ApiMembers, CreateApi, CreateApiMembers, UpdateMemberRequest, UpdateApi};

pub struct FetchRepository {
    pool: PgPool,
}
pub struct FetchDataRepository {
    pool: PgPool
}
pub struct FetchHeaderRepository {
    pool: PgPool
}
pub struct FetchExecuteRepository {
    pool: PgPool
}
pub struct FetchMemberRepository {
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
            r#"INSERT INTO fetch_api (name, type, job_id, description, execute_id, header_id, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(data.name)
        .bind(data.r#type)
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
            SET name = $1, type = $2, job_id = $3, description = $4, execute_id = $5, header_id = $6, is_active = $7, secure = $8
            WHERE id = $9
            RETURNING *
            "#
        )
        .bind(data.name)
        .bind(data.r#type)
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
}

impl FetchMemberRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {pool}
    }

    pub async fn find_by_id(&self, id: i32) -> Result<ApiMembers, sqlx::Error> {
        sqlx::query_as::<_, ApiMembers> (
            r#"SELECT * FROM fetch_api_members WHERE id = $1"#
        )
        .bind(id)
        .fetch_one(&self.pool)
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

    pub async fn update(&self, member_id: i32, payload: UpdateMemberRequest) -> Result<(), sqlx::Error> {
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
        qb.push_bind(member_id);
        let query = qb.build();
        query.execute(&self.pool).await?;

        Ok(())
    }
}
