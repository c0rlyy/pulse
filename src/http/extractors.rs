use axum::{
    async_trait, extract::FromRequestParts, http::request::Parts, Extension, RequestPartsExt,
};

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};

use crate::auth::claims::Claims;

use super::{
    error::{ApiError, AuthError},
    ApiContext,
};

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ApiError::Unauthorized)?;

        // this works
        let Extension(ctx) = parts
            .extract::<Extension<ApiContext>>()
            .await
            .expect("state should be provided ");

        let token_data = decode::<Claims>(
            bearer.token(),
            &ctx.config.decoding_key,
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
