pub mod result100;

pub type Result<T> = std::result::Result<T, Box<super::error::Error>>;

#[typetag::serde(tag = "type")]
pub trait APIResult {
    fn version(&self) -> String;
    fn get_as_version(&self, version: String) -> Result<Box<dyn APIResult>>;
    fn print(&self) -> Result<()>;
    fn print_to(&self, w: Box<dyn std::io::Write>) -> Result<()>;
}
