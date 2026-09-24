use fractalengine::{FractalEngine, ServiceRegistry};
use fractalengine_dashboard::DashboardService;

fn main() {
    println!("Hello, world!");

    let mut service_registry = ServiceRegistry::new();

    let dashboard_service = DashboardService::new();
    DashboardService::register(dashboard_service.clone(), &mut service_registry);
    
    let mut engine = FractalEngine::new(service_registry);
    engine.run();
}
