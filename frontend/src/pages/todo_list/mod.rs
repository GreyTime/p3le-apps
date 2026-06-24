use yew::prelude::*;

#[component]
pub fn TodoList() -> Html {
    html! {
        <div style="padding: 2rem; font-family: sans-serif;">
            <h1>{"✅ Todo List"}</h1>
            <p>{"Hier kommt die Todo List hin."}</p>
        </div>
    }
}
