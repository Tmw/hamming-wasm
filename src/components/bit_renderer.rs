use yew::prelude::*;
use yew::Properties;

use crate::types::{Bit, RenderingMode};

pub struct BitRenderer {
    link: ComponentLink<Self>,
    props: BitRendererProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct BitRendererProps {
    pub bits: Vec<Bit>,
    pub rendering_mode: RenderingMode,
}

impl Component for BitRenderer {
    type Message = ();
    type Properties = BitRendererProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props: props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="bit-container">
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
    fn render_sequential(&self) -> Html {
        html! {
            <div class="wrapper sequential">
                {for self.props.bits.iter().map(|bit| BitRenderer::render_bit(bit)) }
            </div>
        }
    }

    fn render_blocks(&self) -> Html {
        html! {
            <div class="wrapper blocks">
                {for self.props.bits.chunks(16).map(|block| BitRenderer::render_block(block)) }
            </div>
        }
    }

    fn render_block(block: &[Bit]) -> Html {
        html! {
            <div class="block">
                {for block.iter().map(|bit| BitRenderer::render_bit(bit))}
            </div>
        }
    }

    fn render_bit(bit: &Bit) -> Html {
        let (class, val) = match (bit.is_high, bit.is_parity()) {
            (true, true)   => ("bit active parity", "1"),
            (true, false)  => ("bit active", "1"),
            (false, true)  => ("bit inactive parity", "0"),
            (false, false) => ("bit inactive", "0"),
        };

        html! {
            <div class={class}>
                <span>{val}</span>
            </div>
        }
    }
}
