use std::cmp::Ordering;

use crate::conf::ReboundRule;

type NodeIndex = usize;
type LinkIndex = usize;

#[derive(Clone)]
pub enum CircuitType {
    Routable,
    Error
}

#[derive(Clone)]
pub struct CircuitNode {

    pub circuit_type: CircuitType,
    
    pub rule: Option<ReboundRule>, 

    pub path: Option<CircuitPath>,

    pub links: Vec<LinkIndex>
    
}

impl CircuitNode {
    pub fn error() -> Self {
        CircuitNode { circuit_type: CircuitType::Error, rule: None, path: None, links: Vec::new() }
    }
}

impl From<ReboundRule> for CircuitNode {
    fn from(rule: ReboundRule) -> Self {
        let pattern = rule.pattern.clone();
        CircuitNode { 
            circuit_type: CircuitType::Routable,
            rule: Some(rule),
            path: Some(CircuitPath::from(pattern)),
            links: Vec::new()
        }
    }
}

pub struct CircuitLink {
    pub from: NodeIndex,
    pub to: NodeIndex
}

pub struct Circuit {
    pub nodes: Vec<CircuitNode>,
    pub links: Vec<CircuitLink>
}

impl Circuit {
    fn get_links(&self, index: NodeIndex) -> Vec<LinkIndex> {
        self.nodes
        .get(index)
        .map(|x| x.links.to_vec() )
        .unwrap_or_default()
    }

    fn add_node(&mut self, node: CircuitNode) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    fn get_node(&self, path: impl Into<CircuitPath>) -> usize {
        todo!()
    }

    fn add_link(&self, link: CircuitLink) {
        todo!()
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
            nodes: Vec::new(),
            links: Vec::new()
        };

        circuit.add_node(CircuitNode::error());

        self.rules
            .iter()
            .for_each(|x| {
                let node = CircuitNode::from(x.clone());

                let path = node.path.clone().unwrap();
                let to = circuit.add_node(node);
                let from = circuit.get_node(path);
                circuit.add_link(CircuitLink{ from, to })
            });


        circuit
    }

}

#[derive(Clone, Debug)]
pub struct CircuitPath {
    pub str_path: String,
    pub ordered_path: Vec<String>
}

impl PartialEq for CircuitPath {
    fn eq(&self, other: &Self) -> bool {
        self.str_path == other.str_path && self.ordered_path == other.ordered_path
    }
}

impl From<String> for CircuitPath {
    fn from(path: String) -> Self {

        let str_path = path.clone();
        let ordered_path: Vec<String> = str_path
                            .split('/')
                            .map(|x| String::from(x))
                            .collect();

        CircuitPath { 
            str_path,
            ordered_path
         }
    }
}

impl From<&str> for CircuitPath {

    fn from(path: &str) -> Self {
        CircuitPath::from(String::from(path))
    }

}