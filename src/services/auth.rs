use crate::models::user::User; 
use crate::repositories::user::*;
use crate::repositories::token::*;
use crate::utils::auth::*; 
use crate::models::auth::{LoginReq, LoginRes};
use crate::state::AppState;
use crate::utils::response::AppError;
use chrono::{Duration, Utc};

pub struct AuthService {
    user_repo: UserRepository,
    token_repo: TokenRepository,
    state: AppState,
}

impl AuthService {
    pub fn new(state: AppState) -> Self {
        let user_repo = UserRepository::new(state.database.clone());
        let token_repo = TokenRepository::new(state.database.clone());
        Self { user_repo, token_repo ,state }
    }
    
    pub async fn login(&self, req: LoginReq) -> Result<LoginRes, AppError> {
        let user = self.authenticate(&req.identifier, &req.password).await?;
        let expiration_time = Utc::now() + Duration::seconds(self.state.jwt_config.refresh_ttl); 
        let access_token = gen_access_token(&user, &self.state).await?;
        let refresh_token = gen_refresh_token(&user, &self.state).await?;
        self.token_repo.save_token(&refresh_token, user.id, expiration_time).await?;

        Ok(LoginRes { access_token, refresh_token })
    }

    pub async fn logout(&self, refresh_token_str: String) -> Result<(), AppError> {
        let _ = verify_refresh_token(&self.state.jwt_config.secret, &refresh_token_str)
             .map_err(|_| AppError::AuthError("Invalid token".to_string()))?;

        self.token_repo.revoke(&refresh_token_str).await?;
        
        Ok(())
    }

    pub async fn refresh(&self, token_str: String) -> Result<LoginRes, AppError> {
        let claims = verify_refresh_token(&self.state.jwt_config.secret, &token_str)?;
        let exists = self.token_repo.exists(&token_str)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

        if !exists {
            return Err(AppError::AuthError("Refresh token has been revoked".to_string()));
        }

        let user_id = claims.sub.parse::<i32>()
            .map_err(|_| AppError::AuthError("Invalid ID format".to_string()))?;

        let user = self.user_repo.find_by_id(&user_id).await?;
        let expiration_time = Utc::now() + Duration::seconds(self.state.jwt_config.refresh_ttl); 
        let access_token = gen_access_token(&user, &self.state).await?;
        let refresh_token = gen_refresh_token(&user, &self.state).await?;
        self.token_repo.revoke(&token_str).await?;
        self.token_repo.save_token(&refresh_token, user_id, expiration_time).await?;

        Ok(LoginRes { access_token, refresh_token })
    }

    async fn authenticate(&self, identifier: &str, password: &str) -> Result<User, AppError> {
        let user_opt = self.user_repo.find_by_username_or_email(identifier).await?;
        let user = match user_opt {
            Some(u) => u,
            None => return Err(AppError::AuthError("Invalid identifier or password".to_string())),
        };

        let plain_password = password.to_string();
        let hash_from_db = user.password.clone();

        let is_valid = tokio::task::spawn_blocking(move || {
            crate::utils::hash::verify(&plain_password, &hash_from_db)
        })
        .await
        .map_err(|e| AppError::InternalError(format!("Hash verify failed: {}", e)))??;
 
        if !is_valid {
            return Err(AppError::AuthError("Invalid identifier or password".to_string()));
        }

        Ok(user)
    }
}