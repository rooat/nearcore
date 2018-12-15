use primitives::types::*;

/// This structure is used to keep track of all violations detected on the network.
/// Not necessarily restricted to violations regarding TxFlow protocol,
/// but any kind of violation as enumerated in ViolationType
pub trait MisbehaviorReporter {
    fn new() -> Self;

    fn report(&mut self, violation: ViolationType);

    /// Returns one violation stored or None if its empty
    fn next(&mut self) -> Option<ViolationType>;
}

#[derive(Debug)]
pub struct DAGMisbehaviorReporter {
    pub violations: Vec<ViolationType>,
}

impl MisbehaviorReporter for DAGMisbehaviorReporter{
    fn new() -> Self {
        DAGMisbehaviorReporter { violations: vec![] }
    }

    /// Take ownership of the violation
    fn report(&mut self, violation: ViolationType) {
        self.violations.push(violation);
    }

    /// Violations vector behave like a LIFO
    fn next(&mut self) -> Option<ViolationType> {
        self.violations.pop()
    }
}


/// MisbehaviorReporter that ignore all information stored
pub struct NoopMisbehaviorReporter{
}

impl MisbehaviorReporter for NoopMisbehaviorReporter{
    fn new() -> Self{
        Self {}
    }

    fn report(&mut self, _violation: ViolationType) {
    }

    fn next(&mut self) -> Option<ViolationType> {
        None
    }
}

/// Enumeration of all TxFlow protocol violations.
/// Discussion at: https://github.com/nearprotocol/nearcore/issues/131
#[derive(Debug)]
pub enum ViolationType {
    /// Message with incorrect epoch
    BadEpoch(TxFlowHash),
    /// There is no BLS signature for representative when there must be one
    MissingEndorsement(TxFlowHash),
    /// Invalid part of the BLS signature
    InvalidEndorsement(TxFlowHash),
    /// Two messages from the same participant that are not approved by each other.
    ForkAttempt(TxFlowHash, TxFlowHash),

    /// Message contains invalid signature from participant.
    /// Someone pretending being another participant maybe.
    InvalidSignature(TxFlowHash),
}
