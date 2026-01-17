use axum::{extract::{FromRef, FromRequestParts}, http::request::Parts};
use crate::{models::{fetch::{Api, ApiMembers, CreateApi, CreateApiMembers, Role, UpdateApi, UpdateApiMembers}, user::User}, repository::fetch::{FetchMemberRepository, FetchRepository}, state::AppState, utils::response::AppError};

#[allow(dead_code)]
pub struct FetchService {
    fetch_repo: FetchRepository,
    member_repo: FetchMemberRepository,
    state: AppState,
}

impl FetchService {
    pub fn new(state: AppState) -> Self {
        let fetch_repo = FetchRepository::new(state.database.clone());
        let member_repo = FetchMemberRepository::new(state.database.clone());
        Self {fetch_repo, member_repo, state}
    }

    /// API AREA
    pub async fn get_all_fetch(&self) -> Result<Vec<Api>, AppError> {
        let query = self.fetch_repo.get_all_fetch()
            .await?;

        Ok(query)
    }

    pub async fn get_fetch_by_id(&self, id: &i32) -> Result<Api, AppError> {
        let query = self.fetch_repo.get_by_id(id)
            .await
            .map_err(|e| {AppError::NotFound(format!("Database: {}", e))})?;

        Ok(query)
    }

    pub async fn get_fetch_by_job(&self, job_id: &str) -> Result<Api, AppError> {
        let query = self.fetch_repo.find_by_job_id(job_id)
            .await
            .map_err(|e| {AppError::NotFound(format!("Database: {}", e))})?;

        Ok(query)
    }

    pub async fn get(&self, id: i32) -> Result<Api, AppError> {
        let query = self.fetch_repo.get_by_id(&id)
            .await
            .map_err(|e| {AppError::NotFound(format!("Database: {}", e))})?;

        Ok(query)
    }
    
    pub async fn create_fetch(&self, data: CreateApi, user: User) -> Result<Api, AppError> {
        let fetch = self.fetch_repo.create(data)
            .await
            .map_err(|e|{
                if let Some(db_error) = e.as_database_error() {
                    if db_error.is_foreign_key_violation() {
                        match db_error.constraint() {
                            Some("fk_fetch_execute") => {
                                return AppError::BadRequest("Execute ID not found. Please create execute first.".to_string());
                            },
                            Some("fk_fetch_header") => {
                                return AppError::BadRequest("Header ID not found. Please create header first.".to_string());
                            },
                            _ => {
                                return AppError::BadRequest("Reference not found.".to_string());
                            }
                        }
                    }
                }
                AppError::BadRequest(format!("Database error: {}", e))
            })?;

        let add_member= CreateApiMembers { user_id: (user.id), role: Role::Owner };
        let _ = self.member_repo.create(fetch.id,add_member)
            .await?;

        Ok(fetch)
    }

    pub async fn update_fetch(&self,id: &i32, data: UpdateApi, _: User) -> Result<Api, AppError> {
        // BUAT PENGECEKAN MEMBER USER
        let query = self.fetch_repo.update(id, data)
            .await
            .map_err(|e| {
                if let Some(db_error) = e.as_database_error() {
                    if db_error.is_foreign_key_violation() {
                        match db_error.constraint() {
                            Some("fk_fetch_execute") => {
                                return AppError::BadRequest("Execute ID not found. Please create execute first.".to_string());
                            },
                            Some("fk_fetch_header") => {
                                return AppError::BadRequest("Header ID not found. Please create header first.".to_string());
                            },
                            _ => {
                                return AppError::BadRequest("Reference not found.".to_string());
                            }
                        }
                    }
                }
                AppError::BadRequest(format!("Database: {}", e))
            })?;

        Ok(query)
    }
    
    pub async fn delete_fetch(&self,id: &i32, user: User) -> Result<Api, AppError> {
        let member = self.member_repo.find_member_id(*id, user.id).await?;
        if member.role != Some(Role::Owner) {
            return Err(AppError::Forbidden("Only owner allowed to delete fetch api.".to_string()));
        }
        let query = self.fetch_repo.delete(id)
            .await?;

        Ok(query)
    }

    /// # MEMBER AREA
    
    /// Find member by id
    pub async fn find_member(&self, id: i32) -> Result<ApiMembers, AppError>{
        let q = self.member_repo.find_by_id(id)
            .await
            .map_err(|e| {AppError::NotFound(format!("Database : {}", e))})?;
        
        Ok(q)
    }

    /// Find all related fetch members
    pub async fn find_members(&self, fetch_id: i32) -> Result<Vec<ApiMembers>, AppError> {
        let q = self.member_repo.find_members(fetch_id)
            .await
            .map_err(|e| {AppError::NotFound(format!("Database : {}", e))})?;
        Ok(q)
    }

    // Todo : Verifikasi user 
    // Add member of fetch id (Role:Owner required)
    pub async fn add_member(&self,_: User, fetch_id: i32, data: CreateApiMembers) -> Result<ApiMembers, AppError> {
        let q = self.member_repo.create(fetch_id, data)
            .await?;

        Ok(q)
    }

    // Edit member Role:Owner or Editor
    pub async fn update_member(&self,_: User,id: i32, data:UpdateApiMembers) -> Result<ApiMembers, AppError> {
        let q = self.member_repo.update(id, data)
            .await?;

        Ok(q)
    }

    // Delete member Role:Owner or Editor
    pub async fn delete_member(&self,_: User, id: i32) -> Result<ApiMembers, AppError> {
        let q = self.member_repo.delete(id)
            .await
            .map_err(|e| {AppError::NotFound(format!("Database: {}", e))})?;

        Ok(q)
    }
}


impl<S> FromRequestParts<S> for FetchService
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        
        Ok(FetchService::new(state))
    }
}