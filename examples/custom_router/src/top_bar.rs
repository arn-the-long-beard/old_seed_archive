use seed::{prelude::*, Style as Css, *};
use std::borrow::Cow;

use crate::theme::Theme;
use web_sys::HtmlElement;

/// The top bar is the component used for navigation, user actions and title
/// located on the top of the applicatiob
pub struct TopBar<Ms: 'static> {
    title: Option<Cow<'static, str>>,
    style: Theme,
    outline: bool,
    block: bool,
    attrs: Attrs,
    user_logged_in: bool,
    disabled: bool,
    content: Vec<Node<Ms>>,
    el_ref: ElRef<HtmlElement>,
    css: Css,
}

impl<Ms: 'static> TopBar<Ms> {
    pub fn new(title: impl Into<Cow<'static, str>>) -> Self {
        Self::default().title(title)
    }

    pub fn title(mut self, title: impl Into<Cow<'static, str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    // --- style ---

    pub const fn style(mut self, style: Theme) -> Self {
        self.style = style;
        self
    }

    pub const fn daily(self) -> Self {
        self.style(Theme::Light)
    }

    pub const fn nightly(self) -> Self {
        self.style(Theme::Dark)
    }

    pub const fn set_user_login_state(mut self, is_user_logged_in: bool) -> Self {
        self.user_logged_in = is_user_logged_in;
        self
    }
    // --- // ---

    pub const fn outline(mut self) -> Self {
        self.outline = true;
        self
    }

    // --- // ---

    pub const fn block(mut self) -> Self {
        self.block = true;
        self
    }

    // --- // ---

    pub fn add_attrs(mut self, attrs: Attrs) -> Self {
        self.attrs.merge(attrs);
        self
    }

    pub fn add_css(mut self, css: Css) -> Self {
        self.css.merge(css);
        self
    }

    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn content(mut self, content: impl IntoNodes<Ms>) -> Self {
        self.content = content.into_nodes();
        self
    }

    pub fn el_ref(mut self, el_ref: &ElRef<HtmlElement>) -> Self {
        self.el_ref = el_ref.clone();
        self
    }

    fn view(mut self) -> Node<Ms> {
        let tag = Tag::Div;

        let content = div![self.title.take().map(Node::new_text),];

        // if content.is_some() {
        //     let mut node = content.unwrap();
        //     node.add_style(St::TextAlign, "center");
        // }

        let attrs = {
            let mut attrs = attrs! {};

            if self.disabled {
                attrs.add(At::from("aria-disabled"), true);
                attrs.add(At::TabIndex, -1);
            }
            attrs
        };

        let css = {
            let mut css = style! {
                St::TextDecoration => "none",
                St::Height=>"60px"
            };

            let color = match self.style {
                Theme::Dark => "lightgrey",
                Theme::Light => "darkblue",
            };

            let background = match self.style {
                Theme::Dark => "blue",
                Theme::Light => "lightskyblue",
            };

            let font_color = match self.style {
                Theme::Dark => "white",
                Theme::Light => "black",
            };
            // if self.outline {
            if self.user_logged_in {
                css.merge(style! {
                    St::Color => color,

                    St::BackgroundColor => "transparent",
                    St::Border => format!("{} {} {}", px(5), "solid", color),
                });
            } else {
                css.merge(style! { St::Color => font_color,
                // St::Display => "flex",
                // St::AlignItems => "center",
                // St::JustifyContent=> "center" ,
                St::BackgroundColor => background });
            };

            if self.block {
                css.merge(style! {St::Display => "block"});
            }

            if self.disabled {
                css.merge(style! {St::Opacity => 0.5});
            } else {
                css.merge(style! {St::Cursor => "pointer"});
            }

            css
        };

        let mut top_bar = custom![
            tag,
            el_ref(&self.el_ref),
            css,
            self.css,
            attrs,
            self.attrs,
            content,
            self.content,
        ];

        top_bar
    }
}
impl<Ms> Default for TopBar<Ms> {
    fn default() -> Self {
        Self {
            title: None,
            style: Theme::default(),
            outline: false,
            block: false,
            attrs: Attrs::empty(),
            user_logged_in: false,
            disabled: false,
            content: Vec::new(),
            el_ref: ElRef::default(),
            css: Css::empty(),
        }
    }
}

impl<Ms> UpdateEl<Ms> for TopBar<Ms> {
    fn update_el(self, el: &mut El<Ms>) {
        self.view().update_el(el)
    }
}
