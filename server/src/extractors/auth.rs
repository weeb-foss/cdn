use std::future::{Ready, ready};

use actix_identity::Identity;
use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use database::UserModel;

use crate::AppError;

impl FromRequest for UserModel {
    type Error = Error;
    type Future = Ready<Result<UserModel, AppError>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        if let Ok(identify) = Identity::from_request(req, payload).into_inner() {
            if let Ok(user_json) = identify.id() {
                if let Ok(user) = eserde::json::from_str(&user_json) {
                    return ready(Ok(UserModel::from(user)));
                }
            }
        }

        ready(Err(AppError::AuthorizationError.into()))
    }
}
