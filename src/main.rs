// #![cfg(feature = "cli")]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_must_use)]

// use std::io::{self};
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use ratatui::{
//     backend::{Backend, CrosstermBackend},
//     layout::{Alignment, Constraint, Direction, Layout, Rect},
//     style::{Color, Style},
//     text::{Span, Line as TextLine},
//     widgets::{
//         Block, Borders, Paragraph, List, ListItem, Table, Row, Cell,
//     },
//     Frame, Terminal,
// };
// use std::{error::Error, time::Duration};

// mod models;
// mod simulation;
// mod traffic_network;
// mod statistics;
// mod scenarios;

// use models::*;
// use statistics::SimulationStatistics;


// fn main() -> Result<(), Box<dyn Error>> {
//     env_logger::init();
//     run_cli_interface()?;
//     Ok(())
// }

// fn run_cli_interface() -> Result<(), Box<dyn Error>> {
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     let mut app = TrafficSimApp::new();
//     let res = run_app(&mut terminal, &mut app);
    
//     app.generate_report();
    
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;
    
//     if let Err(err) = res {
//         println!("Ошибка: {:?}", err);
//     }
    
//     Ok(())
// }

// fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut TrafficSimApp) -> io::Result<()> {
//     let tick_rate = Duration::from_millis(100);
//     loop {
//         terminal.draw(|f| ui(f, app))?;
        
//         if event::poll(tick_rate)? {
//             if let Event::Key(key) = event::read()? {
//                 match app.handle_input(key.code) {
//                     Action::Quit => return Ok(()),
//                     Action::None => {}
//                 }
//             }
//         }
        
//         app.update();
//     }
// }

// fn ui(f: &mut Frame, app: &TrafficSimApp) {
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .margin(1)
//         .constraints([
//             Constraint::Length(4),
//             Constraint::Min(20),
//             Constraint::Length(4),
//         ])
//         .split(f.size());

//     let title = Block::default()
//         .borders(Borders::ALL)
//         .title(" Система моделирования транспортных потоков ")
//         .title_alignment(Alignment::Center);
    
//     let help_text = if app.simulation_running {
//         "🟢 СИМУЛЯЦИЯ ЗАПУЩЕНА - нажмите 's' для паузы"
//     } else if app.simulation_paused {
//         "🟡 СИМУЛЯЦИЯ НА ПАУЗЕ - нажмите 's' для продолжения"
//     } else {
//         "🔴 СИМУЛЯЦИЯ ОСТАНОВЛЕНА - нажмите 's' для запуска"
//     };
    
//     let title_text = Paragraph::new(format!("{}\n{}", 
//         "Нажмите 's' для запуска/паузы, 'r' для сброса, '+'/'-' для скорости, '1-4' для сценариев, 'o' для отчета, 'q' для выхода",
//         help_text
//     ))
//     .block(title)
//     .alignment(Alignment::Center);
//     f.render_widget(title_text, chunks[0]);

//     let main_chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([
//             Constraint::Percentage(40),
//             Constraint::Percentage(30),
//             Constraint::Percentage(30),
//         ])
//         .split(chunks[1]);

//     render_network_visualization(f, app, main_chunks[0]);
//     render_statistics(f, app, main_chunks[1]);
//     render_controls(f, main_chunks[2]);

//     let status = Block::default().borders(Borders::TOP);
//     let status_text = Paragraph::new(format!(
//         " Статус: {} | Время: {:.1} с | ТС создано: {} | Активных ТС: {} | Скорость: {:.1}x | Сценарий: {} | {}",
//         app.simulation_status_string(),
//         app.current_time,
//         app.total_vehicles,
//         app.vehicles.len(),
//         app.simulation_speed,
//         app.current_scenario_name,
//         app.message
//     ))
//     .block(status);
//     f.render_widget(status_text, chunks[2]);
// }

// fn render_network_visualization(f: &mut Frame, app: &TrafficSimApp, area: Rect) {
//     let block = Block::default()
//         .borders(Borders::ALL)
//         .title(" Дорожная сеть ")
//         .title_alignment(Alignment::Center);
    
//     let mut content = String::new();
//     content.push_str("\n");
//     content.push_str("  🟢═══════════════════════════════════════════════════════════════════════════→\n");
//     content.push_str("  Main Street East                                🚪 Въезд                     \n");
//     content.push_str("                                                                               \n");
//     content.push_str("                                    ┌───┐                                      \n");
//     content.push_str("                                    │ ○ │  ← Перекресток                       \n");
//     content.push_str("                                    └───┘                                      \n");
//     content.push_str("                                      │                                        \n");
//     content.push_str("  🟡══════════════════════════════════╪═══════════════════════════════════════→\n");
//     content.push_str("  Cross Street                                                                 \n");
//     content.push_str("                                      │                                        \n");
//     content.push_str("  🟢══════════════════════════════════╪═══════════════════════════════════════→\n");
//     content.push_str("  Main Street West                               🏁 Выезд                     \n");
//     content.push_str("                                                                               \n");
    
//     if !app.vehicles.is_empty() {
//         content.push_str("\n  🚗 ТРАНСПОРТНЫЕ СРЕДСТВА:\n");
//         for (i, vehicle) in app.vehicles.iter().enumerate() {
//             let symbol = match vehicle.vehicle_type {
//                 VehicleType::Car => "🚗",
//                 VehicleType::Truck => "🚚",
//                 VehicleType::Bus => "🚌",
//                 VehicleType::Emergency => "🚑",
//             };
            
//             let road_name = if vehicle.current_road == "road_1" { "Main Street East" } 
//                            else if vehicle.current_road == "road_2" { "Main Street West" }
//                            else { "Cross Street" };
            
//             let progress = ((vehicle.distance_traveled / 38.0) * 100.0).min(100.0);
//             let bar_len = (progress / 5.0) as usize;
//             let bar = "█".repeat(bar_len);
            
//             content.push_str(&format!("  {}. {} {} [{}] {:.0}%\n", 
//                 i+1, symbol, road_name, bar, progress));
//         }
//     } else {
//         content.push_str("\n  ❌ НЕТ АКТИВНЫХ ТРАНСПОРТНЫХ СРЕДСТВ\n");
//         content.push_str("     Нажмите 's' для запуска симуляции\n");
//     }
    
//     content.push_str(&format!("\n  📊 Всего машин на дорогах: {}\n", app.vehicles.len()));
    
//     let paragraph = Paragraph::new(content)
//         .block(block)
//         .alignment(Alignment::Left);
    
//     f.render_widget(paragraph, area);
// }

// fn render_statistics(f: &mut Frame, app: &TrafficSimApp, area: Rect) {
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Length(12),
//             Constraint::Min(0),
//         ])
//         .split(area);

//     let block = Block::default()
//         .borders(Borders::ALL)
//         .title(" Статистика ");
    
//     let total_vehicles_str = app.statistics.total_vehicles.to_string();
//     let avg_speed_str = format!("{:.1} км/ч", app.statistics.average_speed);
//     let max_congestion_str = format!("{:.1}%", app.statistics.max_congestion);
//     let avg_wait_time_str = format!("{:.1} с", app.statistics.average_wait_time);
//     let throughput_str = format!("{:.0}/мин", app.statistics.throughput);
//     let active_vehicles_str = app.vehicles.len().to_string();
    
//     let stats_data = vec![
//         vec!["Всего создано ТС:", &total_vehicles_str],
//         vec!["Средняя скорость:", &avg_speed_str],
//         vec!["Макс. загрузка сети:", &max_congestion_str],
//         vec!["Ср. время ожидания:", &avg_wait_time_str],
//         vec!["Пропускная способность:", &throughput_str],
//         vec!["Активных ТС сейчас:", &active_vehicles_str],
//     ];
    
//     let rows: Vec<Row> = stats_data.iter().map(|row| {
//         Row::new(vec![
//             Cell::from(row[0]).style(Style::default().fg(Color::Cyan)),
//             Cell::from(row[1]).style(Style::default().fg(Color::White)),
//         ])
//     }).collect();
    
//     let table = Table::new(rows)
//         .block(block)
//         .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);
    
//     f.render_widget(table, chunks[0]);
    
//     let congestion_block = Block::default()
//         .borders(Borders::ALL)
//         .title(" Наиболее загруженные участки ");
    
//     let congestion_items: Vec<ListItem> = if app.statistics.most_congested_roads.is_empty() {
//         vec![ListItem::new("Нет загруженных участков".to_string())]
//     } else {
//         app.statistics.most_congested_roads
//             .iter()
//             .take(5)
//             .map(|(name, level)| {
//                 let bar_len = ((level / 10.0) as usize).min(20);
//                 let bar = "█".repeat(bar_len);
//                 let text = format!("{}: {:.0}% [{}]", name, level, bar);
//                 ListItem::new(text).style(Style::default().fg(Color::Yellow))
//             })
//             .collect()
//     };
    
//     let congestion_list = List::new(congestion_items).block(congestion_block);
//     f.render_widget(congestion_list, chunks[1]);
// }

// fn render_controls(f: &mut Frame, area: Rect) {
//     let block = Block::default()
//         .borders(Borders::ALL)
//         .title(" Управление ");
    
//     let controls_text = vec![
//         TextLine::from(vec![
//             Span::styled(" ▶", Style::default().fg(Color::Green)),
//             Span::raw(" - "),
//             Span::styled("[s]", Style::default().fg(Color::Green)),
//             Span::raw(" Старт/Пауза/Стоп"),
//         ]),
//         TextLine::from(vec![
//             Span::styled(" 🔄", Style::default().fg(Color::Yellow)),
//             Span::raw(" - "),
//             Span::styled("[r]", Style::default().fg(Color::Yellow)),
//             Span::raw(" Полный сброс"),
//         ]),
//         TextLine::from(vec![
//             Span::styled(" ⚡", Style::default().fg(Color::Cyan)),
//             Span::raw(" - "),
//             Span::styled("[+] / [-]", Style::default().fg(Color::Cyan)),
//             Span::raw(" Скорость"),
//         ]),
//         TextLine::from(vec![
//             Span::styled(" 📊", Style::default().fg(Color::Blue)),
//             Span::raw(" - "),
//             Span::styled("[1]", Style::default().fg(Color::Blue)),
//             Span::raw(" Базовое движение"),
//         ]),
//         TextLine::from(vec![
//             Span::styled(" 📈", Style::default().fg(Color::Blue)),
//             Span::raw(" - "),
//             Span::styled("[2]", Style::default().fg(Color::Blue)),
//             Span::raw(" Увеличение интенсивности"),
//         ]),
//         TextLine::from(vec![
//             Span::styled(" 🚧", Style::default().fg(Color::Blue)),
//             Span::raw(" - "),
//             Span::styled("[3]", Style::default().fg(Color::Blue)),
//             Span::raw(" Перекрытие дороги"),
//         ]),
//         TextLine::from(vec![
//             Span::styled(" 🚦", Style::default().fg(Color::Blue)),
//             Span::raw(" - "),
//             Span::styled("[4]", Style::default().fg(Color::Blue)),
//             Span::raw(" Оптимизация светофоров"),
//         ]),
//         TextLine::from(vec![
//             Span::styled(" 📄", Style::default().fg(Color::Magenta)),
//             Span::raw(" - "),
//             Span::styled("[o]", Style::default().fg(Color::Magenta)),
//             Span::raw(" Отчет"),
//         ]),
//         TextLine::from(vec![
//             Span::styled(" 🗑", Style::default().fg(Color::Red)),
//             Span::raw(" - "),
//             Span::styled("[c]", Style::default().fg(Color::Red)),
//             Span::raw(" Очистить статистику"),
//         ]),
//         TextLine::from(vec![
//             Span::styled(" ❌", Style::default().fg(Color::Red)),
//             Span::raw(" - "),
//             Span::styled("[q]", Style::default().fg(Color::Red)),
//             Span::raw(" Выход"),
//         ]),
//         TextLine::from(vec![]),
//         TextLine::from(vec![
//             Span::styled("Цвета дорог:", Style::default().fg(Color::White)),
//         ]),
//         TextLine::from(vec![
//             Span::styled("🟢", Style::default().fg(Color::Green)),
//             Span::raw(" - Свободно (<30%)"),
//         ]),
//         TextLine::from(vec![
//             Span::styled("🟡", Style::default().fg(Color::Yellow)),
//             Span::raw(" - Средняя (30-60%)"),
//         ]),
//         TextLine::from(vec![
//             Span::styled("🔴", Style::default().fg(Color::Red)),
//             Span::raw(" - Затор (>60%)"),
//         ]),
//     ];
    
//     let paragraph = Paragraph::new(controls_text)
//         .block(block)
//         .alignment(Alignment::Left);
    
//     f.render_widget(paragraph, area);
// }

// struct TrafficSimApp {
//     network: TrafficNetwork,
//     vehicles: Vec<Vehicle>,
//     statistics: SimulationStatistics,
//     simulation_running: bool,
//     simulation_paused: bool,
//     simulation_speed: f64,
//     current_time: f64,
//     total_vehicles: u32,
//     current_scenario_name: String,
//     message: String,
//     message_timer: u8,
// }

// impl TrafficSimApp {
//     fn new() -> Self {
//         let network = TrafficNetwork::create_demo_network();
//         Self {
//             network,
//             vehicles: Vec::new(),
//             statistics: SimulationStatistics::default(),
//             simulation_running: false,
//             simulation_paused: false,
//             simulation_speed: 1.0,
//             current_time: 0.0,
//             total_vehicles: 0,
//             current_scenario_name: "Базовое движение".to_string(),
//             message: String::new(),
//             message_timer: 0,
//         }
//     }
    
//     fn show_message(&mut self, msg: &str) {
//         self.message = msg.to_string();
//         self.message_timer = 50;
//     }
    
//     fn handle_input(&mut self, key: KeyCode) -> Action {
//         match key {
//             KeyCode::Char('q') => Action::Quit,
//             KeyCode::Char('s') => {
//                 if !self.simulation_running && !self.simulation_paused {
//                     self.simulation_running = true;
//                     self.simulation_paused = false;
//                     self.show_message("▶ Симуляция ЗАПУЩЕНА");
//                 } else if self.simulation_running && !self.simulation_paused {
//                     self.simulation_running = false;
//                     self.simulation_paused = true;
//                     self.show_message("⏸ Симуляция НА ПАУЗЕ");
//                 } else {
//                     self.simulation_running = false;
//                     self.simulation_paused = false;
//                     self.reset_simulation();
//                     self.show_message("⏹ Симуляция ОСТАНОВЛЕНА");
//                 }
//                 Action::None
//             }
//             KeyCode::Char('r') => {
//                 self.reset_simulation();
//                 self.show_message("🔄 Полный сброс выполнен");
//                 Action::None
//             }
//             KeyCode::Char('+') => {
//                 self.simulation_speed = (self.simulation_speed * 1.5).min(10.0);
//                 self.show_message(&format!("⚡ Скорость: {:.1}x", self.simulation_speed));
//                 Action::None
//             }
//             KeyCode::Char('-') => {
//                 self.simulation_speed = (self.simulation_speed / 1.5).max(0.1);
//                 self.show_message(&format!("⚡ Скорость: {:.1}x", self.simulation_speed));
//                 Action::None
//             }
//             KeyCode::Char('1') => {
//                 self.load_scenario(0);
//                 Action::None
//             }
//             KeyCode::Char('2') => {
//                 self.load_scenario(1);
//                 Action::None
//             }
//             KeyCode::Char('3') => {
//                 self.load_scenario(2);
//                 Action::None
//             }
//             KeyCode::Char('4') => {
//                 self.load_scenario(3);
//                 Action::None
//             }
//             KeyCode::Char('o') => {
//                 self.generate_report();
//                 Action::None
//             }
//             KeyCode::Char('c') => {
//                 self.statistics.reset();
//                 self.total_vehicles = 0;
//                 self.show_message("🗑 Статистика очищена");
//                 Action::None
//             }
//             _ => Action::None,
//         }
//     }
    
//     fn update(&mut self) {
//         if self.message_timer > 0 {
//             self.message_timer -= 1;
//             if self.message_timer == 0 {
//                 self.message.clear();
//             }
//         }
        
//         if self.simulation_running && !self.simulation_paused {
//             self.current_time += 0.1 * self.simulation_speed;
//             self.update_vehicles();
//             self.spawn_vehicles();
//             self.update_statistics();
//             self.update_congestion_stats();
//         }
//     }
    
//     fn reset_simulation(&mut self) {
//         self.vehicles.clear();
//         self.current_time = 0.0;
//         self.total_vehicles = 0;
//         self.statistics.reset();
//         self.simulation_running = false;
//         self.simulation_paused = false;
//         self.network = TrafficNetwork::create_demo_network();
//         self.current_scenario_name = "Базовое движение".to_string();
//     }
    
//     fn load_scenario(&mut self, index: usize) {
//         let scenarios = scenarios::get_demo_scenarios();
//         if let Some((name, scenario)) = scenarios.get(index) {
//             let mut new_network = TrafficNetwork::create_demo_network();
//             if let Ok(()) = scenario.apply(&mut new_network) {
//                 self.network = new_network;
//                 self.current_scenario_name = name.clone();
//                 self.vehicles.clear();
//                 self.total_vehicles = 0;
//                 self.current_time = 0.0;
//                 self.statistics.reset();
//                 self.simulation_running = false;
//                 self.simulation_paused = false;
//                 self.show_message(&format!("📊 Сценарий загружен: {}", name));
//             }
//         }
//     }
    
//     fn generate_report(&mut self) {
//         use crate::statistics::Report;
        
//         let report = Report {
//             timestamp: chrono::Local::now().to_string(),
//             total_vehicles_processed: self.total_vehicles as u64,
//             average_travel_time: self.statistics.average_wait_time,
//             average_speed: self.statistics.average_speed,
//             segments_with_highest_load: self.statistics.most_congested_roads.clone(),
//             congestion_events: Vec::new(),
//             scenario_comparisons: std::collections::HashMap::new(),
//         };
        
//         if let Ok(json) = serde_json::to_string_pretty(&report) {
//             if std::fs::write("report.json", json).is_ok() {
//                 self.show_message("📄 Отчет сохранен в report.json");
//                 println!("\n=== ОТЧЕТ ===\nВсего ТС: {}\nСр. скорость: {:.1} км/ч\n", 
//                     report.total_vehicles_processed, report.average_speed);
//             }
//         }
//     }
    
//     fn update_vehicles(&mut self) {
//         for vehicle in &mut self.vehicles {
//             let step = vehicle.target_speed * self.simulation_speed * 0.1;
//             vehicle.distance_traveled += step;
            
//             if let Some(road) = self.network.roads.iter().find(|r| r.id == vehicle.current_road) {
//                 if vehicle.distance_traveled >= road.length {
//                     if vehicle.route.len() > 1 {
//                         vehicle.current_road = vehicle.route[1].clone();
//                         vehicle.distance_traveled = 0.0;
//                     }
//                 }
                
//                 let t = (vehicle.distance_traveled / road.length).min(1.0);
//                 vehicle.position.x = road.start.x + (road.end.x - road.start.x) * t;
//                 vehicle.position.y = road.start.y + (road.end.y - road.start.y) * t;
//             }
//         }
        
//         self.vehicles.retain(|v| {
//             if let Some(road) = self.network.roads.iter().find(|r| r.id == v.current_road) {
//                 v.distance_traveled < road.length || v.route.len() > 1
//             } else {
//                 false
//             }
//         });
//     }
    
//     fn spawn_vehicles(&mut self) {
//         use rand::Rng;
//         let mut rng = rand::thread_rng();
        
//         for entry in &self.network.entry_points {
//             let chance = entry.spawn_rate * self.simulation_speed / 20.0;
//             if rng.gen::<f64>() < chance {
//                 let vehicle_type = match rng.gen::<f64>() {
//                     x if x < 0.7 => VehicleType::Car,
//                     x if x < 0.9 => VehicleType::Truck,
//                     _ => VehicleType::Bus,
//                 };
                
//                 let target_speed = match vehicle_type {
//                     VehicleType::Car => 50.0,
//                     VehicleType::Truck => 35.0,
//                     VehicleType::Bus => 30.0,
//                     VehicleType::Emergency => 60.0,
//                 };
                
//                 let vehicle = Vehicle {
//                     id: uuid::Uuid::new_v4().to_string(),
//                     vehicle_type,
//                     position: entry.position.clone(),
//                     speed: target_speed,
//                     target_speed,
//                     route: vec!["road_1".to_string(), "road_2".to_string()],
//                     current_road: entry.road_id.clone(),
//                     distance_traveled: 0.0,
//                     waiting_time: 0.0,
//                 };
                
//                 self.vehicles.push(vehicle);
//                 self.total_vehicles += 1;
//             }
//         }
//     }
    
//     fn update_statistics(&mut self) {
//         if !self.vehicles.is_empty() {
//             let total_speed: f64 = self.vehicles.iter().map(|v| v.speed).sum();
//             self.statistics.average_speed = total_speed / self.vehicles.len() as f64;
//         }
//         self.statistics.total_vehicles = self.total_vehicles;
//         self.statistics.current_vehicles = self.vehicles.len() as u32;
        
//         if self.current_time > 0.0 {
//             self.statistics.throughput = (self.total_vehicles as f64 / self.current_time) * 60.0;
//             self.statistics.average_wait_time = self.current_time / (self.total_vehicles as f64).max(1.0);
//         }
//     }
    
//     fn update_congestion_stats(&mut self) {
//         let mut congestions = Vec::new();
//         for road in &self.network.roads {
//             let vehicle_count = self.vehicles.iter()
//                 .filter(|v| v.current_road == road.id)
//                 .count();
//             let load = vehicle_count as f64 / road.capacity as f64 * 100.0;
//             if load > 10.0 {
//                 congestions.push((road.name.clone(), load));
//             }
//         }
//         congestions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
//         self.statistics.most_congested_roads = congestions;
//         self.statistics.max_congestion = self.statistics.most_congested_roads
//             .first()
//             .map(|(_, l)| *l)
//             .unwrap_or(0.0);
//     }
    
//     fn simulation_status_string(&self) -> &str {
//         if self.simulation_running && !self.simulation_paused {
//             "▶ ЗАПУЩЕНА"
//         } else if !self.simulation_running && self.simulation_paused {
//             "⏸ НА ПАУЗЕ"
//         } else {
//             "⏹ ОСТАНОВЛЕНА"
//         }
//     }
// }

// enum Action {
//     Quit,
//     None,
// }