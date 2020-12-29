#![recursion_limit="256"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: String,
    bits: Vec<bool>,
    input_ref: NodeRef
}

enum Msg {
    Update(String),
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.value = val;
                self.set_bits();
            }
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {

        use web_sys::HtmlElement;

        if first_render {
            if let Some(input) = self.input_ref.cast::<HtmlElement>() {
                input.focus().expect("unable to focus");
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="grid">
                <header>
                    <h1>{ "Hamming Playground" }</h1>
                    <div class="toolbar">
                        <button>{ "Sequential" }</button>
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
