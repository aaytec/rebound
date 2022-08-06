use crate::conf::ReboundRule;

type NodePtr = usize;

#[allow(dead_code)]
type LinkPtr = usize;

#[derive(Clone, Debug)]
pub enum CircuitType {
    Routable,
    Error
}

#[derive(Clone, Debug)]
pub struct CircuitNode {

    pub circuit_type: CircuitType,
    
    pub rule: Option<ReboundRule>, 

    pub path: Option<CircuitPath>
    
}

impl CircuitNode {
    pub fn error() -> Self {
        CircuitNode { circuit_type: CircuitType::Error, rule: None, path: None }
    }
}

impl PartialEq<CircuitPath> for CircuitNode {
    fn eq(&self, other: &CircuitPath) -> bool {
        let ctype = &self.circuit_type;
        match ctype {
            CircuitType::Routable => return self.path.clone().unwrap().eq(other),
            CircuitType::Error => true,
        }
    }
}

impl From<ReboundRule> for CircuitNode {
    fn from(rule: ReboundRule) -> Self {
        let mut pattern = rule.pattern.clone();
        if !pattern.ends_with("/") {
            pattern += "/";
        }
        CircuitNode { 
            circuit_type: CircuitType::Routable,
            rule: Some(rule),
            path: Some(CircuitPath::from(pattern))
        }
    }
}

#[derive(Clone, Debug)]
pub struct CircuitLink {
    pub from: NodePtr,
    pub to: NodePtr
}

#[derive(Clone, Debug)]
pub struct Circuit {
    pub head_index: NodePtr,
    pub nodes: Vec<CircuitNode>,
    pub links: Vec<CircuitLink>
}

impl Circuit {

    fn add_node(&mut self, node: CircuitNode) -> NodePtr {
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    fn get_node_ptr(&self, path: impl Into<CircuitPath>) -> NodePtr {
        let path: CircuitPath = path.into();
        let mut current_ptr: NodePtr = self.head_index;
        let mut node = self.nodes.get(current_ptr).unwrap();

        while node.eq(&path) {

            let nodes = self.links.iter()
                .filter(|x| x.from == current_ptr)
                .map(|x| {
                    let to_node = x.to;
                    let n = self.nodes.get(to_node).unwrap();
                    (to_node, n)
                })
                .filter(|(_i, x)| (*x).eq(&path) )
                .collect::<Vec<(usize, &CircuitNode)>>();

            match nodes.first() {
                Some((i, n)) => {
                    current_ptr = *i;
                    node = *n;
                },
                None => {
                    break;
                },
            }

        };

        current_ptr
    }

    pub fn get_node(&self, path: impl Into<CircuitPath>) -> &CircuitNode {
        let ptr: NodePtr = self.get_node_ptr(path);
        self.nodes.get(ptr).unwrap()
    }

    fn add_rule(&mut self, rule: &ReboundRule) {
        let node = CircuitNode::from(rule.clone());
        let path = node.path.clone().unwrap();
        
        let from = self.get_node_ptr(path);
        let to = self.add_node(node);
        self.links.push( CircuitLink{ from, to });
    }
}

pub struct CircuitBuilder {

    rules: Vec<ReboundRule>

}

impl CircuitBuilder {
   
    pub fn new(rules: Vec<ReboundRule>) -> Self {

        CircuitBuilder {
            rules
        }
    }

