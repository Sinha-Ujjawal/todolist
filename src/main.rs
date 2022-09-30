use stylist::css;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{html, Component, Context, Event, Html};

fn render_items<C: Component<Message = Msg>, Msg: 'static>(
    items: &Vec<String>,
    ctx: &Context<C>,
    mk_event: fn(usize) -> Msg,
) -> Html {
    let items_as_li = items
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            html! {
                <li onclick={ctx.link().callback(move |_| mk_event(idx))}>{item}</li>
            }
        })
        .collect::<Vec<Html>>();
    html! {
        <div>
        <ol>
            {items_as_li}
        </ol>
        </div>
    }
}

fn render_textbox<C: Component<Message = Msg>, Msg: 'static>(
    placeholder: &'static str,
    value: String,
    ctx: &Context<C>,
    mk_event: fn(String) -> Msg,
) -> Html {
    let handle_event = move |e: Event| {
        let target = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        input.map(move |t| mk_event(t.value()))
    };
    let onchange = ctx.link().batch_callback(handle_event);
    let onkeyup = ctx
        .link()
        .batch_callback(move |e: KeyboardEvent| handle_event(e.into()));
    html! {
        <div>
            <input
                {placeholder}
                {value}
                {onchange}
                {onkeyup}
            />
        </div>
    }
}

fn render_button<C: Component<Message = Msg>, Msg: 'static>(
    button_text: &'static str,
    ctx: &Context<C>,
    mk_event: fn() -> Msg,
) -> Html {
    let onclick = ctx.link().callback(move |_| mk_event());
    html! {
        <button {onclick} class={css!("margin: 10px;")}>
            {button_text}
        </button>
    }
}

struct App {
    todos: Vec<String>,
    textbox_value: String,
}

enum Msg {
    SetTextBoxValue(String),
    AddTodo,
    RemoveTodo(usize),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            todos: Vec::new(),
            textbox_value: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddTodo => {
                if self.textbox_value == "" {
                    false
                } else {
                    self.todos.push(self.textbox_value.clone());
                    true
                }
            }
            Msg::SetTextBoxValue(value) => {
                self.textbox_value = value;
                true
            }
            Msg::RemoveTodo(idx) => {
                self.todos.remove(idx);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"TodoLists"}</h1>
                {render_textbox("Enter todo here", self.textbox_value.clone(), ctx, Msg::SetTextBoxValue)}
                {render_button("Add", ctx, || Msg::AddTodo)}
                {render_items(&self.todos, ctx, Msg::RemoveTodo)}
                <i>{"Click on todos to remove from the list"}</i>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
