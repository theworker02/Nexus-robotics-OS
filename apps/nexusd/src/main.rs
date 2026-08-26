use nexus_gateway::Gateway;
fn main() {
    let mut gateway = Gateway::default();
    for _ in 0..4 {
        gateway.advance();
    }
    println!(
        "nexusd local gateway READY\nstate={:?}\nlocal emergency control={}",
        gateway.state, gateway.local_emergency_control
    );
}
