use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                ),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => return false,
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen font-sans">
                <div class="flex-none w-64 h-screen bg-gradient-to-b from-indigo-500 to-purple-500 shadow-xl text-white">
                    <div class="text-2xl p-4 font-extrabold tracking-wide">{"👥 Users"}</div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="flex items-center m-3 bg-white/10 hover:bg-white/20 transition rounded-lg p-3 backdrop-blur-md shadow-lg cursor-pointer">
                                    <img class="w-12 h-12 rounded-full border-2 border-white hover:scale-105 transition duration-150" src={u.avatar.clone()} alt="avatar"/>
                                    <div class="ml-4">
                                        <div class="font-bold">{u.name.clone()}</div>
                                        <div class="text-sm text-white/70 italic">{"Ready to chat!"}</div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="grow h-screen flex flex-col bg-gradient-to-br from-indigo-50 to-white">
                    <div class="w-full h-16 bg-white shadow flex items-center px-6 border-b border-indigo-200">
                        <div class="text-2xl font-semibold text-indigo-700">{"💬 Chat Room"}</div>
                    </div>
                    <div class="flex-1 overflow-y-auto px-6 py-4 space-y-5">
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                html!{
                                    <div class="flex items-start space-x-3">
                                        <img class="w-10 h-10 rounded-full border-2 border-indigo-400 shadow-sm" src={user.avatar.clone()} alt="avatar"/>
                                        <div class="bg-white rounded-xl px-4 py-3 shadow-md max-w-md">
                                            <div class="font-semibold text-indigo-700">{m.from.clone()}</div>
                                            <div class="text-sm text-gray-700 mt-1">
                                                if m.message.ends_with(".gif") {
                                                    <img class="mt-2 rounded-md" src={m.message.clone()} />
                                                } else {
                                                    {m.message.clone()}
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="w-full h-16 px-6 py-2 bg-white border-t border-indigo-200 flex items-center space-x-3">
                        <input
                            ref={self.chat_input.clone()}
                            type="text"
                            placeholder="Type something cool..."
                            class="flex-1 py-3 px-5 rounded-full border border-gray-300 shadow focus:outline-none focus:ring-2 focus:ring-indigo-400 text-gray-800"
                            name="message"
                            required=true
                        />
                        <button onclick={submit} class="bg-gradient-to-br from-indigo-600 to-purple-600 hover:opacity-90 transition w-12 h-12 rounded-full flex justify-center items-center shadow-lg">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white w-6 h-6">
                                <path d="M0 0h24v24H0z" fill="none"></path>
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
