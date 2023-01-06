use std::marker::PhantomData;

mod type_state_param;
use type_state_param::{Machine, MachineState, Start};

fn main() {
    let mut machine: Machine<Start> = Machine {
        is_stopped: false,
        _marker: PhantomData,
    };

    loop {
        match machine.start() {
            MachineState::Start(__machine) => {
                machine = __machine;
            }
            MachineState::Stop(__machine) => {
                __machine.stop();
                break;
            }
        }
    }
}
