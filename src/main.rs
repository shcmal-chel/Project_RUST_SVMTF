use std::io::{self, Write};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, Borders, Paragraph, List, ListItem, Table, Row, Cell,
        canvas::{Canvas, Points, Line},
    },
    Frame, Terminal,
};
use std::{error::Error, time::Duration};

mod models;
mod simulation;
mod traffic_network;
mod statistics;
mod scenarios;
mod validators;

use models::*;
use simulation::*;
use traffic_network::*;
use statistics::*;
use scenarios::*;
use validators::*;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    run_cli_interface()?;
    Ok(())
}

fn run_cli_interface() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = TrafficSimApp::new();
    let res = run_app(&mut terminal, &mut app);
    
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    if let Err(err) = res {
        println!("Error: {:?}", err);
    }
    
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut TrafficSimApp) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100);
    loop {
        terminal.draw(|f| ui(f, app))?;
        
        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                match app.handle_input(key.code) {
                    Action::Quit => return Ok(()),
                    Action::None => {}
                }
            }
        }
        
        app.update();
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &TrafficSimApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(20),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = Block::default()
        .borders(Borders::ALL)
        .title(" Traffic Flow Simulation ")
        .title_alignment(Alignment::Center);
    let title_text = Paragraph::new("Press 'q' to quit, 's' to start/stop, 'r' to reset")
        .block(title)
        .alignment(Alignment::Center);
    f.render_widget(title_text, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(chunks[1]);

    render_network_visualization(f, app, main_chunks[0]);
    render_statistics(f, app, main_chunks[1]);
    render_controls(f, app, main_chunks[2]);

    let status = Block::default().borders(Borders::TOP);
    let status_text = Paragraph::new(format!(
        " Status: {} | Time: {:.1} | Vehicles: {} | Speed: {:.1}x ",
        app.simulation_status(),
        app.current_time,
        app.total_vehicles,
        app.simulation_speed
    ))
    .block(status);
    f.render_widget(status_text, chunks[2]);
}

fn render_network_visualization<B: Backend>(f: &mut Frame<B>, app: &TrafficSimApp, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Road Network ")
        .title_alignment(Alignment::Center);
    
    let canvas = Canvas::default()
        .block(block)
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(|ctx| {
            // Рисуем дороги - исправлено для новой версии ratatui
            for road in &app.network.roads {
                let color = match road.congestion_level() {
                    CongestionLevel::Free => Color::Green,
                    CongestionLevel::Moderate => Color::Yellow,
                    CongestionLevel::Heavy => Color::Red,
                    CongestionLevel::Gridlock => Color::DarkRed,
                };
                
                // Исправленное создание Line
                ctx.draw(&Line {
                    x1: road.start.x,
                    y1: road.start.y,
                    x2: road.end.x,
                    y2: road.end.y,
                    color: color,
                });
            }
            
            // Рисуем перекрестки
            for intersection in &app.network.intersections {
                let _ = ctx.draw(&Points {
                    coords: &[(intersection.position.x, intersection.position.y)],
                    color: Color::White,
                });
            }
            
            // Рисуем транспортные средства
            for vehicle in &app.vehicles {
                let color = match vehicle.vehicle_type {
                    VehicleType::Car => Color::Cyan,
                    VehicleType::Truck => Color::Yellow,
                    VehicleType::Bus => Color::Magenta,
                    VehicleType::Emergency => Color::Red,
                };
                let _ = ctx.print(vehicle.position.x, vehicle.position.y, "V");
            }
        });
    
    f.render_widget(canvas, area);
}

fn render_statistics<B: Backend>(f: &mut Frame<B>, app: &TrafficSimApp, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Min(0),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Statistics ");
    
    let stats_data = vec![
        vec!["Total Vehicles:", &app.statistics.total_vehicles.to_string()],
        vec!["Avg Speed:", &format!("{:.1} km/h", app.statistics.average_speed)],
        vec!["Max Congestion:", &format!("{:.1}%", app.statistics.max_congestion)],
        vec!["Avg Wait Time:", &format!("{:.1}s", app.statistics.average_wait_time)],
        vec!["Throughput:", &format!("{:.0}/min", app.statistics.throughput)],
    ];
    
    let rows: Vec<Row> = stats_data.iter().map(|row| {
        Row::new(vec![
            Cell::from(row[0]).style(Style::default().fg(Color::Cyan)),
            Cell::from(row[1]).style(Style::default().fg(Color::White)),
        ])
    }).collect();
    
    let table = Table::new(rows)
        .block(block)
        .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);
    
    f.render_widget(table, chunks[0]);
    
    let congestion_block = Block::default()
        .borders(Borders::ALL)
        .title(" Most Congested Roads ");
    
    let congestion_items: Vec<ListItem> = app.statistics.most_congested_roads
        .iter()
        .take(5)
        .map(|(name, level)| {
            let text = format!("{}: {:.0}%", name, level);
            ListItem::new(text).style(Style::default().fg(Color::Yellow))
        })
        .collect();
    
    let congestion_list = List::new(congestion_items).block(congestion_block);
    f.render_widget(congestion_list, chunks[1]);
}

