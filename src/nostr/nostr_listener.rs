pub struct NostrListener {}

impl NostrListener {
    pub fn new() -> Self {
        NostrListener {}
    }

    pub fn listen_and_dispatch(&self, orchestrator: &Orchestrator) {
        // Mock Nostr event
        let nostr_event = "bitmap_inscription_detected";
        orchestrator.broadcast_state(nostr_event);
    }
}
