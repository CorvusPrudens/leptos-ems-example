use leptos::*;
use leptos_use::use_event_listener;

#[component]
pub fn App() -> impl IntoView {
    let (counter, set_counter) = create_signal(0);

    let node_ref = create_node_ref::<html::Button>();
    let _ = use_event_listener(node_ref, ev::click, move |_| {
        set_counter.update(|c| *c += 1);
    });

    let direct_counter = html::button().child("Direct counter");
    let _ = use_event_listener(direct_counter.clone(), ev::click, move |_| {
        set_counter.update(|c| *c += 2);
    });

    view! {
        <div style="display: flex; flex-direction: column; align-items: center;">
            <button node_ref=node_ref>
                "NodeRef counter"
            </button>

            {direct_counter}

            <div>
                {counter}
            </div>
        </div>
    }
}
