use crate::api::{Container, ContainerStatus};
use crate::api::ContainerStatus::{FROZEN, RUNNING};

struct ContainerImpl {
    status: ContainerStatus
}

impl ContainerImpl {
    pub fn new() -> ContainerImpl {
        return ContainerImpl {
            status: ContainerStatus::UNTOUCHED,
        }
    }
}

impl Container for ContainerImpl {
    fn initialize(&self) -> Result<(), None> {
        todo!()
    }

    fn compile(&self) -> Result<(), None> {
        todo!()
    }

    fn tick(&self) -> Result<(), None> {
        todo!()
    }

    fn freeze(&mut self) -> Result<(), None> {
        if self.status == FROZEN {
            //just return if already frozen
            return Ok(())
        }

        self.status = FROZEN;

        return Ok(())
    }

    fn unfreeze(&mut self) -> Result<(), None> {
        if self.status != RUNNING {
            //just return if not running
            return Ok(())
        }

        self.status = RUNNING;

        return Ok(())
    }

    fn stop(&self) -> Result<(), None> {
        todo!()
    }

    fn get_status(&self) -> ContainerStatus {
        return *self.status;
    }
}
