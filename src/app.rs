use std::io;
use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
       // Outer block with centered title "Rustify"
        let outer_block = Block::default()
            .title(Span::styled(
                " Rustify ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        // Calculate inner layout inside the outer block
        let area = frame.size();
        frame.render_widget(outer_block.clone(), area);
        let inner_area = outer_block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(80),  // Top part (Music Library + Player)
                Constraint::Percentage(20),  // Bottom part (Instructions)
            ])
            .split(inner_area);

        // Split the top part into two columns
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),  // Left panel (Music Library)
                Constraint::Percentage(60),  // Right panel (Music Player)
            ])
            .split(chunks[0]);

        // Left Panel: Music Library
        let songs = vec![
            ListItem::new("Song 1"),
            ListItem::new("Song 2"),
            ListItem::new("Song 3"),
            ListItem::new("Song 4"),
        ];
        let music_library = List::new(songs)
            .block(Block::default().title("Music Library").borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_widget(music_library, top_chunks[0]);

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
        ])
        .block(Block::default().title("Controls").borders(Borders::ALL));
        frame.render_widget(instructions, chunks[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        // Handle keyboard inputs or other events here
        Ok(())
    }
}
