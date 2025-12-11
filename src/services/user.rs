use axum::{extract::{FromRef, FromRequestParts},http::request::Parts};
use crate::{models::user::UpdateUserReq, utils::response::AppError};
use crate::{repositories::user::UserRepository, state::AppState};
use crate::models::user::{User, UserProfile};

#[allow(dead_code)]
pub struct UserService {
    user_repo: UserRepository,
    state: AppState,
}

impl UserService {
    pub fn new(state: AppState)-> Self {
        let user_repo = UserRepository::new(state.database.clone());
        Self { user_repo, state }
    }

    pub async fn get_profile(&self, user: &User) -> Result<UserProfile, AppError> {

        Ok(UserProfile { id: user.id, username: user.username.clone(), email: user.email.clone(), is_superuser: user.is_superuser, created_at: user.created_at, updated_at: user.updated_at })
    }

    pub async fn update_profile(&self, current_user: &User, req: &UpdateUserReq) -> Result<UserProfile, AppError> {    
        let new_username = req.username
            .as_ref()
            .unwrap_or(&current_user.username)
            .clone();

        let new_email = req.email
            .as_ref()
            .unwrap_or(&current_user.email)
            .clone();

        let new_is_superuser = req.is_superuser
            .unwrap_or(current_user.is_superuser);

        let user_to_save = User {
            id: current_user.id,
            username: new_username,
            email: new_email,
            is_superuser: new_is_superuser,
            password: current_user.password.clone(),
            created_at: current_user.created_at,
            updated_at: current_user.updated_at,
        };

        let updated_user_db = self.user_repo.update(user_to_save).await?;

        Ok(UserProfile::from(updated_user_db))
    }
}


impl<S> FromRequestParts<S> for UserService
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        
        Ok(UserService::new(state))
    }
}