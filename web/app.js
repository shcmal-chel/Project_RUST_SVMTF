import init, { TrafficSimulation } from './pkg/traffic_simulation.js';

let sim = null;

const canvas = document.getElementById('roadCanvas');
const ctx = canvas.getContext('2d');

// Переменные для хранения состояния камеры
let cameraZoom = 1.0;
let cameraOffsetX = 0;
let cameraOffsetY = 0;

// Функция уведомлений
function showNotification(message) {
    const notification = document.createElement('div');
    notification.style.position = 'fixed';
    notification.style.bottom = '20px';
    notification.style.right = '20px';
    notification.style.backgroundColor = '#00d4ff';
    notification.style.color = '#1a1a2e';
    notification.style.padding = '10px 20px';
    notification.style.borderRadius = '10px';
    notification.style.fontSize = '14px';
    notification.style.fontWeight = 'bold';
    notification.style.zIndex = '1000';
    notification.style.boxShadow = '0 5px 15px rgba(0,0,0,0.3)';
    notification.textContent = message;
    document.body.appendChild(notification);
    
    setTimeout(() => {
        notification.style.opacity = '0';
        notification.style.transition = 'opacity 0.5s';
        setTimeout(() => notification.remove(), 500);
    }, 2000);
}

function drawRoads(state) {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    ctx.fillStyle = '#0a0a1a';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    const zoom = state.zoom || 1.0;
    const offsetX = state.offset_x || 0;
    const offsetY = state.offset_y || 0;
    
    cameraZoom = zoom;
    cameraOffsetX = offsetX;
    cameraOffsetY = offsetY;
    
    const centerX = canvas.width / 2 + offsetX;
    const centerY = canvas.height / 2 + offsetY;
    
    const roadColors = { road_1: '#00ff00', road_2: '#00ff00', road_3: '#ffff00' };
    if (state.roads) {
        for (let r of state.roads) {
            if (r.id === 'road_1') roadColors.road_1 = r.color === 'green' ? '#00ff00' : (r.color === 'yellow' ? '#ffff00' : '#ff0000');
            if (r.id === 'road_2') roadColors.road_2 = r.color === 'green' ? '#00ff00' : (r.color === 'yellow' ? '#ffff00' : '#ff0000');
            if (r.id === 'road_3') roadColors.road_3 = r.color === 'green' ? '#00ff00' : (r.color === 'yellow' ? '#ffff00' : '#ff0000');
        }
    }
    
    ctx.save();
    ctx.scale(zoom, zoom);
    ctx.translate(offsetX / zoom, offsetY / zoom);
    
    // Дорога 1
    ctx.beginPath();
    ctx.strokeStyle = roadColors.road_1;
    ctx.lineWidth = 6 / zoom;
    ctx.moveTo(50, centerY - 60);
    ctx.lineTo(centerX - 30, centerY - 60);
    ctx.stroke();
    
    // Дорога 2
    ctx.beginPath();
    ctx.strokeStyle = roadColors.road_2;
    ctx.moveTo(50, centerY + 60);
    ctx.lineTo(centerX - 30, centerY + 60);
    ctx.stroke();
    
    // Дорога 3
    ctx.beginPath();
    ctx.strokeStyle = roadColors.road_3;
    ctx.moveTo(centerX, 50);
    ctx.lineTo(centerX, canvas.height - 50);
    ctx.stroke();
    
    // Стрелки
    ctx.fillStyle = roadColors.road_1;
    ctx.font = `${Math.floor(24 / zoom)}px Arial`;
    ctx.fillText('→', (centerX - 50) / zoom, (centerY - 55) / zoom);
    
    ctx.fillStyle = roadColors.road_2;
    ctx.fillText('→', (centerX - 50) / zoom, (centerY + 65) / zoom);
    
    ctx.fillStyle = roadColors.road_3;
    ctx.fillText('↓', (centerX - 10) / zoom, (canvas.height - 60) / zoom);
    ctx.fillText('↑', (centerX - 10) / zoom, 60 / zoom);
    
    // Перекресток
    ctx.fillStyle = '#ffffff';
    ctx.shadowBlur = 5 / zoom;
    ctx.beginPath();
    ctx.arc(centerX / zoom, centerY / zoom, 12 / zoom, 0, Math.PI * 2);
    ctx.fill();
    
    // Светофор
    ctx.fillStyle = '#ff0000';
    ctx.beginPath();
    ctx.arc((centerX + 18) / zoom, (centerY - 18) / zoom, 6 / zoom, 0, Math.PI * 2);
    ctx.fill();
    ctx.fillStyle = '#00ff00';
    ctx.beginPath();
    ctx.arc((centerX + 18) / zoom, (centerY - 30) / zoom, 6 / zoom, 0, Math.PI * 2);
    ctx.fill();
    
    // Точка въезда
    ctx.fillStyle = '#00ff00';
    ctx.font = `${Math.floor(28 / zoom)}px Arial`;
    ctx.fillText('🚪', 20 / zoom, (centerY - 70) / zoom);
    ctx.font = `${Math.floor(12 / zoom)}px Arial`;
    ctx.fillText('ВЪЕЗД', 15 / zoom, (centerY - 45) / zoom);
    
    // Точка выезда
    ctx.fillStyle = '#ff0000';
    ctx.font = `${Math.floor(28 / zoom)}px Arial`;
    ctx.fillText('🏁', (canvas.width - 60) / zoom, (centerY + 50) / zoom);
    ctx.font = `${Math.floor(12 / zoom)}px Arial`;
    ctx.fillText('ВЫЕЗД', (canvas.width - 75) / zoom, (centerY + 75) / zoom);
    
    // Надписи дорог
    ctx.font = `${Math.floor(10 / zoom)}px Arial`;
    ctx.fillStyle = roadColors.road_1;
    ctx.fillText('Main Street East', (centerX - 120) / zoom, (centerY - 70) / zoom);
    
    ctx.fillStyle = roadColors.road_2;
    ctx.fillText('Main Street West', (centerX - 120) / zoom, (centerY + 75) / zoom);
    
    ctx.fillStyle = roadColors.road_3;
    ctx.fillText('Cross Street', (centerX + 20) / zoom, (centerY - 80) / zoom);
    
    // Загрузка дорог в процентах
    if (state.roads) {
        let road1 = state.roads.find(r => r.id === 'road_1');
        if (road1) {
            ctx.fillStyle = road1.color === 'green' ? '#00ff00' : (road1.color === 'yellow' ? '#ffff00' : '#ff0000');
            ctx.fillText(`Загрузка: ${road1.congestion.toFixed(0)}%`, (centerX - 120) / zoom, (centerY - 55) / zoom);
        }
        let road2 = state.roads.find(r => r.id === 'road_2');
        if (road2) {
            ctx.fillStyle = road2.color === 'green' ? '#00ff00' : (road2.color === 'yellow' ? '#ffff00' : '#ff0000');
            ctx.fillText(`Загрузка: ${road2.congestion.toFixed(0)}%`, (centerX - 120) / zoom, (centerY + 90) / zoom);
        }
        let road3 = state.roads.find(r => r.id === 'road_3');
        if (road3) {
            ctx.fillStyle = road3.color === 'green' ? '#00ff00' : (road3.color === 'yellow' ? '#ffff00' : '#ff0000');
            ctx.fillText(`Загрузка: ${road3.congestion.toFixed(0)}%`, (centerX + 20) / zoom, (centerY - 65) / zoom);
        }
    }
    
    ctx.restore();
    
    // Легенда
    ctx.font = '9px Arial';
    ctx.fillStyle = '#888';
    ctx.fillText('🟢 Свободно (<30%)', 10, canvas.height - 25);
    ctx.fillText('🟡 Средняя (30-60%)', 10, canvas.height - 15);
    ctx.fillText('🔴 Затор (>60%)', 10, canvas.height - 5);
    
    ctx.font = '8px Arial';
    ctx.fillStyle = '#666';
    ctx.fillText(`Масштаб: ${zoom.toFixed(1)}x`, canvas.width - 80, canvas.height - 5);
}

