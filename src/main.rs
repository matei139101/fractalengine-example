use fractalengine::{FractalEngine, Service, ServiceRegistry, log};
use fractalengine_dashboard::DashboardService;

fn main() {
    log!(Minor, "Started program.");

    let mut service_registry = ServiceRegistry::new();

    let dashboard_service = DashboardService::new();
    dashboard_service.register(&mut service_registry);
    
    let mut engine = FractalEngine::new(service_registry);
    engine.run();

    log!(Minor, "Shutdown program.");
}
