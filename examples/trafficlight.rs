// This example demonstrates how to use the `EnumTryAsInner` derive macro to help construct a state machine.
//
// Every state transition is guarded by a check that the current state is the expected one. Accessing the state
// is convenient, while propagating state errors can be done with the `?` operator.

mod state {
    use enum_try_as_inner::EnumTryAsInner;

    #[derive(Debug, EnumTryAsInner)]
    #[derive_err(Debug)]
    pub enum State {
        Red(Red),
        Yellow,
        Green(Green),
    }

    impl Default for State {
        fn default() -> Self {
            State::Red(Red::default())
        }
    }

    #[derive(Debug, Default)]
    pub struct Red {
        pub cars_waiting: usize,
    }

    #[derive(Debug, Default)]
    pub struct Green {
        pub cars_passed: usize,
    }
}

#[derive(Debug, Default)]
pub struct TrafficLight {
    state: state::State,
}

#[derive(Debug)]
pub enum TrafficLightError {
    InvalidState(String),
    NotEnoughCarsPassed(usize),
    NotEnoughCarsWaiting(usize),
}

impl From<state::StateError> for TrafficLightError {
    fn from(err: state::StateError) -> Self {
        TrafficLightError::InvalidState(err.to_string())
    }
}

impl TrafficLight {
    pub fn turn_red(&mut self) -> Result<(), TrafficLightError> {
        self.state.try_as_yellow()?;

        self.state = state::State::Red(state::Red::default());

        Ok(())
    }

    pub fn turn_yellow(&mut self) -> Result<(), TrafficLightError> {
        let &state::Green { cars_passed } = self.state.try_as_green()?;

        if cars_passed > 10 {
            self.state = state::State::Yellow;
        } else {
            return Err(TrafficLightError::NotEnoughCarsPassed(cars_passed));
        }

        Ok(())
    }

    pub fn turn_green(&mut self) -> Result<(), TrafficLightError> {
        let &state::Red { cars_waiting } = self.state.try_as_red()?;

        if cars_waiting > 0 {
            self.state = state::State::Green(state::Green::default());
        } else {
            return Err(TrafficLightError::NotEnoughCarsWaiting(cars_waiting));
        }

        Ok(())
    }

    pub fn record_passed_car(&mut self) -> Result<(), TrafficLightError> {
        let state::Green { cars_passed } = self.state.try_as_green_mut()?;

        *cars_passed += 1;

        Ok(())
    }

    pub fn record_waiting_car(&mut self) -> Result<(), TrafficLightError> {
        let state::Red { cars_waiting } = self.state.try_as_red_mut()?;

        *cars_waiting += 1;

        Ok(())
    }
}

fn main() {
    let mut light = TrafficLight::default();

    light.record_waiting_car().unwrap();
    light.turn_green().unwrap();

    for _ in 0..=9 {
        light.record_passed_car().unwrap();
    }

    println!("{:?}", light.turn_yellow().unwrap_err());

    light.record_passed_car().unwrap();

    light.turn_yellow().unwrap();
    light.turn_red().unwrap();
}
