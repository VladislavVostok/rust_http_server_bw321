mod auth;
mod path;
mod to_do;

use auth::auth_views_factroy;
use to_do::to_do_views_factory;
use actix_web::web::ServiceConfig;

pub fn views_factory(app: &mut ServiceConfig){
    auth_views_factroy(app);
    to_do_views_factory(app);
}