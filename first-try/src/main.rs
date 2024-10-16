use std::any::Any;

// Define Input struct to hold conditions for transitions
struct Input {
    command_stop: bool,
    emergency_stop: bool,
}

// Define trait for transition capabilities
trait TransitionTo<T> {
    fn success(&self, input: &Input) -> bool;
}

// Define a trait for general state behavior with a method to get the state name
trait State: Any {
    fn state_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
}

// Define each state as an empty struct
#[derive(Clone)]
struct Init;
#[derive(Clone)]
struct Idle;
#[derive(Clone)]
struct Braking;
#[derive(Clone)]
struct Emergency;

// Implement the State trait for each state
impl State for Init {
    fn state_name(&self) -> &'static str {
        "Init"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl State for Idle {
    fn state_name(&self) -> &'static str {
        "Idle"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl State for Braking {
    fn state_name(&self) -> &'static str {
        "Braking"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl State for Emergency {
    fn state_name(&self) -> &'static str {
        "Emergency"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Implement the TransitionTo trait for each valid state transition
impl TransitionTo<Idle> for Init {
    fn success(&self, _input: &Input) -> bool {
        true // Always transition from Init to Idle
    }
}

impl TransitionTo<Braking> for Idle {
    fn success(&self, input: &Input) -> bool {
        input.command_stop // Transition to Braking if command_stop is true
    }
}

impl TransitionTo<Idle> for Braking {
    fn success(&self, input: &Input) -> bool {
        !input.command_stop // Transition to Idle if command_stop is false
    }
}

impl TransitionTo<Emergency> for Idle {
    fn success(&self, input: &Input) -> bool {
        input.emergency_stop // Transition to Emergency if emergency_stop is true
    }
}

impl TransitionTo<Idle> for Emergency {
    fn success(&self, input: &Input) -> bool {
        !input.emergency_stop // Transition to Idle if emergency_stop is false
    }
}

// Define the state machine struct and step function
struct BrakeController {
    state: Box<dyn State>,
}

impl BrakeController {
    fn new() -> Self {
        Self {
            state: Box::new(Init),
        }
    }

    fn step(&mut self, input: &Input) {
        if self.state.as_any().downcast_ref::<Init>().is_some() && Init.success(input) {
            self.state = Box::new(Idle);
        } else if let Some(idle_state) = self.state.as_any().downcast_ref::<Idle>() {
            if <Idle as TransitionTo<Braking>>::success(idle_state, input) {
                self.state = Box::new(Braking);
            } else if <Idle as TransitionTo<Emergency>>::success(idle_state, input) {
                self.state = Box::new(Emergency);
            }
        } else if let Some(braking_state) = self.state.as_any().downcast_ref::<Braking>() {
            if <Braking as TransitionTo<Idle>>::success(braking_state, input) {
                self.state = Box::new(Idle);
            }
        } else if let Some(emergency_state) = self.state.as_any().downcast_ref::<Emergency>() {
            if <Emergency as TransitionTo<Idle>>::success(emergency_state, input) {
                self.state = Box::new(Idle);
            }
        }
    }

    // Method to get the current state's name
    fn print_current_state(&self) {
        println!("Current state: {}", self.state.state_name());
    }
}

// Main function to demonstrate the state machine functionality
fn main() {
    let mut controller = BrakeController::new();

    // Create an input struct instance
    let mut input = Input {
        command_stop: false,
        emergency_stop: false,
    };

    // Print the initial state
    controller.print_current_state();

    // Normal condition, no command_stop or emergency
    controller.step(&input);
    controller.print_current_state();

    // Trigger the braking condition
    input.command_stop = true;
    controller.step(&input);
    controller.print_current_state();

    // Transition to Idle again
    input.command_stop = false;
    controller.step(&input);
    controller.print_current_state();

    // Emergency condition
    input.emergency_stop = true;
    controller.step(&input);
    controller.print_current_state();

    // Clear emergency condition
    input.emergency_stop = false;
    controller.step(&input);
    controller.print_current_state();
}
