import init, { TrafficSimulation } from './pkg/traffic_simulation.js';

let sim = null;

const canvas = document.getElementById('roadCanvas');
const ctx = canvas.getContext('2d');

function drawRoads(state) {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    ctx.fillStyle = '#0a0a1a';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    const centerX = canvas.width / 2;
    const centerY = canvas.height / 2;
    
    // Получаем цвета дорог из состояния
    const roadColors = { road_1: '#00ff00', road_2: '#00ff00', road_3: '#ffff00' };
    if (state.roads) {
        for (let r of state.roads) {
            if (r.id === 'road_1') roadColors.road_1 = r.color === 'green' ? '#00ff00' : (r.color === 'yellow' ? '#ffff00' : '#ff0000');
            if (r.id === 'road_2') roadColors.road_2 = r.color === 'green' ? '#00ff00' : (r.color === 'yellow' ? '#ffff00' : '#ff0000');
            if (r.id === 'road_3') roadColors.road_3 = r.color === 'green' ? '#00ff00' : (r.color === 'yellow' ? '#ffff00' : '#ff0000');
        }
    }
    
    // Дорога 1 - Main Street East (верхняя)
    ctx.beginPath();
    ctx.strokeStyle = roadColors.road_1;
    ctx.lineWidth = 6;
    ctx.moveTo(50, centerY - 60);
    ctx.lineTo(centerX - 30, centerY - 60);
    ctx.stroke();
    
    // Стрелка дороги 1
    ctx.fillStyle = roadColors.road_1;
    ctx.font = '24px Arial';
    ctx.fillText('→', centerX - 50, centerY - 55);
    
    // Дорога 2 - Main Street West (нижняя)
    ctx.beginPath();
    ctx.strokeStyle = roadColors.road_2;
    ctx.moveTo(50, centerY + 60);
    ctx.lineTo(centerX - 30, centerY + 60);
    ctx.stroke();
    
    // Стрелка дороги 2
    ctx.fillStyle = roadColors.road_2;
    ctx.fillText('→', centerX - 50, centerY + 65);
    
    // Дорога 3 - Cross Street (вертикальная)
    ctx.beginPath();
    ctx.strokeStyle = roadColors.road_3;
    ctx.moveTo(centerX, 50);
    ctx.lineTo(centerX, canvas.height - 50);
    ctx.stroke();
    
    // Стрелки дороги 3
    ctx.fillStyle = roadColors.road_3;
    ctx.fillText('↓', centerX - 10, canvas.height - 60);
    ctx.fillText('↑', centerX - 10, 60);
    
    // Перекресток
    ctx.fillStyle = '#ffffff';
    ctx.shadowBlur = 5;
    ctx.shadowColor = '#00d4ff';
    ctx.beginPath();
    ctx.arc(centerX, centerY, 12, 0, Math.PI * 2);
    ctx.fill();
    
    // Светофор
    ctx.fillStyle = '#ff0000';
    ctx.beginPath();
    ctx.arc(centerX + 18, centerY - 18, 6, 0, Math.PI * 2);
    ctx.fill();
    ctx.fillStyle = '#00ff00';
    ctx.beginPath();
    ctx.arc(centerX + 18, centerY - 30, 6, 0, Math.PI * 2);
    ctx.fill();
    
    // Точка въезда
    ctx.fillStyle = '#00ff00';
    ctx.font = '28px Arial';
    ctx.fillText('🚪', 20, centerY - 70);
    ctx.fillStyle = '#00ff00';
    ctx.font = '12px Arial';
    ctx.fillText('ВЪЕЗД', 15, centerY - 45);
    
    // Точка выезда
    ctx.fillStyle = '#ff0000';
    ctx.font = '28px Arial';
    ctx.fillText('🏁', canvas.width - 60, centerY + 50);
    ctx.fillStyle = '#ff0000';
    ctx.font = '12px Arial';
    ctx.fillText('ВЫЕЗД', canvas.width - 75, centerY + 75);
    
    // Надписи с загрузкой
    ctx.font = '10px Arial';
    ctx.fillStyle = roadColors.road_1;
    ctx.fillText('Main Street East', centerX - 120, centerY - 70);
    if (state.roads) {
        let road1 = state.roads.find(r => r.id === 'road_1');
        if (road1) ctx.fillText(`${road1.congestion.toFixed(0)}%`, centerX - 120, centerY - 58);
    }
    
    ctx.fillStyle = roadColors.road_2;
    ctx.fillText('Main Street West', centerX - 120, centerY + 75);
    if (state.roads) {
        let road2 = state.roads.find(r => r.id === 'road_2');
        if (road2) ctx.fillText(`${road2.congestion.toFixed(0)}%`, centerX - 120, centerY + 88);
    }
    
    ctx.fillStyle = roadColors.road_3;
    ctx.fillText('Cross Street', centerX + 20, centerY - 80);
    if (state.roads) {
        let road3 = state.roads.find(r => r.id === 'road_3');
        if (road3) ctx.fillText(`${road3.congestion.toFixed(0)}%`, centerX + 20, centerY - 68);
    }
    
    ctx.shadowBlur = 0;
    
    // Легенда
    ctx.font = '9px Arial';
    ctx.fillStyle = '#888';
    ctx.fillText('🟢 Свободно (<30%)', 10, canvas.height - 25);
    ctx.fillStyle = '#888';
    ctx.fillText('🟡 Средняя (30-60%)', 10, canvas.height - 15);
    ctx.fillStyle = '#888';
    ctx.fillText('🔴 Затор (>60%)', 10, canvas.height - 5);
}

