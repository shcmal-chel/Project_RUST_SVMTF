use std::io::{self};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Line as TextLine},
    widgets::{
        Block, Borders, Paragraph, List, ListItem, Table, Row, Cell,
        canvas::{Canvas, Points, Line as CanvasLine},
    },
    Frame, Terminal,
};
use std::{error::Error, time::Duration};

mod models;
mod simulation;
mod traffic_network;
mod statistics;
mod scenarios;

use models::*;
use statistics::SimulationStatistics;

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
    
    // При выходе генерируем отчет
    app.generate_report();
    
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    if let Err(err) = res {
        println!("Ошибка: {:?}", err);
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

fn ui(f: &mut Frame, app: &TrafficSimApp) {
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
        .title(" Система моделирования транспортных потоков ")
        .title_alignment(Alignment::Center);
    let title_text = Paragraph::new("Нажмите 's' для запуска/паузы, 'r' для сброса, '+'/'-' для скорости, 'l' для сценария, 'o' для отчета, 'q' для выхода")
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
        " Статус: {} | Время: {:.1} с | ТС: {} | Скорость: {:.1}x | Сценарий: {} ",
        app.simulation_status(),
        app.current_time,
        app.total_vehicles,
        app.simulation_speed,
        app.current_scenario_name
    ))
    .block(status);
    f.render_widget(status_text, chunks[2]);
}

fn render_network_visualization(f: &mut Frame, app: &TrafficSimApp, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Дорожная сеть ")
        .title_alignment(Alignment::Center);
    
    let zoom = app.zoom;
    let offset_x = app.offset_x;
    let offset_y = app.offset_y;
    
    let canvas = Canvas::default()
        .block(block)
        .x_bounds([0.0 + offset_x, 100.0 * zoom + offset_x])
        .y_bounds([0.0 + offset_y, 100.0 * zoom + offset_y])
        .paint(|ctx| {
            for road in &app.network.roads {
                let color = match road.congestion_level() {
                    CongestionLevel::Free => Color::Green,
                    CongestionLevel::Moderate => Color::Yellow,
                    CongestionLevel::Heavy => Color::Red,
                    CongestionLevel::Gridlock => Color::Red,
                };
                
                ctx.draw(&CanvasLine {
                    x1: road.start.x * zoom + offset_x,
                    y1: road.start.y * zoom + offset_y,
                    x2: road.end.x * zoom + offset_x,
                    y2: road.end.y * zoom + offset_y,
                    color: color,
                });
                
                // Рисуем стрелку направления
                let mid_x = (road.start.x + road.end.x) / 2.0 * zoom + offset_x;
                let mid_y = (road.start.y + road.end.y) / 2.0 * zoom + offset_y;
                let _ = ctx.print(mid_x, mid_y, "→");
            }
            
            for intersection in &app.network.intersections {
                let _ = ctx.draw(&Points {
                    coords: &[(
                        intersection.position.x * zoom + offset_x,
                        intersection.position.y * zoom + offset_y
                    )],
                    color: Color::White,
                });
            }
            
            for vehicle in &app.vehicles {
                let _ = ctx.print(
                    vehicle.position.x * zoom + offset_x,
                    vehicle.position.y * zoom + offset_y,
                    "🚗"
                );
            }
        });
    
    f.render_widget(canvas, area);
}

