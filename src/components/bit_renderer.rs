use yew::prelude::*;
use yew::Properties;
use web_sys::{Element, ScrollToOptions};

use crate::types::{Bit, RenderingMode};

pub struct BitRenderer {
    link: ComponentLink<Self>,
    props: BitRendererProps,
    bit_container_ref: NodeRef,
    should_scroll_to_bottom: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct BitRendererProps {
    pub bits: Vec<Bit>,
    pub rendering_mode: RenderingMode,
    pub on_flip: Callback<usize>,
}

pub enum BitRendererMessage {
    Flip(usize),
}

impl Component for BitRenderer {
    type Message = BitRendererMessage;
    type Properties = BitRendererProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: props,
            should_scroll_to_bottom: false,
            bit_container_ref: NodeRef::default(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.should_scroll_to_bottom = props.bits.len() > self.props.bits.len();
        self.props = props;

        true
    }

    fn rendered(&mut self, _first_render: bool) {
        if self.should_scroll_to_bottom {
            self.scroll_to_bottom()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BitRendererMessage::Flip(index) => {
                self.props.on_flip.emit(index);
            }
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="bit-container" ref=self.bit_container_ref.clone()>
            {
                match &self.props.rendering_mode {
                    RenderingMode::Sequential => self.render_sequential(),
                    RenderingMode::Blocks => self.render_blocks(),
                }
            }
            </div>
        }
    }
}

impl BitRenderer {
    fn scroll_to_bottom(&self) {
        if let Some(wrapper) = self.bit_container_ref.cast::<Element>() {
            let mut options = ScrollToOptions::new();
            options.top(wrapper.scroll_height() as f64);
            wrapper.scroll_with_scroll_to_options(&options)
        }
    }

    fn render_sequential(&self) -> Html {
        html! {
            <div class="wrapper sequential">
                {for self.props.bits.iter().map(|bit| self.render_bit(bit)) }
            </div>
        }
    }

    fn render_blocks(&self) -> Html {
        html! {
            <div class="wrapper blocks">
                {for self.props.bits.chunks(16).map(|block| self.render_block(block)) }
            </div>
        }
    }

    fn render_block(&self, block: &[Bit]) -> Html {
        html! {
            <div class="block">
                {for block.iter().map(|bit| self.render_bit(bit))}
            </div>
        }
    }

    fn render_bit(&self, bit: &Bit) -> Html {
        let bit_index = bit.index;
        let clicked = self
            .link
            .callback(move |_| BitRendererMessage::Flip(bit_index));

        let (class, val) = match (bit.is_high, bit.is_parity(), bit.is_flipped) {
            (true, _, true) => ("bit flipped", "1"),
            (false, _, true) => ("bit flipped", "0"),

            (true, true, false) => ("bit active parity", "1"),
            (true, false, false) => ("bit active", "1"),
            (false, true, false) => ("bit inactive parity", "0"),
            (false, false, false) => ("bit inactive", "0"),
        };

        html! {
            <div class={class} onclick=&clicked>
                <span>{val}</span>
            </div>
        }
    }
}
