use std::{fs, io, path::Path, rc::Rc, sync::Arc, time::{Duration, Instant}};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};
use glob::glob;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

pub struct App {
    exit: bool,
    music_files: Vec<String>,
    selected_song: usize,  
    sink: Arc<Sink>,    
    is_playing: bool,      
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            music_files: Vec::new(),
            selected_song: 0,
            sink: Arc::new(Sink::new_idle().0),
            is_playing: false,
        }
    }
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
            Line::from(Span::styled("[↑/↓] Navigate ", Style::default().fg(Color::Yellow))),
            Line::from(Span::styled("[P] Play ", Style::default().fg(Color::Green))),
            Line::from(Span::styled("[Q] Quit ", Style::default().fg(Color::Red))),
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
            )
            .highlight_symbol("> ");  // Show an arrow next to the selected item

        frame.render_stateful_widget(music_library, chunks, &mut self.selected_song_state());
    }

    fn selected_song_state(&self) -> ListState {
        let mut state = ListState::default();
        state.select(Some(self.selected_song));
        state
    }


    fn move_selection_up(&mut self) {
        if self.selected_song > 0 {
            self.selected_song -= 1;
        }
    }

    fn move_selection_down(&mut self) {
        if self.selected_song < self.music_files.len() - 1 {
            self.selected_song += 1;
        }
    }

    fn play_song(&mut self) {
        if let Some(song) = self.music_files.get(self.selected_song) {
            // If the song is already playing, just return
            if self.is_playing {
                println!("Song is already playing.");
                return;
            }

            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            // let sink = Sink::try_new(&stream_handle).unwrap();

            let file = BufReader::new(File::open(song).expect("Failed to open file"));
            let source = Decoder::new(file).expect("Failed to decode audio");

            // Store the sink and playback state
            self.sink = Arc::new(Sink::try_new(&stream_handle).unwrap());
            self.is_playing = true;
            let sclone = self.sink.clone();
            sclone.append(source);
            

            // Wait until the song ends
            sclone.sleep_until_end();

            println!("Playing: {}", song);
        } else {
            println!("No song selected.");
        }
    }

    // Method to pause the song
    fn pause_song(&mut self) {
         if self.sink.is_paused() {
            self.sink.play()
        } else {
            self.sink.pause()
        }
    }

    // Method to resume the song
    // fn resume_song(&mut self) {
    //     if let Some(song) = self.music_files.get(self.selected_song) {
    //         if !self.is_playing {
    //             // If the song was paused, resume it
    //             let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    //             let sink = Sink::try_new(&stream_handle).unwrap();

    //             let file = BufReader::new(File::open(song).expect("Failed to open file"));
    //             let source = Decoder::new(file).expect("Failed to decode audio");

    //             sink.append(source);

    //             // Store the sink and set the song to playing
    //             self.sink = Some(sink);
    //             self.is_playing = true;

    //             println!("Resuming: {}", song);
    //         } else {
    //             println!("The song is already playing.");
    //         }
    //     } else {
    //         println!("No song selected.");
    //     }
    // }

    fn scan_for_songs(&mut self) {
        let mut files = Vec::new();
        let extensions = ["mp3", "wav", "ogg"];

        for ext in &extensions {
            let pattern = format!("C:/Users/user/Downloads/*.{}", ext);

            for entry in glob(&pattern).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        if path.is_file() {
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
        }

        self.music_files = files;
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
                    KeyCode::Char('p') => self.play_song(),
                    KeyCode::Char('[') => self.pause_song(),
                    // KeyCode::Char(']') => self.resume_song(),
                    KeyCode::Up => self.move_selection_up(),
                    KeyCode::Down => self.move_selection_down(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        Ok(())
    }
}
