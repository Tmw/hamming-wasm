#![recursion_limit="256"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::HtmlElement;

use hamming::blocks::Blocks;
mod types;
mod components;
use components::BitRenderer;
use types::RenderingMode;

struct Model {
    link: ComponentLink<Self>,
    input_string: String,
    output_string: String,
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
            input_string: "".into(),
            output_string: "".into(),
            bits: vec![],
            input_ref: NodeRef::default(),
            rendering_mode: RenderingMode::Sequential,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.input_string = val.clone();
                self.output_string = val.clone();
                self.hamming_encode();
                self.hamming_decode();
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
                        value=self.output_string
                        disabled=true
                    />
                </div>

                <BitRenderer
                    bits=&self.bits
                    rendering_mode=&self.rendering_mode
                />
            </div>
        }
    }
}

type Bytes = Vec<u8>;
type Bits = Vec<bool>;

impl Model {
    fn set_focus(&mut self) {
        if let Some(input) = self.input_ref.cast::<HtmlElement>() {
            input.focus().expect("unable to focus");
        }
    }

    fn hamming_encode(&mut self) {
        let blocks = Blocks::new(&self.input_string.as_bytes(), false);
        self.bits = Model::bytes_to_bits(&blocks.to_byte_vec());
    }

    fn hamming_decode(&mut self) {
        let bytes = Model::bits_to_bytes(&self.bits);
        let mut blocks = Blocks::new(&bytes, true);
        blocks.repair();
        self.output_string = blocks.to_string();
    }

    fn bytes_to_bits(bytes: &Bytes) -> Bits {
        let mut bits: Bits = Vec::new();

        for byte in bytes {
            for bit in 0..8 {
                bits.push(byte & 0b1 << bit > 0)
            }
        }

        bits
    }

    fn bits_to_bytes(bits: &Bits) -> Bytes {
        let mut bytes: Bytes = Vec::new();

        for chunk in bits.chunks(8) {
            let byte = chunk.iter().enumerate().fold(0_u8, |acc, (idx, b)| {
                match b {
                    true => acc | 0b1 << idx,
                    false => acc | 0b0 << idx
                }
            });

            bytes.push(byte);
        }

        bytes
    }
}

#[wasm_bindgen(start)]
pub fn run_app(){
    App::<Model>::new().mount_to_body();
}
