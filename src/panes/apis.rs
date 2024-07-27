use color_eyre::eyre::Result;
use ratatui::{
  prelude::*,
  widgets::{block::*, *},
};

use crate::{
  action::Action,
  panes::Pane,
  state::{OperationItemType, State},
  tui::Frame,
};

pub struct ApisPane {
  focused: bool,
  focused_border_style: Style,
  current_operation_index: usize,
}

impl ApisPane {
  pub fn new(focused: bool, focused_border_style: Style) -> Self {
    Self { focused, focused_border_style, current_operation_index: 0 }
  }

  fn border_style(&self) -> Style {
    match self.focused {
      true => self.focused_border_style,
      false => Style::default(),
    }
  }

  fn border_type(&self) -> BorderType {
    match self.focused {
      true => BorderType::Thick,
      false => BorderType::Plain,
    }
  }

  fn method_color(method: &str) -> Color {
    match method {
      "GET" => Color::LightCyan,
      "POST" => Color::LightBlue,
      "PUT" => Color::LightYellow,
      "DELETE" => Color::LightRed,
      _ => Color::Gray,
    }
  }
}

impl Pane for ApisPane {
  fn height_constraint(&self) -> Constraint {
    match self.focused {
      true => Constraint::Fill(3),
      false => Constraint::Fill(3),
    }
  }

  fn update(&mut self, action: Action, state: &mut State) -> Result<Option<Action>> {
    match action {
      Action::Down => {
        let operations_len = state.operations_len();
        if operations_len > 0 {
          self.current_operation_index = self.current_operation_index.saturating_add(1) % operations_len;
        }
        state.active_operation_index = self.current_operation_index;
        return Ok(Some(Action::Update));
      },
      Action::Up => {
        let operations_len = state.operations_len();
        if operations_len > 0 {
          self.current_operation_index =
            self.current_operation_index.saturating_add(operations_len.saturating_sub(1)) % operations_len;
        }
        state.active_operation_index = self.current_operation_index;
        return Ok(Some(Action::Update));
      },
      Action::Focus => {
        self.focused = true;
        static STATUS_LINE: &str = "[j,k → movement] [ENTER → request]";
        return Ok(Some(Action::TimedStatusLine(STATUS_LINE.into(), 3)));
      },
      Action::UnFocus => {
        self.focused = false;
      },
      Action::Submit => {},
      Action::Update => {
        self.current_operation_index = state.active_operation_index;
      },
      _ => {},
    }

    Ok(None)
  }

  fn draw(&mut self, frame: &mut Frame<'_>, area: Rect, state: &State) -> Result<()> {
    let items = state.openapi_operations.iter().filter_map(|operation_item| {
      if let Some(active_tag) = &state.active_tag_name {
        if !operation_item.has_tag(active_tag) {
          return None;
        }
      }
      if !operation_item.path.contains(state.active_filter.as_str()) {
        return None;
      }
      Some(Line::from(vec![
        Span::styled(
          format!(
            " {:7}",
            match operation_item.r#type {
              OperationItemType::Path => operation_item.method.as_str(),
              OperationItemType::Webhook => "EVENT",
            }
          ),
          match operation_item.r#type {
            OperationItemType::Path => Self::method_color(operation_item.method.as_str()),
            OperationItemType::Webhook => Color::LightMagenta,
          },
        ),
        Span::styled(format!(" {:7}", operation_item.path), Color::White),
      ]))
    });

    let list = List::new(items)
      .block(Block::default().borders(Borders::ALL))
      .highlight_symbol(symbols::scrollbar::HORIZONTAL.end)
      .highlight_spacing(HighlightSpacing::Always)
      .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    let mut list_state = ListState::default().with_selected(Some(self.current_operation_index));

    frame.render_stateful_widget(list, area, &mut list_state);
    let active_tag = format!("[{}]", state.active_tag_name.clone().unwrap_or(String::from("ALL")));
    frame.render_widget(
      Block::default()
        .title("APIs")
        .borders(Borders::ALL)
        .border_style(self.border_style())
        .border_type(self.border_type())
        .title_bottom(
          Line::from(format!(
            "{} of {}",
            self.current_operation_index.saturating_add(1).min(state.operations_len()),
            state.operations_len()
          ))
          .right_aligned(),
        )
        .title(Line::styled(active_tag, Style::default().add_modifier(Modifier::ITALIC)).right_aligned()),
      area,
    );
    Ok(())
  }
}
