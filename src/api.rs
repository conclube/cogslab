#[derive(Eq, Copy, Clone, PartialEq)]
pub enum ContainerStatus {
    UNTOUCHED,
    INITIALIZED,
    RUNNING,
    FROZEN,
    TERMINATED
}

pub trait Container {
    fn initialize(&self) -> Result<(),None>;

    fn compile(&self) -> Result<(),None>;

    fn tick(&self) -> Result<(),None>;

    fn freeze(&self) -> Result<(),None>;

    fn unfreeze(&self) -> Result<(),None>;

    fn stop(&self) -> Result<(),None>;

    fn get_status(&self) -> ContainerStatus;
}

//common cog data
pub struct CogInfo {

}

//super type of all cogs
pub trait Cog {
    fn get_info(&self) -> CogInfo;

}

pub trait CogHolder {

}