use std::{collections::HashMap, io::Cursor};
use std::str::FromStr;
use tiny_http::{Response, Header};

pub struct ReboundResponse {
    
    pub status: u16,
    
    pub headers: HashMap<String, String>,

    pub body: Box<Vec<u8>>

}


impl ReboundResponse {
        pub async fn from(mut res: surf::Response) -> Self {

        let sc: u16 = res.status().into();
        let hdrs_vec: Vec<(String, String)> = res.header_names().map(|h| (String::from(h.as_str()), String::from(res.header(h).expect("failed to get header").as_str()))).collect();
        ReboundResponse {
            status: sc,
            headers: hdrs_vec.into_iter().collect(),
            body: Box::from(res.body_bytes().await.unwrap_or_default())
        }
    }
}

impl Into<Response<Cursor<Vec<u8>>>> for ReboundResponse {
    fn into(self) -> Response<Cursor<Vec<u8>>> {
        let rebound_res_body_size = self.body.len();
        Response::new(
            self.status.into(),
            self.headers
                .iter()
                .map(|(k, v)| { 
                    Header::from_str(format!("{}:{}", k.as_str(), v.as_str()).as_str()).unwrap()
                })
                .collect::<Vec<Header>>(),
            Cursor::new(*self.body), 
            Some(rebound_res_body_size),
            None
        )
    }
}