use crate::*;

pub struct MacOsNetComponentLookup;
impl cross::NetComponentLookup for MacOsNetComponentLookup {
    fn lookup_hostname(&self) -> cross::BridgeResult<String> {
        unix::lookup_hostname()
    }
    
    fn lookup_domain(&self) -> cross::BridgeResult<cross::Capable<Option<String>>> {
        cross::BridgeError::err_incapable(cross::Capability::Domains)
    }

    fn lookup_domain_authorities(&self) -> cross::BridgeResult<Vec<cross::DomainAuthority>> {
        cross::BridgeError::err_incapable(cross::Capability::Domains)
    }
}
