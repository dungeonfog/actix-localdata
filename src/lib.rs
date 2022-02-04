use std::{ops::Deref, rc::Rc};

use actix_web::{dev::Payload, error::ErrorInternalServerError, FromRequest, HttpRequest};
use futures_util::future::{err, ok, Ready};

pub struct LocalData<T> {
    data: Rc<T>,
}
impl<T> LocalData<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Rc::new(data),
        }
    }
}
impl<T> Clone for LocalData<T> {
    fn clone(&self) -> Self {
        Self {
            data: Rc::clone(&self.data),
        }
    }
}
impl<T> Deref for LocalData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T: 'static> FromRequest for LocalData<T> {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, actix_web::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(data) = req.app_data::<Self>() {
            ok(data.clone())
        } else {
            err(ErrorInternalServerError(
                "the requested LocalData was not configured",
            ))
        }
    }
}
