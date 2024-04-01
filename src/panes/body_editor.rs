use std::sync::Arc;

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
  prelude::*,
  widgets::{block::*, *},
};
use tui_textarea::TextArea;

use crate::{
  action::Action,
  pages::phone::{RequestBuilder, RequestPane},
  panes::Pane,
  state::{InputMode, OperationItem, State},
  tui::{EventResponse, Frame},
};

pub struct BodyEditor<'a> {
  focused: bool,
  focused_border_style: Style,
  operation_item: Arc<OperationItem>,
  input: TextArea<'a>,
  content_types: Vec<String>,
  content_type_index: usize,
}

impl<'a> BodyEditor<'a> {
  pub fn new(operation_item: Arc<OperationItem>, focused: bool, focused_border_style: Style) -> Self {
    Self {
      operation_item,
      focused,
      focused_border_style,
      input: TextArea::default(),
      content_types: vec![],
      content_type_index: 0,
    }
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
}

impl RequestPane for BodyEditor<'_> {
}

impl RequestBuilder for BodyEditor<'_> {
  fn reqeust(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
    if let Some(content_type) = self.content_types.get(self.content_type_index) {
      request.header("content-type", content_type).body(self.input.lines().join("\n"))
    } else {
      request
    }
  }
}

impl Pane for BodyEditor<'_> {
  fn init(&mut self, state: &State) -> Result<()> {
    self.input.set_cursor_line_style(Style::default());
    self.input.set_line_number_style(Style::default().dim());
    self.content_types = self
      .operation_item
      .operation
      .request_body
      .as_ref()
      .and_then(|request_body| request_body.resolve(&state.openapi_spec).ok())
      .map(|request| request.content.keys().cloned().collect::<Vec<_>>())
      .unwrap_or_default();
    Ok(())
  }

  fn focus(&mut self) -> Result<()> {
    self.focused = true;
    Ok(())
  }

  fn unfocus(&mut self) -> Result<()> {
    self.focused = false;
    Ok(())
  }

  fn height_constraint(&self) -> Constraint {
    Constraint::Fill(1)
  }

  fn handle_key_events(&mut self, key: KeyEvent, state: &mut State) -> Result<Option<EventResponse<Action>>> {
    match state.input_mode {
      InputMode::Normal => Ok(None),
      InputMode::Insert => {
        match key.code {
          KeyCode::Esc => Ok(Some(EventResponse::Stop(Action::Submit))),
          _ => {
            self.input.input(key);
            Ok(Some(EventResponse::Stop(Action::Noop)))
          },
        }
      },
    }
  }

  fn update(&mut self, action: Action, state: &mut State) -> Result<Option<Action>> {
    if self.content_types.is_empty() {
      return Ok(None);
    }
    match action {
      Action::Update => {},
      Action::Submit if state.input_mode == InputMode::Normal => {
        state.input_mode = InputMode::Insert;
      },
      Action::Submit if state.input_mode == InputMode::Insert => {
        state.input_mode = InputMode::Normal;
      },
      Action::Tab(index) if index < self.content_types.len().try_into()? => {
        self.content_type_index = index.try_into()?;
      },
      Action::TabNext => {
        let next_tab_index = self.content_type_index + 1;
        self.content_type_index =
          if next_tab_index < self.content_types.len() { next_tab_index } else { self.content_type_index };
      },
      Action::TabPrev => {
        self.content_type_index =
          if self.content_type_index > 0 { self.content_type_index - 1 } else { self.content_type_index };
      },
      _ => {},
    }
    Ok(None)
  }

  fn draw(&mut self, frame: &mut Frame<'_>, area: Rect, state: &State) -> Result<()> {
    let margin_h1_v1: Margin = Margin { horizontal: 1, vertical: 1 };
    let inner = area.inner(&margin_h1_v1);

    if self.focused && state.input_mode == InputMode::Insert {
      self.input.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    } else {
      self.input.set_cursor_style(Style::default());
    }

    if !self.content_types.is_empty() {
      frame.render_widget(self.input.widget(), inner);
    }

    let content_types = if !self.content_types.is_empty() {
      let ctype = self.content_types[self.content_type_index].clone();
      let ctype_progress = if self.content_types.len() > 1 {
        format!("[{}/{}]", self.content_type_index + 1, self.content_types.len())
      } else {
        String::default()
      };
      format!(": {ctype} {ctype_progress}")
    } else {
      String::from(": Not Applicable")
    };

    frame.render_widget(
      Block::default()
        .title(format!("Body{content_types}"))
        .borders(Borders::ALL)
        .border_style(self.border_style())
        .border_type(self.border_type()),
      area,
    );

    Ok(())
  }
}
