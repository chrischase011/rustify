use std::{fs, io, path::Path, rc::Rc, time::{Duration, Instant}};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};
use glob::glob;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    music_files: Vec<String>,  // To hold the list of music files
}

impl App {
    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
        let secs = 1;
        let tick_rate = Duration::from_secs(secs);

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events(tick_rate)?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        let outer_block = self.draw_outer_block(frame, area);
        let inner_area = outer_block.inner(area);
        let (chunks, top_chunks) = self.draw_layout(inner_area);

        // Left Panel: Music Library
        self.draw_music_library(frame, top_chunks[0]);

        // Right Panel: Music Player / Histogram
        let data = [("Song 1", 7), ("Song 2", 5), ("Song 3", 10), ("Song 4", 3)];
        let barchart = BarChart::default()
            .block(Block::default().title("Music Player").borders(Borders::ALL))
            .data(&data)
            .bar_width(8)
            .bar_gap(2)
            .bar_style(Style::default().fg(Color::LightBlue))
            .value_style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(barchart, top_chunks[1]);

        // Bottom Panel: Instructions
        let instructions = Paragraph::new(vec![
            Line::from(Span::raw("Instructions: ")),
            Line::from(Span::styled("[P] Play ", Style::default().fg(Color::Green))),
            Line::from(Span::styled("[S] Stop ", Style::default().fg(Color::Red))),
            Line::from(Span::styled("[N] Next ", Style::default().fg(Color::Blue))),
            Line::from(Span::styled("[T] Scan for Songs ", Style::default().fg(Color::Magenta))),
        ])
        .block(Block::default().title("Controls").borders(Borders::ALL));
        frame.render_widget(instructions, chunks[1]);
    }

    fn draw_outer_block(&self, frame: &mut Frame, area: Rect) -> Block {
        let outer_block = Block::default()
            .title(Span::styled(
                " Rustify ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        frame.render_widget(outer_block.clone(), area);

        outer_block
    }

    fn draw_layout(&self, inner: Rect) -> (Rc<[ratatui::layout::Rect]>, Rc<[ratatui::layout::Rect]>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(80),  // Top part (Music Library + Player)
                Constraint::Percentage(20),  // Bottom part (Instructions)
            ])
            .split(inner);

        // Split the top part into two columns
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),  // Left panel (Music Library)
                Constraint::Percentage(60),  // Right panel (Music Player)
            ])
            .split(chunks[0]);

        (chunks, top_chunks)
    }

    fn draw_music_library(&self, frame: &mut Frame, chunks: Rect) {
        let songs: Vec<ListItem> = self.music_files.iter().map(|s| ListItem::new(s.as_str())).collect();
        let music_library = List::new(songs)
            .block(Block::default().title("Music Library").borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_widget(music_library, chunks);
    }

    fn handle_events(&mut self, tick_rate: Duration) -> io::Result<()> {
        let mut last_tick = Instant::now();
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout.clone())? {
            if let Ok(Event::Key(key)) = event::read() {
                match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('t') => self.scan_for_songs(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        Ok(())
    }

    // Method to scan the directory for audio files
    fn scan_for_songs(&mut self) {
        println!("Scanning for songs...");
        
        // Use forward slashes for path pattern
        let pattern = "C:/Users/**/**/*.{mp3,wav,ogg}";
    
        let mut files = Vec::new();

        // Using glob crate to match patterns
        for entry in glob(pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    // Check if the path is a file
                    if path.is_file() {
                        // Ensure proper conversion of Path to String
                        if let Some(path_str) = path.to_str() {
                            files.push(path_str.to_string());
                        } else {
                            eprintln!("Error converting path to string: {:?}", path);
                        }
                    }
                }
                Err(e) => eprintln!("Error reading file: {:?}", e),
            }
        }

        self.music_files = files;
        println!("Found {} songs", self.music_files.len());
    }
}
