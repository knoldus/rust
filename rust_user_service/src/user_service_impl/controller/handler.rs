use actix_web::{Json, Result};
use actix_web::Path;
use actix_web::State;
use eventsourcing::Aggregate;
use uuid::Uuid;

use crate::user_service_impl::constants::constant::TAKE_FIRST;
use crate::user_service_impl::controller::error::CustomError;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::env_setup::keyspace::create_keyspace;
use crate::user_service_impl::env_setup::table::create_table;
use crate::user_service_impl::eventsourcing::user_command::models::UserCommand;
use crate::user_service_impl::eventsourcing::user_event::models::UserEvent;
use crate::user_service_impl::eventsourcing::user_repository::display::select_user;
use crate::user_service_impl::eventsourcing::user_repository::insertion::event_persistent;
use crate::user_service_impl::eventsourcing::user_repository::is_present::is_present;
use crate::user_service_impl::eventsourcing::user_state::models::UserState;
use crate::user_service_impl::models::get_user::GetUser;
use crate::user_service_impl::models::p_user::PUser;
use crate::user_service_impl::models::user::User;
use crate::user_service_impl::models::user_login::UserLogin;
use crate::user_service_impl::models::user_registration::UserRegistration;
use crate::user_service_impl::utilities::initial_state::initial_state;
use crate::user_service_impl::utilities::mappers::user_mapper;
use crate::user_service_impl::eventsourcing::user_repository::display::select_all_user;

pub struct AppState {
    pub session: CurrentSession,
}

pub fn initializer(data: State<AppState>) -> Result<&'static str> {
    create_keyspace(&data.session);
    create_table(&data.session);
    Ok("environment successfully up")
}

pub fn create_user(data: State<AppState>, user_reg: Json<UserRegistration>)
                   -> Result<Json<User>, CustomError> {
    let new_user: UserRegistration = user_reg.into_inner();
    let new_user_id: String = get_id_by_email(&new_user).to_string();
    if is_present(&data.session, new_user_id.clone()) {
        let initial_user_state: UserState = initial_state();
        let create_user_command: UserCommand = UserCommand::CreateUser(new_user);
        let user_events: Vec<UserEvent> =
            PUser::handle_command(&initial_user_state, create_user_command).unwrap();
        let user_state: UserState =
            PUser::apply_event(&initial_user_state, user_events[TAKE_FIRST].clone()).unwrap();
        match event_persistent(&data.session, &user_events[TAKE_FIRST],
                               new_user_id, &user_state) {
            Ok(_) => Ok(Json(user_mapper(user_state.user))),
            _custom_error => Err(CustomError::InvalidInput { field: "Internal Server Error" }),
        }
    } else {
        Err(CustomError::InvalidInput {
            field: "user with this state already exist",
        })
    }
}

pub fn get_user(data: State<AppState>, user_id: Path<String>) -> Result<Json<User>, CustomError> {
    let result: Vec<GetUser> = select_user(&data.session, user_id.into_inner());
    if result.is_empty() {
        Err(CustomError::InvalidInput { field: "user with this id doesn't exist" })
    } else {
        let user_state: UserState = serde_json::from_str(&result[TAKE_FIRST].user_state).unwrap();
        Ok(Json(user_mapper(user_state.user)))
    }
}

pub fn get_all_users(data: State<AppState>) -> Result<Vec<User>, CustomError> {
    let result: Vec<GetUser> = select_all_user(&data.session);
    if result.is_empty() {
        Err(CustomError::InternalError { field: "error in getting all users" })
    } else {
        let user_state: UserState = serde_json::from_str(&result[TAKE_FIRST].user_state).unwrap();
        Ok(vec![user_mapper(user_state.user)])
    }
}

pub fn user_login(data: State<AppState>,user_login: Json<UserLogin>) -> Result<&'static str, CustomError> {
    unimplemented!()
}

/// this method is used to retrieve the id from email
pub fn get_id_by_email(user_reg: &UserRegistration) -> Uuid {
    let user_id = Uuid::new_v5(&Uuid::NAMESPACE_URL, user_reg.email.as_bytes());
    user_id
}
