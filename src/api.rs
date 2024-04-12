use cogslab::{ContainerImpl, ContainerStatus};

pub trait Container {
    fn initialize(&self) -> Result<(),None>;

    fn compile(&self) -> Result<(),None>;

    fn tick(&self) -> Result<(),None>;

    fn freeze(&self) -> Result<(),None>;

    fn unfreeze(&self) -> Result<(),None>;

    fn stop(&self) -> Result<(),None>;

    fn get_status(&self) -> ContainerStatus;
}