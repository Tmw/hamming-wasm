#![recursion_limit="256"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::HtmlElement;

mod rendering_mode;
use rendering_mode::RenderingMode;

struct Model {
    link: ComponentLink<Self>,
    value: String,
    bits: Vec<bool>,
    input_ref: NodeRef,
    rendering_mode: RenderingMode,
}


enum Msg {
    Update(String),
    ToggleMode,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: "".into(),
            bits: vec![],
            input_ref: NodeRef::default(),
            rendering_mode: RenderingMode::Sequential,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.value = val;
                self.set_bits();
            },

            Msg::ToggleMode => {
                self.rendering_mode.toggle();
            }
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.set_focus()
        }
    }

    fn view(&self) -> Html {
        let handle_mode_click = self.link.callback(|_| Msg::ToggleMode);

        html! {
            <div class="grid">
                <header>
                    <h1>{ "Hamming Playground" }</h1>
                    <div class="toolbar">
                        <button onclick=handle_mode_click>{ &self.rendering_mode }</button>
                        <button>{ "Corrupt" }</button>
                        <button>{ "Repair" }</button>
                    </div>
                </header>

                <div class="input-box">
                    <h2>{ "Input" }</h2>
                    <textarea
                        ref=self.input_ref.clone()
                        placeholder="Insert your text here"
                        oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                    />
                </div>

                <div class="input-box">
                    <h2>{ "Output" }</h2>
                    <textarea
                        placeholder="As you type, your data will appear here.."
                        value=self.value
                        disabled=true
                    />
                </div>

                <div class="bit-container">
                    <div class="wrapper">
                        {for self.bits.iter().map(|bit| Model::view_bit(*bit)) }
                    </div>
                </div>
            </div>
        }
    }
}

impl Model {
    fn set_focus(&mut self) {
        if let Some(input) = self.input_ref.cast::<HtmlElement>() {
            input.focus().expect("unable to focus");
        }
    }

    fn set_bits(&mut self) {
        let bytes = self.value.as_bytes();
        let mut bits: Vec<bool> = Vec::new();

        for byte in bytes {
            for bit in 0..8 {
                bits.push(byte & 0b1 << bit > 0)
            }
        }

        self.bits = bits;
    }

    fn view_bit(bit: bool) -> Html {
        match bit {
            true =>  html!{ <div>{"1"}</div> },
            false => html!{ <div>{"0"}</div> },
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app(){
    App::<Model>::new().mount_to_body();
}
