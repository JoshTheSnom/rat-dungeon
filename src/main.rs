use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::execute;
use crossterm::style::style;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Color;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use std::io::{Result, Stdout, Write, stdout};

const ROOM_W: i16 = 40;
const ROOM_H: i16 = 20;

enum Action {
    Move(i16, i16),
    Quit,
    None,
}

struct Enemy {
    x: i16,
    y: i16,
}

impl Enemy {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

struct App {
    x: i16,
    y: i16,
    enemies: Vec<Enemy>,
}

impl App {
    fn new() -> Self {
        let mut app = Self {
            x: 1,
            y: 1,
            enemies: Vec::new(),
        };
        app.spawn_enemies();
        app
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Left => self.try_move(-1, 0),
            KeyCode::Right => self.try_move(1, 0),
            KeyCode::Up => self.try_move(0, -1),
            KeyCode::Down => self.try_move(0, 1),
            KeyCode::Char('q') => return false,
            _ => {}
        }
        true
    }

    fn try_move(&mut self, dx: i16, dy: i16) {
        let nx = (self.x + dx).clamp(0, ROOM_W - 1);
        let ny = (self.y + dy).clamp(0, ROOM_H - 1);

        self.x = nx;
        self.y = ny;
    }

    fn spawn_enemies(&mut self) {
        let specs: &[(i16, i16)] = &[
            (3, 3),
            (ROOM_W - 4, 4),
            (ROOM_W / 2, ROOM_H / 2)];
        self.enemies = specs.iter().map(|&(x,y)| Enemy::new(x,y)).collect();
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

    std::thread::sleep(std::time::Duration::from_secs(2));

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
        terminal.draw(|f| draw(f, &app))?;

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

    let floor_style = Style::default().fg(Color::DarkGray);
    let player_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let mut cells: Vec<Vec<(char, Style)>> =
        vec![vec![('.', floor_style); ROOM_W as usize]; ROOM_H as usize];

    for enemy in app.enemies.iter() {
        let gx = enemy.x as usize;
        let gy = enemy.y as usize;

        let e_style = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
        cells[gy][gx] = ('*', e_style);
    }


    cells[app.y as usize][app.x as usize] = ('@', player_style);

    let lines: Vec<Line> = cells
        .iter()
        .map(|row| {
            Line::from(
                row.iter()
                    .map(|&(ch, st)| Span::styled(ch.to_string(), st))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let room = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Dungeon Room")
            .style(Style::default().fg(Color::White)),
    );
    f.render_widget(room, game_area);
}
