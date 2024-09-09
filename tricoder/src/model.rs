use serde::Deserialize;
pub struct Port {
    pub port: u16,
    pub is_open: bool,
}
pub struct Subdomain {
    pub domain: String,
    pub open_ports: Vec<Port>
}

#[derive(Debug, Clone, Deserialize)]
pub struct CrtShEntry {
    pub name_value: String,
}
