//! Module contains the home screen for the TUI that shows the list of commands and their [Status]
//!
//!

use color_eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{
    prelude::*,
    style::palette::tailwind::{BLUE, GREEN, RED, SLATE, YELLOW},
    widgets::*,
};
use strum::IntoEnumIterator as _;
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{
    cli::{Command, CommandVariants},
    tui::{action::Action, config::Config},
};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const INFEASIBLE_ROW_BG_COLOR: Color = YELLOW.c900;
const NOT_IMPLEMENTED_ROW_BG_COLOR: Color = RED.c900;
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    list: CommandList,
}

struct CommandList {
    items: Vec<CommandItem>,
    state: ListState,
}

impl Default for CommandList {
    fn default() -> Self {
        let mut items: Vec<CommandItem> = Command::iter()
            .map(|el| CommandItem {
                name: el.to_string(),
                short_help: "".into(),
                status: Status::NotImplemented,
            })
            .collect();

        // mark the feasible / infeasible commands
        items[CommandVariants::List as usize].status = Status::Feasible;

        // sort by status
        items.sort_by(|lhs, rhs| (rhs.status as usize).cmp(&(lhs.status as usize)));

        Self { items, state: Default::default() }
    }
}

impl FromIterator<(Status, &'static str, &'static str)> for CommandList {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(status, name, short_help)| {
                CommandItem::new(name.into(), short_help.into(), status)
            })
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    /// Command is not implemented
    NotImplemented,

    /// Infeasible for a reason, e.g. not synced yet.
    Infeasible,

    /// Command is ready to be called
    Feasible,
}

#[derive(Debug)]
struct CommandItem {
    name: String,
    short_help: String,
    status: Status,
}

impl CommandItem {
    pub fn new(name: String, short_help: String, status: Status) -> Self {
        Self { name, short_help, status }
    }
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }

    fn select_none(&mut self) {
        self.list.state.select(None);
    }

    fn select_next(&mut self) {
        self.list.state.select_next();
    }

    fn select_previous(&mut self) {
        self.list.state.select_previous();
    }

    fn select_first(&mut self) {
        self.list.state.select_first();
    }

    fn select_last(&mut self) {
        self.list.state.select_last();
    }

    fn run_selected_cmd(&mut self) {
        if let Some(idx) = self.list.state.selected() {
            let item = &self.list.items[idx];
            if let Some(sender) = &mut self.command_tx {
                sender.send(Action::Command(item.name.clone())).unwrap();
            }
        }
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Command List").bold().centered().render(area, buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Command List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .list
            .items
            .iter()
            .map(|command_item| {
                let color = alternate_colors(command_item.status);
                ListItem::from(command_item).bg(color)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.list.state.selected() {
            match self.list.items[i].status {
                Status::NotImplemented => format!("[NI] {}", self.list.items[i].name),
                Status::Infeasible => format!("[NA] {}", self.list.items[i].name),
                Status::Feasible => format!("[GO] {}", self.list.items[i].name),
            }
        } else {
            "Nothing selected...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("TODO Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char('h') | KeyCode::Left => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Enter => self.run_selected_cmd(),
            _ => {}
        }

        Ok(None)
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        //frame.render_widget(Paragraph::new("hello ratatui"), area);

        let [header_area, main_area, footer_area] =
            Layout::vertical([Constraint::Length(2), Constraint::Fill(1), Constraint::Length(1)])
                .areas(area);

        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        let buf = frame.buffer_mut();
        self.render_header(header_area, buf);
        self.render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);

        Ok(())
    }
}

const fn alternate_colors(i: Status) -> Color {
    match i {
        Status::NotImplemented => NOT_IMPLEMENTED_ROW_BG_COLOR,
        Status::Infeasible => INFEASIBLE_ROW_BG_COLOR,
        Status::Feasible => NORMAL_ROW_BG,
    }
}

impl From<&CommandItem> for ListItem<'_> {
    fn from(value: &CommandItem) -> Self {
        let line = match value.status {
            Status::NotImplemented | Status::Infeasible => {
                Line::styled(format!(" ☐ {}", value.name), TEXT_FG_COLOR)
            }
            Status::Feasible => Line::styled(format!(" ✓ {}", value.name), COMPLETED_TEXT_FG_COLOR),
        };
        ListItem::new(line)
    }
}
