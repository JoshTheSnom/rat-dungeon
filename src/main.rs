use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::Paragraph;
use std::io::{Result, Stdout, Write, stdout};
use ratatui::text::Line;

const ROOM_W: u16 = 40;
const ROOM_H: u16 = 20;

enum Action {
    Move(i16, i16),
    Quit,
    None,
}

struct App {
    x: i16,
    y: i16,
}

impl App {
    fn new() -> Self {
        Self { x: 1, y: 1 }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Left => self.x -= 1,
            KeyCode::Right => self.x += 1,
            KeyCode::Up => self.y -= 1,
            KeyCode::Down => self.y += 1,
            KeyCode::Char('q') => return false,
            _ => {}
        }
        true
    }
}

fn main() -> Result<()> {
    println!(
        "You have been sent on a quest!\
    \nYour task is to kill all the rats in the dungeon.\
    \nExcept...\
    \nYou are also a rat.\
    \nAnd the other rats are also on a quest.\
    \nTo kill you.\
    \n\
    \nGood luck."
    );

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run(&mut terminal);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    res
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|mut f| {
            let area = Rect::new(app.x as u16, app.y as u16, 1, 1);
            let p = Paragraph::new("@");
            f.render_widget(p, area);
        })?;

        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                if !app.handle_key_event(key_event) {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn draw(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let game_area = chunks[0];
    let status_area = chunks[1];

    // Dungeon room

    let mut cells: Vec<Vec<char>> = vec![vec!['.'; ROOM_W as usize]; ROOM_H as usize];

    cells[app.y as usize][app.x as usize] = '@';
    
}