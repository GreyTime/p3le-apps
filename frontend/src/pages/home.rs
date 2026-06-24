use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[component]
pub fn Home() -> Html {
    html! {
        <div style="padding: 2rem; font-family: sans-serif;">
            <h1>{"P3LE Apps"}</h1>
            <p>{"Wähle eine App:"}</p>
            <ul style="list-style: none; padding: 0; line-height: 2;">
                <li><Link<Route> to={Route::HabitTracker}>{"🏃 Habit Tracker"}</Link<Route>></li>
                <li><Link<Route> to={Route::TodoList}>{"✅ Todo List"}</Link<Route>></li>
                <li><Link<Route> to={Route::HobbyChooser}>{"🎲 Hobby Chooser"}</Link<Route>></li>
                <li><Link<Route> to={Route::CableDb}>{"🔌 Cable Database"}</Link<Route>></li>
            </ul>
        </div>
    }
}
