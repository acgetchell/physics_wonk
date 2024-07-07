// Format using leptosfmt ./**/*.rs

use ev::SubmitEvent;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[allow(non_snake_case)]
fn App() -> impl IntoView {
    view! {
        <h2>"Query"</h2>
        <Query/>
    }
}

#[allow(non_snake_case)]
fn Query() -> impl IntoView {
    // import the type for <input>
    use leptos::html::Input;

    let (name, set_name) = create_signal("Query".to_string());

    let input_element: NodeRef<Input> = create_node_ref();

    // fires when form `submit` event is triggered
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            .expect("<input> to exist")
            .value();
        set_name.set(value);
    };

    view! {
        <form on:submit=on_submit>
            <input
                type="text"
                value=name
                node_ref=input_element
                style="width: 300px; height: 40px;"
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name: " {name}</p>
    }
}