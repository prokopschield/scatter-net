use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PeerUsage {
    pub sent_fetch_success: usize,
    pub sent_put_success: usize,
    pub served_fetch_requests: usize,
    pub served_put_requests: usize,
    pub received_unsol_fetch: usize,
    pub received_unsol_put: usize,
    pub reputation_score: isize,
    pub our_reputation_score: isize,
}

impl PeerUsage {
    pub fn inc_sent_fetch_success(&mut self) {
        self.sent_fetch_success += 1;
        self.reputation_score += 1;
    }

    pub fn inc_sent_put_success(&mut self) {
        self.sent_put_success += 1;
        self.reputation_score += 3;
        self.our_reputation_score -= 1;
    }

    pub fn inc_served_fetch_requests(&mut self) {
        self.served_fetch_requests += 1;
        self.our_reputation_score += 1;
    }

    pub fn inc_served_put_requests(&mut self) {
        self.served_put_requests += 1;
        self.reputation_score -= 1;
        self.our_reputation_score += 3;
    }

    pub fn inc_received_unsol_fetch(&mut self) {
        self.received_unsol_fetch += 1;
        self.reputation_score -= 7;
    }

    pub fn inc_received_unsol_put(&mut self) {
        self.received_unsol_put += 1;
        self.reputation_score -= 7;
    }
}
