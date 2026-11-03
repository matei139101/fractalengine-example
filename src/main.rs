use fractalengine::{FractalEngine, Service, ServiceRegistry, log};
use fractalengine_dashboard::DashboardService;

fn main() {
    log!(Minor, "Registering services.");
    let mut service_registry = ServiceRegistry::new();

    let dashboard_service = DashboardService::new();
    dashboard_service.register(&mut service_registry);
    
    log!(Minor, "Creating engine");
    let mut engine = FractalEngine::new(service_registry);


    log!(Minor, "Initializing engine.");
    engine.init();

    log!(Minor, "Running engine.");
    engine.run();

    log!(Minor, "Closing engine.");
    engine.shutdown();
}
