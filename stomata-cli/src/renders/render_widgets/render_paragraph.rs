//! Paragraph widget rendering utilities
//!
//! Provides functions for creating styled paragraph widgets with borders
//! and titles. Used for displaying text content like system information,
//! help text, and status messages.

use ratatui::{
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

/// Creates a styled paragraph widget with a border and title.
///
/// Wraps text content in a bordered block with a title header, providing
/// a consistent appearance for text displays throughout the application.
///
/// # Arguments
///
/// * `text` - The text content to display in the paragraph
/// * `title` - The title text to show in the border
///
/// # Returns
///
/// A configured `Paragraph` widget ready for rendering
///
/// # Examples
///
/// ```ignore
/// use crate::renders::render_widgets::render_paragraph::paragraph_widget;
///
/// let text = "OS: Linux\nKernel: 5.15.0\nHostname: server";
/// let widget = paragraph_widget(text, "System Info");
/// frame.render_widget(widget, area);
/// ```
///
/// # Styling
///
/// - Border: All sides (top, bottom, left, right)
/// - Title: Positioned at the top-left of the border
/// - Text: Rendered as-is with newlines preserved
///
/// # Notes
///
/// The paragraph can be further customized after creation by chaining
/// additional methods like `.alignment()`, `.wrap()`, or `.style()`.
pub fn paragraph_widget<T: Into<Text<'static>>, U: Into<Line<'static>>>(
    text: T,
    title: U,
) -> Paragraph<'static> {
    Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(title))
}
