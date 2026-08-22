// Element for rendering a panel background

use calloop::LoopHandle;
use lingmo::iced::core::Shadow;
use lingmo::iced::{Color, Length, id};
use lingmo::widget::space;
use lingmo::{Theme, theme};

use crate::iced::{Element, IcedElement, Program};
use crate::xdg_shell_wrapper::shared_state::GlobalState;

pub type BackgroundElement = IcedElement<Background>;

pub fn background_element(
    id: id::Id,
    logical_width: i32,
    logical_height: i32,
    radius: [f32; 4],
    loop_handle: LoopHandle<'static, GlobalState>,
    theme: Theme,
    panel_id: usize,
    logical_pos: [f32; 2],
    color: [f32; 4],
    scale: f64,
) -> BackgroundElement {
    IcedElement::new(
        Background {
            id,
            logical_width,
            logical_height,
            radius,
            logical_pos: (logical_pos[0].round() as i32, logical_pos[1].round() as i32),
            color,
            scale,
        },
        (logical_width, logical_height),
        loop_handle,
        theme,
        panel_id,
        false,
    )
}

pub struct Background {
    pub id: id::Id,
    pub logical_width: i32,
    pub logical_height: i32,
    pub radius: [f32; 4],
    pub logical_pos: (i32, i32),
    pub color: [f32; 4],
    pub scale: f64,
}

impl Program for Background {
    type Message = ();

    fn view(&self) -> Element<'_, ()> {
        let width = self.logical_width as f32;
        let height = self.logical_height as f32;
        let radius_arr: [f32; 4] = self.radius;

        let color = self.color;
        Element::from(
            lingmo::widget::container(space::horizontal().width(Length::Fixed(width)))
                .width(Length::Fixed(width))
                .height(Length::Fixed(height))
                .class(theme::Container::custom(move |theme| {
                    let cosmic = theme.cosmic();

                    lingmo::widget::container::Style {
                        text_color: Some(cosmic.background(theme.transparent).on.into()),
                        background: Some(Color::from(color).into()),
                        border: lingmo::iced::Border {
                            radius: radius_arr.into(),
                            width: 0.,
                            color: cosmic.background(theme.transparent).divider.into(),
                        },
                        shadow: Shadow::default(),
                        snap: true,
                        icon_color: Some(cosmic.background(theme.transparent).on.into()),
                    }
                })),
        )
    }
}
