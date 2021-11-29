use kube::{Client, ResourceExt, api::{Api, ListParams}};
use k8s_openapi::api::core::v1::Pod;
use std::io::{stdout, stdin};
use termion::raw::IntoRawMode;
use termion::event::{Key, Event};
use termion::input::TermRead;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, List, ListItem};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Modifier, Style};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;

    let pods: Api<Pod> = Api::namespaced(client, "linkapi");
    let lp = ListParams::default().labels("run=engine-code");
    let mut items = vec![];
    for p in pods.list(&lp).await? {
        items.push(ListItem::new(p.name()));
    }

    let stdout = stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let stdin = stdin();

    loop {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ].as_ref()
            )
            .split(f.size());


        let list_items = items.clone();
        let list = List::new(list_items)
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        let block = Block::default()
             .title("Resources")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        f.render_widget(list, chunks[1]);
    })?;

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            _ => {}
        }
    }
    }
    Ok(())
}

