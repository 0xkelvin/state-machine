use std::any::Any;

// Define Input struct to hold conditions for transitions
struct Input {
    power_on: bool,
    brake: bool,
    e_stop: bool,
}

// Define trait for transition capabilities
trait TransitionTo<T> {
    fn success(&self, input: &Input) -> bool;
}

// Define a trait for general state behavior with a method to get the state name
trait State: Any {
    fn state_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
    fn perform_action(&self); // Method to perform actions specific to the state
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

// Implement the State trait for each state with specific actions
impl State for Init {
    fn state_name(&self) -> &'static str {
        "Init"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn perform_action(&self) {
        println!("System initializing...");
    }
}

impl State for Idle {
    fn state_name(&self) -> &'static str {
        "Idle"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn perform_action(&self) {
        println!("System is idle and ready.");
    }
}

impl State for Braking {
    fn state_name(&self) -> &'static str {
        "Braking"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn perform_action(&self) {
        println!("System is braking.");
    }
}

impl State for Emergency {
    fn state_name(&self) -> &'static str {
        "Emergency"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn perform_action(&self) {
        println!("Emergency mode activated!");
    }
}

// Implement the TransitionTo trait for each valid state transition
impl TransitionTo<Idle> for Init {
    fn success(&self, input: &Input) -> bool {
        input.power_on // Transition to Idle if power_on is true
    }
}

impl TransitionTo<Braking> for Idle {
    fn success(&self, input: &Input) -> bool {
        input.brake // Transition to Braking if brake is true
    }
}

impl TransitionTo<Idle> for Braking {
    fn success(&self, input: &Input) -> bool {
        !input.brake // Transition to Idle if brake is false
    }
}

impl TransitionTo<Emergency> for Idle {
    fn success(&self, input: &Input) -> bool {
        input.e_stop // Transition to Emergency if e_stop is true
    }
}

impl TransitionTo<Idle> for Emergency {
    fn success(&self, input: &Input) -> bool {
        !input.e_stop // Transition to Idle if e_stop is false
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

        // Call the action method after the state is updated
        self.state.perform_action();
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
        power_on: false,
        brake: false,
        e_stop: false,
    };

    // Print the initial state
    controller.print_current_state();
    controller.state.perform_action();

    // Transition to Idle when power is on
    input.power_on = true;
    controller.step(&input);
    controller.print_current_state();

    // Trigger braking condition
    input.brake = true;
    controller.step(&input);
    controller.print_current_state();

    // Release braking condition
    input.brake = false;
    controller.step(&input);
    controller.print_current_state();

    // Trigger emergency stop
    input.e_stop = true;
    controller.step(&input);
    controller.print_current_state();

    // Clear emergency stop
    input.e_stop = false;
    controller.step(&input);
    controller.print_current_state();
}