function drawVehicles(vehicles, zoom, offsetX, offsetY) {
    if (!vehicles) return;
    const centerX = canvas.width / 2;
    const centerY = canvas.height / 2;
    
    ctx.save();
    ctx.scale(zoom, zoom);
    ctx.translate(offsetX / zoom, offsetY / zoom);
    
    for (let v of vehicles) {
        let symbol = '🚗';
        if (v.vehicle_type === 'Truck') symbol = '🚚';
        if (v.vehicle_type === 'Bus') symbol = '🚌';
        
        let x, y;
        if (v.current_road === 'Main Street East') {
            x = 50 + (v.progress / 100) * (centerX - 80);
            y = centerY - 60;
        } else if (v.current_road === 'Main Street West') {
            x = 50 + (v.progress / 100) * (centerX - 80);
            y = centerY + 60;
        } else {
            x = centerX;
            y = 50 + (v.progress / 100) * (canvas.height - 100);
        }
        
        ctx.font = `${Math.floor(20 / zoom)}px Arial`;
        ctx.fillText(symbol, x / zoom - 10, y / zoom - 5);
        
        ctx.font = `${Math.floor(8 / zoom)}px Arial`;
        ctx.fillStyle = 'white';
        ctx.fillText(`${v.speed.toFixed(0)} км/ч`, x / zoom - 12, y / zoom - 15);
    }
    ctx.restore();
}

