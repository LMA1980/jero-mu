use ratatui_core::{
    buffer::Buffer,
    layout::Rect,
    // text::Line,
    widgets::Widget
};
use ratatui::{
    // Frame,
    // layout::{Constraint, Layout, Rect},
    // style::{Style, Stylize},
    // text::{ Line, Span },
    widgets::{
        Block,
        Borders
        //BorderType,
    },
};
#[derive(Debug, Copy, Clone)]
pub struct About {
    _about_bin: AboutBin,
}

#[derive(Debug, Copy, Clone)]
pub struct AboutBin {
    _package_name: &'static str,
    _package_version: &'static str,
    _bin_name: &'static str,
}

impl About {
    pub fn new(value: Option<AboutBin>) -> Self {
        Self {
            _about_bin: value.unwrap_or_else(|| { 
                AboutBin {
                    _package_name: env!("CARGO_PKG_NAME"),
                    _package_version: env!("CARGO_PKG_VERSION"),
                    _bin_name: env!("CARGO_BIN_NAME"),
                }
            }),
        }
    }
}

impl Widget for &mut About {
    // Goal: render a block titled About, displaying the
    // package name, binary name and the package version.
    //
    // The widget will close if the Enter or Esc key is used.
    //
    // /--- About --------------------------------------\
    // | <package name> - <bin name> v<package version> |
    // \------------------------------------------------/
    //
    // For now, it will be rendered in the middle of the screen.
    // Maybe future version will enable to position the widget
    // where the integrator will want it.
    fn render(self, _area: Rect, _buf: &mut Buffer) {
        let pkg = env!("CARGO_PKG_NAME");
        let pkg_version = env!("CARGO_PKG_VERSION");
        let bin = env!("CARGO_BIN_NAME");

        let _about_line = format!("{} v{} :: {}", pkg, pkg_version, bin);
        let _about_block = Block::default()
            .title("About")
            .borders(Borders::ALL);
    }
}
