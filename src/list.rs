use vertigo::{Computed, Driver, Value, VDomElement, html, Css, css};

#[derive(PartialEq)]
pub struct State {
    items: Value<Vec<String>>,
    new_item: Value<String>,
    count: Computed<usize>,
}

impl State {
    pub fn new(driver: &Driver) -> Computed<State> {
        let items = driver.new_value(vec![
            "Item 1".to_string(),
            "Item 2".to_string(),
        ]);

        let count = {
            let items = items.clone();
            driver.from(move || items.get_value().len())
        };

        let state = State {
            items,
            new_item: driver.new_value("".to_string()),
            count,
        };

        driver.new_computed_from(state)
    }

    pub fn add(&self) -> impl Fn() {
        let items = self.items.clone();
        let new_item = self.new_item.clone();
        move || {
            let mut items_vec = items.get_value().to_vec();
            items_vec.push(new_item.get_value().to_string());
            items.set_value(items_vec);
            new_item.set_value("".to_string());
        }
    }

    pub fn change(&self) -> impl Fn(String) {
        let new_item = self.new_item.clone();
        move |value: String| {
            new_item.set_value(value)
        }
    }
}

fn alternate_rows(excl: bool) -> Css {
    let bg_color = if excl { "yellow" } else { "inherit" };

    css!("
        color: black;
        background: { bg_color };

        :nth-child(odd) {
            color: blue;
        };
    ")
}

pub fn render(state: &Computed<State>) -> VDomElement {
    let state = state.get_value();

    let items = state.items.get_value();

    let elements = items.iter()
        .map(|item| {
            let excl = item.ends_with('!');
            html! {
                <li css={alternate_rows(excl)}>{item}</li>
            }
        });


    let new_value = &*state.new_item.get_value();

    let count = *state.count.get_value();

    html! {
        <div>
            <p>"My list (" { count } ")"</p>
            <ol>
                { ..elements }
            </ol>
            <input value={new_value} on_input={state.change()} />
            <button on_click={state.add()}>"Add"</button>
        </div>
    }
}
