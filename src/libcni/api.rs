use crate::libcni::result::{APIResult, Result};
use crate::libcni::types;
use crate::libcni::types::{NetworkConfig, NetworkConfigList, RuntimeConf};

pub trait CNI {
    fn add_network_list(
        &self,
        net: NetworkConfigList,
        rt: RuntimeConf,
    ) -> Result<Box<dyn APIResult>>;

    fn check_network_list(&self, net: NetworkConfigList, rt: RuntimeConf) -> Result<()>;

    fn delete_network_list(&self, net: NetworkConfigList, rt: RuntimeConf) -> Result<()>;

    fn get_network_list_cached_result(
        &self,
        net: NetworkConfigList,
        rt: RuntimeConf,
    ) -> Result<Box<dyn APIResult>>;

    fn add_network(&self, net: NetworkConfig, rt: RuntimeConf) -> Result<Box<dyn APIResult>>;

    fn check_network(&self, net: NetworkConfigList, rt: RuntimeConf) -> Result<()>;
    fn delete_network(&self, net: NetworkConfigList, rt: RuntimeConf) -> Result<()>;

    fn get_network_cached_result(
        &self,
        net: NetworkConfig,
        rt: RuntimeConf,
    ) -> Result<Box<dyn APIResult>>;

    fn get_network_cached_config(
        &self,
        net: NetworkConfig,
        rt: RuntimeConf,
    ) -> Result<(Vec<u8>, Box<dyn APIResult>)>;

    fn validate_network_list(&self, net: NetworkConfigList) -> Result<Vec<String>>;

    fn validate_network(&self, net: NetworkConfig) -> Result<Vec<String>>;
}
