use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Gauge, Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn read_lines_from_file<P>(file_path: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();
    let mut out = vec![];
    for line in lines.flatten() {
        out.push(line);
    }
    Ok(out)
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppState {
    Start,
    Parsing,
    Done,
    Exit,
}

#[derive(Debug)]
pub struct App {
    input_lines: Vec<String>,
    parsed_lines: Vec<String>,
    chunk_size: usize,
    last_parsing_idx: usize,
    total: u32,
    state: AppState,
}

impl App {
    pub fn default(input_lines: Vec<String>) -> App {
        App {
            input_lines,
            state: AppState::Start,
            parsed_lines: vec![],
            chunk_size: 5,
            last_parsing_idx: 0,
            total: 0,
        }
    }

    pub fn with_chunk_size(&mut self, chunk_size: usize) -> &mut App {
        self.chunk_size = chunk_size;
        self
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.state != AppState::Exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            self.update();
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let timeout = Duration::from_secs_f32(1.0 / 20.0);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(' ') | KeyCode::Enter => self.start_parsing(),
                        KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.state = AppState::Exit;
    }

    fn start_parsing(&mut self) {
        self.state = AppState::Parsing;
    }

    fn update(&mut self) {
        if self.state == AppState::Parsing {
            // parse chunk of lines
            let last_idx = self.input_lines.len() - 1;
            let next_idx = (self.last_parsing_idx + self.chunk_size).max(last_idx);
            for line_idx in self.last_parsing_idx..next_idx {
                // Line parsing logic goes here
                let line_parts: Vec<&str> = self.input_lines[line_idx].split(" ").collect();
                let result_str_parts: Vec<&str> = line_parts[0].split(":").collect();
                let result: u32 = result_str_parts[0].parse().unwrap();
                let mut nums: Vec<u32> = vec![];
                for num_idx in 1..line_parts.len() {
                    nums.push(line_parts[num_idx].parse().unwrap())
                }
                if is_valid(result, nums) {
                    self.total += result;
                }   
            }
            self.last_parsing_idx = self.chunk_size;
            if next_idx == last_idx {
                self.state = AppState::Done;
            }
        }
    }
}


pub fn is_valid(result: u32, nums: Vec<u32>) -> bool {
    // check if result cna be found with operations
    
    
    false
}

// implement Widget trait for App so we can just call render_widget once
// and all the render code can live here
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            AppState::Start => {
                let title = Line::from(" Advent of Code 2024 Over Engineered Terminal UI ".bold());
                let instructions = Line::from(vec![
                    " Quit ".into(),
                    "<Q> ".blue().bold(),
                    " Parse ".into(),
                    "<Space> ".blue().bold(),
                ]);
                let block = Block::bordered()
                    .title(title.centered())
                    .title_bottom(instructions.centered())
                    .border_set(border::THICK);
                let line_count = self.input_lines.len();
                let line_count_text = Text::from(vec![Line::from(vec![
                    "Input Line Count: ".into(),
                    format!("{line_count}").to_string().yellow(),
                ])]);

                Paragraph::new(line_count_text)
                    .centered()
                    .block(block)
                    .render(area, buf);
            }
            AppState::Parsing => {
                let title = Line::from(" Advent of Code 2024 Over Engineered Terminal UI ".bold());
                let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
                let block = Block::bordered()
                    .title(title.centered())
                    .title_bottom(instructions.centered())
                    .border_set(border::THICK);
                let progress = self.last_parsing_idx as f64 / self.input_lines.len() as f64;
                Gauge::default()
                    .block(block)
                    .ratio(progress)
                    .render(area, buf);
            }
            AppState::Done => {
                let title = Line::from(" Advent of Code 2024 Over Engineered Terminal UI ".bold());
                let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
                let block = Block::bordered()
                    .title(title.centered())
                    .title_bottom(instructions.centered())
                    .border_set(border::THICK);
                let total = self.total;
                let line_count_text = Text::from(vec![Line::from(vec![
                    "Total: ".into(),
                    format!("{total}").to_string().yellow(),
                ])]);

                Paragraph::new(line_count_text)
                    .centered()
                    .block(block)
                    .render(area, buf);
            }
            _ => {}
        }
    }
}

fn main() -> io::Result<()> {
    // grab args
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let lines = read_lines_from_file(file_path).unwrap();

    // create ratatui terminal
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let mut app = App::default(lines);
    app.run(&mut terminal).unwrap();
    // once app exits, return to normal terminal
    ratatui::restore();
    Ok(())
}
