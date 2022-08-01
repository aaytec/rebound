use std::collections::HashMap;

pub struct ReboundResponse {
    
    pub status: u16,
    
    pub headers: HashMap<String, String>,

    pub body: Vec<u8>

}


impl ReboundResponse {
        pub fn from(res: surf::Response, bytes: Vec<u8>) -> Self {

        let sc: u16 = res.status().into();
        let hdrs_vec: Vec<(String, String)> = res.header_names().map(|h| (String::from(h.as_str()), String::from(res.header(h).expect("failed to get header").as_str()))).collect();        
        ReboundResponse {
            status: sc,
            headers: hdrs_vec.into_iter().collect(),
            body: bytes
        }
    }
}