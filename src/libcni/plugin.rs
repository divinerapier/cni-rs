pub trait PluginInfo {
    fn supported_versions(&self) -> Vec<String>;
    fn encode<W: std::io::Write>(&self, w: W) -> super::Result<()>;
}

#[derive(serde::Serialize, serde::Deserialize)]
struct PluginInfoT {
    #[serde(rename = "cniVersion")]
    cni_version: String,
    #[serde(rename = "supportedVersions")]
    supported_versions: Vec<String>,
}

impl PluginInfo for PluginInfoT {
    fn supported_versions(&self) -> Vec<String> {
        self.supported_versions.clone()
    }

    fn encode<W: std::io::Write>(&self, w: W) -> super::Result<()> {
        todo!()
    }
}
