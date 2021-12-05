use std::cmp::PartialEq;
use vertigo::{Driver, Computed, Value};

use crate::list;

#[derive(PartialEq)]
pub struct State {
    driver: Driver,

    pub message: Value<String>,
    pub strong: Value<bool>,
    pub list: Computed<list::State>,
}

impl State {
    pub fn new(driver: &Driver) -> Computed<State> {
        let state = State {
            driver: driver.clone(),
            message: driver.new_value("Hello world".to_string()),
            strong: driver.new_value(true),
            list: list::State::new(driver),
        };

        driver.new_computed_from(state)
    }
}
