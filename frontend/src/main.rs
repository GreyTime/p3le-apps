use yew::prelude::*;
use yew_router::prelude::*;

mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/habit-tracker")]
    HabitTracker,
    #[at("/todo-list")]
    TodoList,
    #[at("/hobby-chooser")]
    HobbyChooser,
    #[at("/cable-db")]
    CableDb,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <pages::home::Home /> },
        Route::HabitTracker => html! { <pages::habit_tracker::HabitTracker /> },
        Route::TodoList => html! { <pages::todo_list::TodoList /> },
        Route::HobbyChooser => html! { <pages::hobby_chooser::HobbyChooser /> },
        Route::CableDb => html! { <pages::cable_db::CableDb /> },
    }
}

#[component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <div>
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
