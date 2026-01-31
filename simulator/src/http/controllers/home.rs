use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use minijinja::context;
use std::sync::Arc;

use common::time_simulator::TimeScale;

use crate::AppState;

pub async fn controller_home(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("home").unwrap();

    let mut simulator_state = state.state.lock().unwrap();
    simulator_state.timer.set_time_scale(TimeScale::OneDayPerSecond);
    let rendered = template
        .render(context! {
            title => "Company Simulator Home",
        })
        .unwrap();

    Ok(Html(rendered))
}