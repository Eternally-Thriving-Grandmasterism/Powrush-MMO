# mod tests {
    use powrush_divine_module::{MercyCore, ValenceGate};
    use ra_thor_core::ValenceScore;

    # async fn mercy_gate_blocks_harm() {
        let mut core = MercyCore::new();
        let fake_msg = b"{\"type\":\"harm_action\",\"target\":\"ally\"}";
        
        let result = core.gate_server_message(fake_msg).await;
        assert!(result.is_err(), "Harm should be blocked");
        assert_eq!(result.err().unwrap().to_string().contains("valence"), true);
    }

    # fn valence_gate_allow_high() {
        let gate = ValenceGate::new(0.75);
        assert!(gate.allow_action(ValenceScore(0.85)));
        assert!(!gate.allow_action(ValenceScore(0.60)));
    }
}
