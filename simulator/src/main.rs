mod simulator;
mod http;

use std::{sync::{Arc, Mutex}, time::Duration};
use simulator::param::{SimulatorParameters, SimulatorState};
use minijinja::Environment;
use axum::{Router, http::header, response::IntoResponse, routing::get};
use common::{self};
use http::{controllers, views};

const BOOTSTRAP_CSS: &[u8] = include_bytes!("./http/static/css/bootstrap.min.css");
const BOOTSTRAP_JS: &[u8] = include_bytes!("./http/static/js/bootstrap.bundle.min.js");

struct AppState {
    env: Environment<'static>,
    state: Arc<Mutex<SimulatorState>>,
}

async fn serve_bootstrap_css() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/css")], BOOTSTRAP_CSS.to_vec())
}

async fn serve_bootstrap_js() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/javascript")],
        BOOTSTRAP_JS.to_vec(),
    )
}


fn spawn_sim_thread(state: Arc<Mutex<SimulatorState>>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(1));

        let mut simulator_state = state.lock().unwrap();

        simulator_state.tick();

        println!(
            "Simulated time: {} (Elapsed simulation time: {})",
            simulator_state.timer.get_formated_simulated_time(),
            simulator_state.timer.get_formated_elapsed_simulation_time()
        );
        // lock relâché ici quand `simulator_state` sort du scope
    })
}

#[tokio::main]
async fn main() {
    println!("Running the Company Simulator! Version 1.0");
    let parameters = SimulatorParameters::default();
    let mut simulator_state = SimulatorState::new(parameters);
    simulator_state.event_scheduler.init_default_events();
    let port = match common::get_port_from_args() {
        Ok(p) => p,
        Err(err) => {
            eprintln!("{}", err);
            common::print_usage();
            std::process::exit(1);
        }
    };
    println!("Server starts on port : {port}");
    println!("Type http://localhost:{port} in your browser.");

    let mut env = Environment::new();
    views::template::add_template(&mut env);
    let state = Arc::new(Mutex::new(simulator_state));

    let _sim_handle = spawn_sim_thread(Arc::clone(&state));

    let app_state = Arc::new(AppState { env, state: state.clone() });
    let app = Router::new()
        .route("/", get(controllers::home::controller_home))
        .route("/css/bootstrap.min.css", get(serve_bootstrap_css))
        .route("/js/bootstrap.bundle.min.js", get(serve_bootstrap_js))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
