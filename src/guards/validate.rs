// use actix_web::{web, FromRequest, HttpRequest, HttpResponse, Result};
// use serde::de::DeserializeOwned;
// use validator::Validate;

// #[derive(Debug)]
// struct ValidateJson<T: Validate + DeserializeOwned>(T);

// impl<T: Validate + DeserializeOwned> FromRequest for ValidateJson<T> {
//     type Error = actix_web::Error;
//     type Future = Result<Self, Self::Error>;
//     type Config = ();

//     fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
//         let json = web::Json::<T>::from_request(req, &mut actix_web::dev::Payload::None).await?;

//         if let Err(errors) = json.0.validate() {
//             return Err(HttpResponse::BadRequest().json(errors));
//         }

//         Ok(ValidateJson(json.0))
//     }
// }
