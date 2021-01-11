use hamming::blocks::Blocks;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::components::BitRenderer;
use crate::types::{Bit, RenderingMode};

pub struct App {
    link: ComponentLink<Self>,
    input_string: String,
    output_string: String,
    bits: Vec<Bit>,
    input_ref: NodeRef,
    rendering_mode: RenderingMode,
}

pub enum Msg {
    Update(String),
    ToggleMode,
    FlipBit(usize),
    FlipRandom,
    Repair,
    Reset,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input_string: "".into(),
            output_string: "".into(),
            bits: vec![],
            input_ref: NodeRef::default(),
            rendering_mode: RenderingMode::Blocks,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.input_string = val.clone();
                self.output_string = val.clone();
                self.hamming_encode();
                self.hamming_decode();
            }

            Msg::ToggleMode => {
                self.rendering_mode.toggle();
            }

            Msg::FlipBit(index) => {
                if let Some(bit) = self.bits.get_mut(index) {
                    bit.flip();
                    self.hamming_decode();
                }
            }

            Msg::FlipRandom => {
                self.corrupt();
            }

            Msg::Repair => {
                self.repair();
            }

            Msg::Reset => {
                self.reset();
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
        let handle_corrupt_click = self.link.callback(|_| Msg::FlipRandom);
        let handle_repair_click = self.link.callback(|_| Msg::Repair);
        let handle_reset_click = self.link.callback(|_| Msg::Reset);
        let on_flip = self.link.callback(|index| Msg::FlipBit(index));

        html! {
            <div class="grid">
                <header>
                    <h1>{ "Hamming Playground" }</h1>
                    <div class="toolbar">
                        <button onclick=handle_mode_click>{ &self.rendering_mode.opposite() }</button>
                        <button onclick=handle_corrupt_click>{ "Corrupt" }</button>
                        <button onclick=handle_repair_click>{ "Repair" }</button>
                        <button onclick=handle_reset_click>{ "Reset" }</button>
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

                <div class="footer">
                    {self.render_footer_links()}
                </div>

                <BitRenderer
                    bits=&self.bits
                    rendering_mode=&self.rendering_mode
                    on_flip=&on_flip
                />
            </div>
        }
    }
}

type Bytes = Vec<u8>;
type Bits = Vec<Bit>;

impl App {
    fn set_focus(&mut self) {
        if let Some(input) = self.input_ref.cast::<HtmlElement>() {
            input.focus().expect("unable to focus");
        }
    }

    fn corrupt(&mut self) {
        self.reset();

        for block in self.bits.chunks_mut(16) {
            let should_flip = rand::random::<bool>();
            if should_flip {
                let flip_index = rand::random::<usize>() % 15;
                if let Some(bit) = block.get_mut(flip_index) {
                    bit.flip();
                }
            }
        }

        self.hamming_decode();
    }

    fn repair(&mut self) {
        let bytes = App::bits_to_bytes(&self.bits);
        let mut blocks = Blocks::new(&bytes, true);
        blocks.repair();

        self.bits = App::bytes_to_bits(&blocks.to_byte_vec());
        self.hamming_decode();
    }

    fn reset(&mut self) {
        self.hamming_encode();
        self.hamming_decode();
    }

    fn hamming_encode(&mut self) {
        let mut blocks = Blocks::new(&self.input_string.as_bytes(), false);
        blocks.prepare();

        self.bits = App::bytes_to_bits(&blocks.to_byte_vec());
    }

    fn hamming_decode(&mut self) {
        let bytes = App::bits_to_bytes(&self.bits);
        let blocks = Blocks::new(&bytes, true);
        self.output_string = blocks.to_string();
    }

    fn bytes_to_bits(bytes: &Bytes) -> Bits {
        let mut bits: Bits = Vec::new();
        let mut index_cycler = (0..16).cycle();

        for (byte_idx, byte) in bytes.iter().enumerate() {
            for bit in 0..8 {
                let bit = Bit {
                    index: (byte_idx * 8) + bit,
                    is_high: byte & 0b1 << 7 - bit > 0,
                    is_flipped: false,
                    index_in_block: index_cycler.next().unwrap(),
                };

                bits.push(bit)
            }
        }

        bits
    }

    fn bits_to_bytes(bits: &Bits) -> Bytes {
        let mut bytes: Bytes = Vec::new();

        for chunk in bits.chunks(8) {
            let byte = chunk
                .iter()
                .enumerate()
                .fold(0_u8, |acc, (idx, bit)| match bit.is_high {
                    true => acc | 0b1 << 7 - idx,
                    false => acc | 0b0 << 7 - idx,
                });

            bytes.push(byte);
        }

        bytes
    }

    fn render_footer_links(&self) -> Html {
        let help_url = crate::config::help_url();
        let repo_url = crate::config::repo_url();

        html! {
            <>
                <a href={help_url}>{"Help"}</a>
                <a href={repo_url}>{"Source"}</a>
            </>
        }

    }
}
