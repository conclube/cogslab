

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

    fn freeze(&self) -> Result<(), None> {
        todo!()
    }

    fn unfreeze(&self) -> Result<(), None> {
        todo!()
    }

    fn stop(&self) -> Result<(), None> {
        todo!()
    }

    fn get_status(&self) -> ContainerStatus {
        return *self.status;
    }
}
