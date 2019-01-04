use actix_web::http;
use actix_web::HttpResponse;
use actix_web::Result;
use cdrs::frame::Frame;
use cdrs::query::QueryExecutor;
use cdrs::types::prelude::Error;

use crate::user_service_impl::constants::queries::USER_EVENT_STORE_QUERY;
use crate::user_service_impl::constants::queries::USER_STATE_STORE_QUERY;
use crate::user_service_impl::controller::error::CustomError;
use crate::user_service_impl::env_setup::connection::CurrentSession;
use crate::user_service_impl::eventsourcing::user_event::models::UserEvent;
use crate::user_service_impl::eventsourcing::user_state::models::UserState;

pub fn event_persistent(
    session: &CurrentSession,
    new_user: &UserEvent,
    user_id: String,
    user_state: &UserState,
) -> Result<&'static str, CustomError> {
    let user_json: String = serde_json::to_string(&new_user).unwrap();
    session
        .query_with_values(
            USER_EVENT_STORE_QUERY,
            query_values!(user_id.clone(), user_json),
        )
        .expect("insert error");
    let status: HttpResponse = match state_persistent(&session, &user_state, user_id) {
        Ok(_) => HttpResponse::new(http::StatusCode::OK),
        Err(_) => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
    };
    match status {
        _OK => Ok("successfully event stored"),
        _INTERNAL_SERVER_ERROR => Err(CustomError::InternalError{field:"error in event persistent"}),
    }
}

fn state_persistent<'a, 'b>(
    session: &'a CurrentSession,
    new_user: &'b UserState,
    user_id: String,
) -> Result<&'static str, CustomError> {
    let user_state_json: String = serde_json::to_string(&new_user).unwrap();
    let query_status: Result<Frame, Error> = session.query_with_values(
        USER_STATE_STORE_QUERY,
        query_values!(user_id, user_state_json),
    );
    match query_status {
        _Frame => Ok("successfully state stored"),
        _Error => Err(CustomError::InternalError{field:"error in state persistent"}),
    }
}
