trait FaucetFundSender {
    fn send(to: Vec<u8>, amount: u128);
}

struct EVMSender;
impl FaucetFundSender for EVMSender {
    fn send(to: Vec<u8>, amount: u128) {
        
    }
}

struct SubstrateSender;
impl FaucetFundSender for SubstrateSender {
    fn send(to: Vec<u8>, amount: u128) {
        
    }
}