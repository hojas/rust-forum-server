use axum_sessions::{async_session::MemoryStore, SessionLayer};
use rand::Rng;

pub fn session_layer() -> SessionLayer<MemoryStore> {
    let store = MemoryStore::new();
    let mut secret = [0u8; 128];
    let mut rng = rand::thread_rng();
    rng.fill(&mut secret);

    SessionLayer::new(store, &secret)
}
