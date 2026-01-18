use axum::{extract::{FromRef, FromRequestParts}, http::request::Parts};
use crate::{models::{fetch::{Api, ApiExecute, ApiMembers, CreateApi, CreateApiExecute, CreateApiMembers, ReqCreateApiExecute, Role, UpdateApi, UpdateApiExecute, UpdateApiMembers}, user::User}, repository::fetch::{FetchDataRepository, FetchExecuteRepository, FetchHeaderRepository, FetchMemberRepository, FetchRepository}, state::AppState, utils::response::AppError};

#[allow(dead_code)]
pub struct FetchService {
    fetch_repo: FetchRepository,
    member_repo: FetchMemberRepository,
    execute_repo: FetchExecuteRepository,
    header_repo: FetchHeaderRepository,
    data_repo: FetchDataRepository,
    state: AppState,
}

impl FetchService {
    pub fn new(state: AppState) -> Self {
        let fetch_repo = FetchRepository::new(state.database.clone());
        let member_repo = FetchMemberRepository::new(state.database.clone());
        let execute_repo = FetchExecuteRepository::new(state.database.clone());
        let header_repo = FetchHeaderRepository::new(state.database.clone());
        let data_repo = FetchDataRepository::new(state.database.clone());
        Self {fetch_repo, member_repo, execute_repo, header_repo, data_repo, state}
    }

    /// #API AREA

    /// get all fetch api only super user
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
    
    /// delete fetch api
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
    pub async fn find_member(&self, user: User, fetch_id: i32, id: i32) -> Result<ApiMembers, AppError>{
        if !user.is_superuser {
            let is_allowed = match self.member_repo.find_member_id(fetch_id, user.id).await {
                Ok(_) => true,
                Err(_) => false, 
            };

            if !is_allowed {
                return Err(AppError::Forbidden("You don't have permission to view members!".to_string()));
            }
        }
        let q = self.member_repo.find_by_id(id)
            .await
            .map_err(|e| {AppError::NotFound(format!("Database : {}", e))})?;
        
        Ok(q)
    }

    /// Find all related fetch members
    pub async fn find_members(&self, user:User, fetch_id: i32) -> Result<Vec<ApiMembers>, AppError> {
        if !user.is_superuser {
            let is_allowed = match self.member_repo.find_member_id(fetch_id, user.id).await {
                Ok(_) => true,
                Err(_) => false, 
            };

            if !is_allowed {
                return Err(AppError::Forbidden("You don't have permission to view members!".to_string()));
            }
        }
        let q = self.member_repo.find_members(fetch_id)
            .await
            .map_err(|e| {AppError::NotFound(format!("Database : {}", e))})?;
        Ok(q)
    }

    // Add member of fetch id (Role:Owner required)
    pub async fn add_member(&self, user: User, fetch_id: i32, data: CreateApiMembers) -> Result<ApiMembers, AppError> {
        if !user.is_superuser {
            let is_allowed = match self.member_repo.find_member_id(fetch_id, user.id).await {
                Ok(member) => member.role == Some(Role::Owner),
                Err(_) => false, 
            };

            if !is_allowed {
                return Err(AppError::Forbidden("You don't have permission to add members!".to_string()));
            }
        }

        let create = self.member_repo.create(fetch_id, data)
            .await.map_err(|e| {AppError::NotFound(format!("Database : {}", e))})?;

        Ok(create)
    }

    // Edit member Role:Owner
    pub async fn update_member(&self, user: User,id: i32, fetch_id: i32, data:UpdateApiMembers) -> Result<ApiMembers, AppError> {
        if !user.is_superuser {
            let is_allowed = match self.member_repo.find_member_id(fetch_id, user.id).await {
                Ok(member) => member.role == Some(Role::Owner),
                Err(_) => false, 
            };

            if !is_allowed {
                return Err(AppError::Forbidden("You don't have permission to update members!".to_string()));
            }
        }

        let q = self.member_repo.update(id, data)
            .await?;

        Ok(q)
    }

    // Delete member Role:Owner
    pub async fn delete_member(&self, user: User, id: i32, fetch_id: i32) -> Result<ApiMembers, AppError> {
        if !user.is_superuser {
            let is_allowed = match self.member_repo.find_member_id(fetch_id, user.id).await {
                Ok(member) => member.role == Some(Role::Owner),
                Err(_) => false, 
            };

            if !is_allowed {
                return Err(AppError::Forbidden("You don't have permission to delete members!".to_string()));
            }
        }

        let target_member = self.member_repo.find_by_id(id).await
            .map_err(|_| AppError::NotFound("Member target not found".to_string()))?;

        if target_member.fetch_id != fetch_id {
            // Ini tanda-tanda request curang (ID project dan ID member tidak klop)
            return Err(AppError::BadRequest("Member target not a member on this fetch".to_string()));
        }
        
        let q = self.member_repo.delete(id)
            .await
            .map_err(|e| {AppError::NotFound(format!("Database: {}", e))})?;

        Ok(q)
    }

    /// get execute data (id)
    pub async fn get_execute(&self, user:User, id:i32) -> Result<ApiExecute, AppError> {
        let execute = self.execute_repo.find_by_id(id)
            .await.map_err(|e|{AppError::NotFound(format!("Database: {}",e))})?;

        if user.is_superuser || execute.user_id == user.id {
            return Ok(execute);
        }

        Err(AppError::Forbidden("You don't have permission to access this data".to_string()))
    }

    /// get all execute data user
    pub async fn get_all_execute(&self, user: User) -> Result<Vec<ApiExecute>, AppError> {
        let q = self.execute_repo.find_all(user.id)
            .await.map_err(|e|{AppError::NotFound(format!("Database: {}",e))})?;

        Ok(q)
    }

    /// create execute data
    pub async fn create_execute(&self, user: User, req: ReqCreateApiExecute) -> Result<ApiExecute, AppError> {
        let model: CreateApiExecute = req.into_model(user.id);
        let create = self.execute_repo.create(model)
            .await?;

        Ok(create)
    }

    /// update execute data
    pub async fn update_execute(&self, user: User, id: i32, req: UpdateApiExecute) -> Result<ApiExecute, AppError> {
        let execute = self.execute_repo.find_by_id(id).await?;
        if !user.is_superuser || execute.user_id != user.id {
            return Err(AppError::Forbidden("You don't have permission to access this data".to_string()));
        }
        
        Ok(self.execute_repo.update(id, req).await?)
    }

    /// delete execute data
    pub async fn delete_execute(&self, user: User, id: i32) -> Result<ApiExecute, AppError> {
        let execute = self.execute_repo.find_by_id(id).await?;
        if !user.is_superuser || execute.user_id != user.id {
            return Err(AppError::Forbidden("You don't have permission to access this data".to_string()));
        }

        Ok(self.execute_repo.delete(id).await?)
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