function updateUI(state) {
    document.getElementById('totalVehicles').textContent = state.total_vehicles || 0;
    document.getElementById('activeVehicles').textContent = state.vehicles?.length || 0;
    document.getElementById('avgSpeed').textContent = `${(state.avg_speed || 0).toFixed(1)} км/ч`;
    document.getElementById('throughput').textContent = `${(state.throughput || 0).toFixed(0)}/мин`;
    document.getElementById('simTime').textContent = (state.current_time || 0).toFixed(1);
    document.getElementById('simSpeed').textContent = `${(state.simulation_speed || 0.5).toFixed(1)}x`;
    document.getElementById('speedValue').textContent = `${(state.simulation_speed || 0.5).toFixed(1)}x`;
    
    const scenarioName = state.scenario_name || 'Базовое движение';
    document.getElementById('scenarioName').textContent = scenarioName;
    
    // Подсветка кнопок сценариев
    document.querySelectorAll('.scenario-btn').forEach(btn => {
        btn.style.background = 'rgba(0,212,255,0.2)';
    });
    if (scenarioName === 'Базовое движение') document.getElementById('scenario1Btn').style.background = '#00d4ff';
    else if (scenarioName === 'Увеличение интенсивности') document.getElementById('scenario2Btn').style.background = '#00d4ff';
    else if (scenarioName === 'Перекрытие дороги') document.getElementById('scenario3Btn').style.background = '#00d4ff';
    else if (scenarioName === 'Оптимизация светофоров') document.getElementById('scenario4Btn').style.background = '#00d4ff';
    
    const statusDiv = document.getElementById('status');
    if (state.is_running) {
        statusDiv.className = 'status running';
        statusDiv.innerHTML = '▶ ЗАПУЩЕНА';
    } else if (state.is_paused) {
        statusDiv.className = 'status paused';
        statusDiv.innerHTML = '⏸ ПАУЗА';
    } else {
        statusDiv.className = 'status stopped';
        statusDiv.innerHTML = '⏹ ОСТАНОВЛЕНА';
    }
    
    const vehicleList = document.getElementById('vehicleList');
    if (!state.vehicles || state.vehicles.length === 0) {
        vehicleList.innerHTML = '<div class="vehicle-item">🚗 Нет активных ТС</div>';
    } else {
        vehicleList.innerHTML = state.vehicles.map(v => `
            <div class="vehicle-item">
                <strong>${v.vehicle_type === 'Car' ? '🚗' : (v.vehicle_type === 'Truck' ? '🚚' : '🚌')} ${v.vehicle_type}</strong>
                <div>📍 ${v.current_road}</div>
                <div>📊 ${v.progress.toFixed(0)}% пути</div>
                <div>⚡ ${v.speed.toFixed(0)} км/ч</div>
                <div class="progress-bar"><div class="progress-fill" style="width: ${v.progress}%"></div></div>
            </div>
        `).join('');
    }
    
    drawRoads(state);
    drawVehicles(state.vehicles, state.zoom || 1.0, state.offset_x || 0, state.offset_y || 0);
}

