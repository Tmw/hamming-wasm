use yew::prelude::*;
use yew::Properties;

use crate::types::RenderingMode;

pub struct BitRenderer {
    link: ComponentLink<Self>,
    props: BitRendererProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct BitRendererProps {
    pub bits: Vec<bool>,
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
                {for self.props.bits.iter().map(|bit| BitRenderer::render_bit(*bit)) }
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

    fn render_bit(bit: bool) -> Html {
        // TODO: We'll move the "active" vs "inactive" and wether it's a
        // corrupted bit and wether it's a parity bit into a separate struct later.
        // now we could derive these values.

        let (class, val) = match bit {
            true => ("bit active", "1"),
            false => ("bit inactive", "0"),
        };

        html! {
            <div class={class}>
                <span>{val}</span>
            </div>
        }
    }

    fn render_block(block: &[bool]) -> Html {
        html! {
            <div class="block">
                {for block.iter().map(|bit| BitRenderer::render_bit(*bit))}
            </div>
        }
    }
}