    pub fn build(&mut self) -> Circuit {
                    
        let mut circuit = Circuit {
            head_index: 0,
            nodes: Vec::new(),
            links: Vec::new()
        };

        circuit.add_node(CircuitNode::error());

        self.rules
            .iter()
            .for_each(|x| { circuit.add_rule(x); });

        circuit
    }

}

#[derive(Clone, Debug)]
pub enum CircuitUpstreamSchema {
    Http,
    Https
}

pub fn get_circuit_schema(c_upstream: &String) -> CircuitUpstreamSchema {
    if c_upstream.starts_with(CircuitUpstreamSchema::Http.into_str()) {
        CircuitUpstreamSchema::Http
    }
    else if c_upstream.starts_with(CircuitUpstreamSchema::Https.into_str()) {
        CircuitUpstreamSchema::Https
    }
    else {
        CircuitUpstreamSchema::Http
    }
}

impl CircuitUpstreamSchema {
    fn into_str(&self) -> &str {
        match self {
            CircuitUpstreamSchema::Http => "http://",
            CircuitUpstreamSchema::Https => "https://",
        }
    }
}

#[derive(Clone, Debug)]
pub struct CircuitUpstream {

    pub schema: CircuitUpstreamSchema,

    pub host: String,

    pub path: CircuitPath

}

impl From<String> for CircuitUpstream {
    fn from(upstream: String) -> Self {

        let schema = get_circuit_schema(&upstream);
        let path_upstream = upstream.strip_prefix(schema.into_str());
        let mut cpath = CircuitPath::from(path_upstream.unwrap());

        // host[:port] will be first in split('/')
        let host = cpath.ordered_path.remove(0); 

        CircuitUpstream { schema: schema, host: host, path: cpath }
    }
}

impl CircuitUpstream {
    pub fn join(&self, path: &CircuitPath) -> Self {

        let mut cup = self.clone();
        cup.path = cup.path.join(path);
        cup
    }

    pub fn path_undefined(&self) -> bool {
        return self.path.ordered_path.is_empty()
    }
}

impl Into<String> for CircuitUpstream {
    fn into(self) -> String {

        let full_uri = match self.path.ordered_path.is_empty() {
            true =>     format!("{}/", self.host),
            false =>    [
                            self.host,
                            self.path.into()
                        ]
                        .join("/"),
        };

        format!("{}{}", self.schema.into_str(), full_uri)
    }
}

#[derive(Clone, Debug)]
pub struct CircuitPath {
    pub is_resource_dir: bool,
    pub ordered_path: Vec<String>
}

impl CircuitPath {
    
    pub fn join(&self, other: &CircuitPath) -> CircuitPath {
        let mut new_path = self.ordered_path.to_vec();
        new_path.append(&mut other.ordered_path.to_vec());
        CircuitPath { ordered_path: new_path, is_resource_dir: other.is_resource_dir }
    }

    pub fn get_diff(&self, other: &CircuitPath) -> Self {

        let mut new_path = self.ordered_path.to_vec();
        let common_zip: Vec<_> = new_path.iter().zip(other.ordered_path.iter()).collect();

        let mut common_len = 0;
        for (left, right) in common_zip {
            if left == right {
                common_len += 1;
            }
            else {
                break;
            }
        }
        
        new_path.drain(0..common_len);
        CircuitPath { ordered_path: new_path, is_resource_dir: other.is_resource_dir }

    }
}

impl Into<String> for CircuitPath {
    fn into(self) -> String {
        if self.is_resource_dir {
            return format!("{}/", self.ordered_path.join("/"))
        }

        self.ordered_path.join("/")
    }
}

impl PartialEq for CircuitPath {
    fn eq(&self, other: &Self) -> bool {
        if self.ordered_path.len() > other.ordered_path.len() {
            return false;
        }

        self.ordered_path.iter()
            .zip(other.ordered_path.iter())
            .filter(|(left, _)| !left.is_empty())
            .all(|(left, right)| left == right)
    }
}


impl From<String> for CircuitPath {
    fn from(path: String) -> Self {

        let str_path = path.clone();
        let ordered_path: Vec<String> = str_path
                            .trim_matches('/')
                            .split('/')
                            .map(|x| String::from(x))
                            .filter(|x| !x.is_empty())
                            .collect();

        CircuitPath { 
            ordered_path,
            is_resource_dir: path.ends_with('/')
         }
    }
}

impl From<&str> for CircuitPath {

    fn from(path: &str) -> Self {
        CircuitPath::from(String::from(path))
    }

}