function generateReport() {
    const report = {
        timestamp: new Date().toISOString(),
        total_vehicles: document.getElementById('totalVehicles').textContent,
        avg_speed: document.getElementById('avgSpeed').textContent,
        simulation_time: document.getElementById('simTime').textContent,
        scenario: document.getElementById('scenarioName').textContent
    };
    console.log('Отчет:', report);
    showNotification('📄 Отчет сохранен в консоль (F12)');
}

async function initSimulation() {
    try {
        await init();
        sim = new TrafficSimulation();
        
        document.getElementById('startBtn').onclick = () => { sim.start(); };
        document.getElementById('pauseBtn').onclick = () => { sim.pause(); };
        document.getElementById('stopBtn').onclick = () => { sim.stop(); };
        document.getElementById('resetBtn').onclick = () => { sim.reset(); };
        
        document.getElementById('speedUpBtn').onclick = () => {
            let speed = sim.get_speed();
            sim.set_speed(speed + 0.2);
        };
        document.getElementById('speedDownBtn').onclick = () => {
            let speed = sim.get_speed();
            sim.set_speed(speed - 0.2);
        };
        
        document.getElementById('zoomInBtn').onclick = () => { sim.zoom_in(); };
        document.getElementById('zoomOutBtn').onclick = () => { sim.zoom_out(); };
        document.getElementById('moveLeftBtn').onclick = () => { sim.move_left(); };
        document.getElementById('moveRightBtn').onclick = () => { sim.move_right(); };
        document.getElementById('moveUpBtn').onclick = () => { sim.move_up(); };
        document.getElementById('moveDownBtn').onclick = () => { sim.move_down(); };
        
        // Сценарии с уведомлениями
        document.getElementById('scenario1Btn').onclick = () => {
            sim.load_scenario(0);
            document.getElementById('scenarioName').textContent = 'Базовое движение';
            showNotification('📊 Базовое движение: нормальная интенсивность');
        };
        document.getElementById('scenario2Btn').onclick = () => {
            sim.load_scenario(1);
            document.getElementById('scenarioName').textContent = 'Увеличение интенсивности';
            showNotification('📈 Увеличение интенсивности: поток увеличен в 3 раза');
        };
        document.getElementById('scenario3Btn').onclick = () => {
            sim.load_scenario(2);
            document.getElementById('scenarioName').textContent = 'Перекрытие дороги';
            showNotification('🚧 Перекрытие дороги: Main Street East закрыта');
        };
        document.getElementById('scenario4Btn').onclick = () => {
            sim.load_scenario(3);
            document.getElementById('scenarioName').textContent = 'Оптимизация светофоров';
            showNotification('🚦 Оптимизация светофоров: ускорен цикл переключения');
        };
        
        document.getElementById('reportBtn').onclick = generateReport;
        document.getElementById('clearStatsBtn').onclick = () => { sim.reset(); };
        
        async function gameLoop() {
            const state = sim.step();
            updateUI(state);
            requestAnimationFrame(gameLoop);
        }
        
        gameLoop();
        console.log('Симуляция запущена!');
        
    } catch (error) {
        console.error('Ошибка:', error);
        document.getElementById('status').innerHTML = '❌ ОШИБКА ЗАГРУЗКИ';
    }
}

initSimulation();