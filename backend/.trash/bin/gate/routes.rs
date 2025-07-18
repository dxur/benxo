use macros::routes;

use crate::AppState;

pub struct Routes;

#[routes(prefix="/test/gate", state=AppState)]
impl Routes {}
