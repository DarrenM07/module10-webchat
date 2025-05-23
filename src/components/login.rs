use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="min-h-screen w-screen bg-gradient-to-tr from-gray-900 via-gray-800 to-gray-700 flex items-center justify-center px-4">
            <div class="bg-white rounded-xl shadow-2xl p-8 w-full max-w-md">
                <h2 class="text-2xl font-bold text-center text-indigo-600 mb-6">{"Welcome to YewChat 👋"}</h2>
                <form class="flex">
                    <input
                        {oninput}
                        placeholder="Enter username"
                        class="flex-grow py-3 px-4 rounded-l-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-400 transition"
                    />
                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len() < 1}
                            class="px-6 py-3 rounded-r-lg bg-gradient-to-br from-indigo-600 to-purple-600 text-white font-semibold uppercase hover:opacity-90 transition disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {"Go Chatting!"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}
