pub mod client;
pub mod request;
pub mod response;
pub mod circuit;


use self::{request::ReboundRequest, circuit::Circuit};

pub struct ReboundEngine {

    circuit: Circuit

}

impl ReboundEngine {

    pub fn new(circuit: Circuit) -> Self {
        ReboundEngine { circuit }
    }

    pub fn get(&mut self, req: impl Into<ReboundRequest>) -> Option<ReboundRequest> {

        let req: ReboundRequest = req.into();
        let cnode = self.circuit.get_node(req.uri.as_str());
        req.apply(cnode)
    }
}