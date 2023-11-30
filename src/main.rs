mod engine;
use engine::Engine;

fn main() {
    let engine_instance = Engine::new(Some(800u32), Some(600u32), None);
    engine_instance.run();
}
