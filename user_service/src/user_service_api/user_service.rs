use actix_web::Json;
use actix_web::Path;
use actix_web::State;
use futures::Future;
use crate::db_connection::CurrentSession;
use crate::model::{UserRegistration, User, UserLogin};
use crate::error::CustomError;
use crate::utility::Outcomes;

///AppState is a struct with current session as field
pub struct AppState {
    pub session: CurrentSession,
}

pub trait UserService {
    fn create_user(
        data: State<AppState>,
        user_reg: Json<UserRegistration>,
    ) -> Box<Future<Item=Json<User>, Error=CustomError>>;

    fn get_user(
        data: State<AppState>,
        user_id: Path<String>,
    ) -> Box<Future<Item=Json<User>, Error=CustomError>>;

    fn get_all_users(
        data: State<AppState>,
    ) -> Box<Future<Item=Json<Outcomes<User>>, Error=CustomError>>;

    fn user_login(
        data: State<AppState>,
        user_login: Json<UserLogin>,
    ) -> Box<Future<Item=String, Error=CustomError>>;
}
