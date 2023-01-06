use std::marker::PhantomData;

pub struct Start;
pub struct Stop;

pub struct Machine<State> {
    pub is_stopped: bool,
    pub _marker: PhantomData<State>,
}

impl Machine<Start> {
    pub fn start(self) -> MachineState {
        match self.is_stopped {
            true => MachineState::Stop(Machine {
                is_stopped: true,
                _marker: PhantomData,
            }),
            false => MachineState::Start(Machine {
                is_stopped: true,
                _marker: PhantomData,
            }),
        }
    }
}

impl Machine<Stop> {
    pub fn stop(self) {
        println!("stopped");
    }
}

pub enum MachineState {
    Start(Machine<Start>),
    Stop(Machine<Stop>),
}
