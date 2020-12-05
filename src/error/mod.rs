use actix_web::{
    dev::HttpResponseBuilder, error, http::header, http::StatusCode, HttpResponse
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum UserFacingError {
    #[display(fmt = "An internal error occured. Please try again later.")]
    InternalError
}

impl error::ResponseError for UserFacingError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserFacingError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}