fn render_controls<B: Backend>(f: &mut Frame<B>, app: &TrafficSimApp, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Controls ");
    
    let controls_text = vec![
        Spans::from(vec![
            Span::styled("[s]", Style::default().fg(Color::Green)),
            Span::raw(" Start/Pause"),
        ]),
        Spans::from(vec![
            Span::styled("[r]", Style::default().fg(Color::Yellow)),
            Span::raw(" Reset"),
        ]),
        Spans::from(vec![
            Span::styled("[+]", Style::default().fg(Color::Cyan)),
            Span::raw(" Increase Speed"),
        ]),
        Spans::from(vec![
            Span::styled("[-]", Style::default().fg(Color::Cyan)),
            Span::raw(" Decrease Speed"),
        ]),
        Spans::from(vec![
            Span::styled("[l]", Style::default().fg(Color::Magenta)),
            Span::raw(" Load Scenario"),
        ]),
        Spans::from(vec![
            Span::styled("[c]", Style::default().fg(Color::Red)),
            Span::raw(" Clear Statistics"),
        ]),
        Spans::from(vec![
            Span::styled("[q]", Style::default().fg(Color::Red)),
            Span::raw(" Quit"),
        ]),
    ];
    
    let paragraph = Paragraph::new(controls_text)
        .block(block)
        .alignment(Alignment::Left);
    
    f.render_widget(paragraph, area);
}

struct TrafficSimApp {
    network: TrafficNetwork,
    vehicles: Vec<Vehicle>,
    statistics: SimulationStatistics,
    simulation_status: SimulationStatus,
    simulation_speed: f64,
    current_time: f64,
    total_vehicles: u32,
    selected_scenario: Option<Scenario>,
}

impl TrafficSimApp {
    fn new() -> Self {
        Self {
            network: TrafficNetwork::create_demo_network(),
            vehicles: Vec::new(),
            statistics: SimulationStatistics::default(),
            simulation_status: SimulationStatus::Stopped,
            simulation_speed: 1.0,
            current_time: 0.0,
            total_vehicles: 0,
            selected_scenario: None,
        }
    }
    
    fn handle_input(&mut self, key: KeyCode) -> Action {
        match key {
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Char('s') => {
                self.toggle_simulation();
                Action::None
            }
            KeyCode::Char('r') => {
                self.reset_simulation();
                Action::None
            }
            KeyCode::Char('+') => {
                self.increase_speed();
                Action::None
            }
            KeyCode::Char('-') => {
                self.decrease_speed();
                Action::None
            }
            _ => Action::None,
        }
    }
    
    fn update(&mut self) {
        if let SimulationStatus::Running = self.simulation_status {
            self.current_time += 0.1 * self.simulation_speed;
            self.update_vehicles();
            self.update_statistics();
            self.spawn_vehicles();
        }
    }
    
    fn toggle_simulation(&mut self) {
        self.simulation_status = match self.simulation_status {
            SimulationStatus::Running => SimulationStatus::Paused,
            SimulationStatus::Paused => SimulationStatus::Running,
            SimulationStatus::Stopped => SimulationStatus::Running,
        };
    }
    
    fn reset_simulation(&mut self) {
        self.vehicles.clear();
        self.current_time = 0.0;
        self.total_vehicles = 0;
        self.statistics.reset();
        self.simulation_status = SimulationStatus::Stopped;
    }
    
    fn increase_speed(&mut self) {
        self.simulation_speed = (self.simulation_speed * 1.5).min(10.0);
    }
    
    fn decrease_speed(&mut self) {
        self.simulation_speed = (self.simulation_speed / 1.5).max(0.1);
    }
    
    fn update_vehicles(&mut self) {
        for vehicle in &mut self.vehicles {
            vehicle.speed = vehicle.target_speed * self.simulation_speed;
            vehicle.distance_traveled += vehicle.speed * 0.1;
        }
        self.vehicles.retain(|v| v.distance_traveled < 1000.0);
    }
    
    fn update_statistics(&mut self) {
        if !self.vehicles.is_empty() {
            let total_speed: f64 = self.vehicles.iter().map(|v| v.speed).sum();
            self.statistics.average_speed = total_speed / self.vehicles.len() as f64;
        }
        self.statistics.total_vehicles = self.total_vehicles;
    }
    
    fn spawn_vehicles(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let spawn_rate = 0.3 * self.simulation_speed;
        
        if rng.gen::<f64>() < spawn_rate {
            if let Some(vehicle) = self.network.spawn_vehicle() {
                self.vehicles.push(vehicle);
                self.total_vehicles += 1;
            }
        }
    }
    
    fn simulation_status(&self) -> &str {
        match self.simulation_status {
            SimulationStatus::Running => "RUNNING",
            SimulationStatus::Paused => "PAUSED",
            SimulationStatus::Stopped => "STOPPED",
        }
    }
}

enum Action {
    Quit,
    None,
}

#[derive(PartialEq)]
enum SimulationStatus {
    Running,
    Paused,
    Stopped,
}