function drawVehicles(vehicles) {
    if (!vehicles) return;
    const centerX = canvas.width / 2;
    const centerY = canvas.height / 2;
    
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
        
        ctx.font = '20px Arial';
        ctx.fillText(symbol, x - 10, y - 5);
        
        ctx.font = '8px Arial';
        ctx.fillStyle = 'white';
        ctx.fillText(`${v.speed.toFixed(0)} км/ч`, x - 12, y - 15);
    }
}

function updateUI(state) {
    // Статистика
    document.getElementById('totalVehicles').textContent = state.total_vehicles || 0;
    document.getElementById('activeVehicles').textContent = state.vehicles?.length || 0;
    document.getElementById('avgSpeed').textContent = `${(state.avg_speed || 0).toFixed(1)} км/ч`;
    document.getElementById('throughput').textContent = `${(state.throughput || 0).toFixed(0)}/мин`;
    document.getElementById('simTime').textContent = (state.current_time || 0).toFixed(1);
    document.getElementById('simSpeed').textContent = `${(state.simulation_speed || 0.5).toFixed(1)}x`;
    document.getElementById('speedValue').textContent = `${(state.simulation_speed || 0.5).toFixed(1)}x`;
    
    // Отображение активного сценария
    const scenarioName = state.scenario_name || 'Базовое движение';
    document.getElementById('scenarioName').textContent = scenarioName;
    
    // Подсветка активной кнопки сценария
    document.querySelectorAll('.scenario-btn').forEach(btn => {
        btn.style.background = 'rgba(0,212,255,0.2)';
        btn.style.border = 'none';
    });
    if (scenarioName === 'Базовое движение') {
        document.getElementById('scenario1Btn').style.background = '#00d4ff';
    } else if (scenarioName === 'Увеличение интенсивности') {
        document.getElementById('scenario2Btn').style.background = '#00d4ff';
    } else if (scenarioName === 'Перекрытие дороги') {
        document.getElementById('scenario3Btn').style.background = '#00d4ff';
    } else if (scenarioName === 'Оптимизация светофоров') {
        document.getElementById('scenario4Btn').style.background = '#00d4ff';
    }
    
    // Статус
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
    
    // Список машин
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
    drawVehicles(state.vehicles);
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
    alert('📄 Отчет сохранен! Проверьте консоль браузера (F12)');
}

async function initSimulation() {
    try {
        await init();
        sim = new TrafficSimulation();
        
        // Кнопки управления
        document.getElementById('startBtn').onclick = () => { sim.start(); };
        document.getElementById('pauseBtn').onclick = () => { sim.pause(); };
        document.getElementById('stopBtn').onclick = () => { sim.stop(); };
        document.getElementById('resetBtn').onclick = () => { sim.reset(); };
        
        // Скорость
        document.getElementById('speedUpBtn').onclick = () => {
            let speed = sim.get_speed();
            sim.set_speed(speed + 0.2);
        };
        document.getElementById('speedDownBtn').onclick = () => {
            let speed = sim.get_speed();
            sim.set_speed(speed - 0.2);
        };
        
        // Управление картой
        document.getElementById('zoomInBtn').onclick = () => { sim.zoom_in(); };
        document.getElementById('zoomOutBtn').onclick = () => { sim.zoom_out(); };
        document.getElementById('moveLeftBtn').onclick = () => { sim.move_left(); };
        document.getElementById('moveRightBtn').onclick = () => { sim.move_right(); };
        document.getElementById('moveUpBtn').onclick = () => { sim.move_up(); };
        document.getElementById('moveDownBtn').onclick = () => { sim.move_down(); };
        
        // Сценарии с визуальным откликом
        document.getElementById('scenario1Btn').onclick = () => {
            sim.load_scenario(0);
            document.getElementById('scenarioName').textContent = 'Базовое движение';
        };
        document.getElementById('scenario2Btn').onclick = () => {
            sim.load_scenario(1);
            document.getElementById('scenarioName').textContent = 'Увеличение интенсивности';
        };
        document.getElementById('scenario3Btn').onclick = () => {
            sim.load_scenario(2);
            document.getElementById('scenarioName').textContent = 'Перекрытие дороги';
        };
        document.getElementById('scenario4Btn').onclick = () => {
            sim.load_scenario(3);
            document.getElementById('scenarioName').textContent = 'Оптимизация светофоров';
        };
        
        // Отчет
        document.getElementById('reportBtn').onclick = generateReport;
        
        // Очистка
        document.getElementById('clearStatsBtn').onclick = () => {
            sim.reset();
        };
        
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