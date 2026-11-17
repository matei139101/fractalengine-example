use fractalengine::{FractalEngine, log};
use fractalengine_dashboard::DashboardService;

fn main() {
    log!(Minor, "Creating engine");
    let mut engine = FractalEngine::default();

    log!(Minor, "Registering services.");
    let dashboard_service = DashboardService::new();
    engine.register_service(dashboard_service);


    log!(Minor, "Initializing engine.");
    engine.init();

    log!(Minor, "Running engine.");
    engine.run();

    log!(Minor, "Closing engine.");
    engine.cleanup();
}
