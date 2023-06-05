use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use eyre::Result;
use crate::utils::error::Error;
use sui_sponsor_common::{
  utils::store::Store
};

#[derive(Deserialize)]
pub struct Body;

#[derive(Serialize)]
pub struct Response;

pub async fn exec(
  _store: web::Data<Store>,
  _body: web::Json<Body>,
) -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().json(Response {}))
}