fn render_statistics(f: &mut Frame, app: &TrafficSimApp, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12),
            Constraint::Min(0),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Статистика ");
    
    let total_vehicles_str = app.statistics.total_vehicles.to_string();
    let avg_speed_str = format!("{:.1} км/ч", app.statistics.average_speed);
    let max_congestion_str = format!("{:.1}%", app.statistics.max_congestion);
    let avg_wait_time_str = format!("{:.1} с", app.statistics.average_wait_time);
    let throughput_str = format!("{:.0}/мин", app.statistics.throughput);
    let active_vehicles_str = app.vehicles.len().to_string();
    
    let stats_data = vec![
        vec!["Всего ТС:", &total_vehicles_str],
        vec!["Ср. скорость:", &avg_speed_str],
        vec!["Макс. загрузка:", &max_congestion_str],
        vec!["Ср. время ожидания:", &avg_wait_time_str],
        vec!["Пропускная способность:", &throughput_str],
        vec!["Активных ТС:", &active_vehicles_str],
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
        .title(" Наиболее загруженные участки ");
    
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

fn render_controls(f: &mut Frame, _app: &TrafficSimApp, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Управление ");
    
    let controls_text = vec![
        TextLine::from(vec![
            Span::styled("[s]", Style::default().fg(Color::Green)),
            Span::raw(" Старт/Пауза"),
        ]),
        TextLine::from(vec![
            Span::styled("[r]", Style::default().fg(Color::Yellow)),
            Span::raw(" Сброс"),
        ]),
        TextLine::from(vec![
            Span::styled("[+]", Style::default().fg(Color::Cyan)),
            Span::raw(" Увеличить скорость"),
        ]),
        TextLine::from(vec![
            Span::styled("[-]", Style::default().fg(Color::Cyan)),
            Span::raw(" Уменьшить скорость"),
        ]),
        TextLine::from(vec![
            Span::styled("[l]", Style::default().fg(Color::Magenta)),
            Span::raw(" Загрузить сценарий"),
        ]),
        TextLine::from(vec![
            Span::styled("[o]", Style::default().fg(Color::Blue)),
            Span::raw(" Сформировать отчет"),
        ]),
        TextLine::from(vec![
            Span::styled("[z]", Style::default().fg(Color::Green)),
            Span::raw(" Приблизить карту"),
        ]),
        TextLine::from(vec![
            Span::styled("[x]", Style::default().fg(Color::Green)),
            Span::raw(" Отдалить карту"),
        ]),
        TextLine::from(vec![
            Span::styled("[←→↑↓]", Style::default().fg(Color::Green)),
            Span::raw(" Переместить карту"),
        ]),
        TextLine::from(vec![
            Span::styled("[c]", Style::default().fg(Color::Red)),
            Span::raw(" Очистить статистику"),
        ]),
        TextLine::from(vec![
            Span::styled("[q]", Style::default().fg(Color::Red)),
            Span::raw(" Выход"),
        ]),
        TextLine::from(vec![]),
        TextLine::from(vec![
            Span::styled("Цвета дорог:", Style::default().fg(Color::White)),
        ]),
        TextLine::from(vec![
            Span::styled("🟢", Style::default().fg(Color::Green)),
            Span::raw(" - Свободно"),
        ]),
        TextLine::from(vec![
            Span::styled("🟡", Style::default().fg(Color::Yellow)),
            Span::raw(" - Средняя загрузка"),
        ]),
        TextLine::from(vec![
            Span::styled("🔴", Style::default().fg(Color::Red)),
            Span::raw(" - Затор"),
        ]),
        TextLine::from(vec![]),
        TextLine::from(vec![
            Span::styled("→", Style::default().fg(Color::Cyan)),
            Span::raw(" - Направление движения"),
        ]),
    ];
    
    let paragraph = Paragraph::new(controls_text)
        .block(block)
        .alignment(Alignment::Left);
    
    f.render_widget(paragraph, area);
}

struct TrafficSimApp {
    network: traffic_network::TrafficNetwork,
    vehicles: Vec<Vehicle>,
    statistics: SimulationStatistics,
    simulation_status: SimulationStatus,
    simulation_speed: f64,
    current_time: f64,
    total_vehicles: u32,
    current_scenario_name: String,
    zoom: f64,
    offset_x: f64,
    offset_y: f64,
}

impl TrafficSimApp {
    fn new() -> Self {
        Self {
            network: traffic_network::TrafficNetwork::create_demo_network(),
            vehicles: Vec::new(),
            statistics: SimulationStatistics::default(),
            simulation_status: SimulationStatus::Stopped,
            simulation_speed: 1.0,
            current_time: 0.0,
            total_vehicles: 0,
            current_scenario_name: "Базовый".to_string(),
            zoom: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
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
            KeyCode::Char('l') => {
                self.load_scenario_menu();
                Action::None
            }
            KeyCode::Char('o') => {
                self.generate_report();
                Action::None
            }
            KeyCode::Char('z') => {
                self.zoom *= 1.2;
                Action::None
            }
            KeyCode::Char('x') => {
                self.zoom /= 1.2;
                Action::None
            }
            KeyCode::Left => {
                self.offset_x += 10.0;
                Action::None
            }
            KeyCode::Right => {
                self.offset_x -= 10.0;
                Action::None
            }
            KeyCode::Up => {
                self.offset_y += 10.0;
                Action::None
            }
            KeyCode::Down => {
                self.offset_y -= 10.0;
                Action::None
            }
            KeyCode::Char('c') => {
                self.statistics.reset();
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
            self.update_congestion_stats();
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
        self.network = traffic_network::TrafficNetwork::create_demo_network();
    }
    
    fn increase_speed(&mut self) {
        self.simulation_speed = (self.simulation_speed * 1.5).min(10.0);
    }
    
    fn decrease_speed(&mut self) {
        self.simulation_speed = (self.simulation_speed / 1.5).max(0.1);
    }
    
    fn load_scenario_menu(&mut self) {
        let scenarios = scenarios::get_demo_scenarios();
        if let Some((name, scenario)) = scenarios.first() {
            if let Ok(()) = scenario.apply(&mut self.network) {
                self.current_scenario_name = name.clone();
                self.reset_simulation();
            }
        }
    }
    
    fn generate_report(&mut self) {
        use std::collections::HashMap;
        use crate::statistics::Report;
        
        let report = Report {
            timestamp: chrono::Local::now().to_string(),
            total_vehicles_processed: self.total_vehicles as u64,
            average_travel_time: self.statistics.average_wait_time,
            average_speed: self.statistics.average_speed,
            segments_with_highest_load: self.statistics.most_congested_roads.clone(),
            congestion_events: Vec::new(),
            scenario_comparisons: HashMap::new(),
        };
        
        let json = serde_json::to_string_pretty(&report).unwrap();
        std::fs::write("report.json", json).unwrap();
        
        println!("\n=== ОТЧЕТ СОХРАНЕН ===\n");
        println!("{}", report.generate_report());
        println!("\nОтчет сохранен в файл: report.json");
    }
    
    fn update_vehicles(&mut self) {
        for vehicle in &mut self.vehicles {
            vehicle.speed = vehicle.target_speed * self.simulation_speed;
            vehicle.distance_traveled += vehicle.speed * 0.1;
            // Обновляем позицию вдоль маршрута
            if let Some(road) = self.network.roads.iter().find(|r| r.id == vehicle.current_road) {
                let t = vehicle.distance_traveled / road.length;
                if t < 1.0 {
                    vehicle.position.x = road.start.x + (road.end.x - road.start.x) * t;
                    vehicle.position.y = road.start.y + (road.end.y - road.start.y) * t;
                } else if let Some(next_road) = vehicle.route.get(1) {
                    vehicle.current_road = next_road.clone();
                    vehicle.distance_traveled = 0.0;
                }
            }
        }
        self.vehicles.retain(|v| v.distance_traveled < 1000.0);
    }
    
    fn update_statistics(&mut self) {
        if !self.vehicles.is_empty() {
            let total_speed: f64 = self.vehicles.iter().map(|v| v.speed).sum();
            self.statistics.average_speed = total_speed / self.vehicles.len() as f64;
        }
        self.statistics.total_vehicles = self.total_vehicles;
        self.statistics.current_vehicles = self.vehicles.len() as u32;
    }
    
    fn update_congestion_stats(&mut self) {
        let mut congestions = Vec::new();
        for road in &self.network.roads {
            let load = road.current_vehicles.len() as f64 / road.capacity as f64 * 100.0;
            if load > 50.0 {
                congestions.push((road.name.clone(), load));
            }
        }
        congestions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        self.statistics.most_congested_roads = congestions;
        self.statistics.max_congestion = self.statistics.most_congested_roads
            .first()
            .map(|(_, l)| *l)
            .unwrap_or(0.0);
    }
    
    fn spawn_vehicles(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Сначала собираем данные о точках въезда
        let entry_points: Vec<(String, Point, f64)> = self.network.entry_points
            .iter()
            .map(|entry| (entry.road_id.clone(), entry.position.clone(), entry.spawn_rate))
            .collect();
        
        for (road_id, position, spawn_rate) in entry_points {
            let actual_spawn_rate = spawn_rate * self.simulation_speed;
            if rng.gen::<f64>() < actual_spawn_rate / 10.0 {
                if let Some(mut vehicle) = self.network.spawn_vehicle() {
                    vehicle.position = position;
                    vehicle.current_road = road_id.clone();
                    self.vehicles.push(vehicle);
                    self.total_vehicles += 1;
                }
            }
        }
    }
    
    fn simulation_status(&self) -> &str {
        match self.simulation_status {
            SimulationStatus::Running => "ЗАПУЩЕНА",
            SimulationStatus::Paused => "ПАУЗА",
            SimulationStatus::Stopped => "ОСТАНОВЛЕНА",
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