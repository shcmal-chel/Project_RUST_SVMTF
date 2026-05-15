import init, { TrafficSimulation } from './pkg/traffic_simulation.js';

let sim = null;
let state = {
    vehicles: [],
    total_vehicles: 0,
    current_time: 0,
    simulation_speed: 0.5,
    is_running: false,
    is_paused: false,
    avg_speed: 0,
    throughput: 0,
    zoom: 1.0,
    offset_x: 0,
    offset_y: 0,
    scenario_name: 'Базовое движение'
};

const canvas = document.getElementById('roadCanvas');
const ctx = canvas.getContext('2d');

// Устанавливаем размер canvas
canvas.width = 800;
canvas.height = 400;

function draw() {
    if (!sim) return;
    
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.fillStyle = '#0a0a1a';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    const zoom = state.zoom;
    const offsetX = state.offset_x;
    const offsetY = state.offset_y;
    
    // Рисуем дорогу (без трансформации, фиксированные координаты)
    const roadY = canvas.height / 2;
    const startX = 80;
    const endX = canvas.width - 80;
    
    ctx.beginPath();
    ctx.strokeStyle = '#00ff00';
    ctx.lineWidth = 4;
    ctx.moveTo(startX, roadY);
    ctx.lineTo(endX, roadY);
    ctx.stroke();
    
    // Стрелка
    ctx.fillStyle = 'white';
    ctx.font = '20px Arial';
    ctx.fillText('→', canvas.width / 2 - 20, roadY - 5);
    
    // Въезд
    ctx.fillStyle = '#00ff00';
    ctx.font = '24px Arial';
    ctx.fillText('🚪', startX - 15, roadY - 10);
    ctx.font = '10px Arial';
    ctx.fillText('ВЪЕЗД', startX - 20, roadY + 15);
    
    // Выезд
    ctx.fillStyle = '#ff0000';
    ctx.font = '24px Arial';
    ctx.fillText('🏁', endX - 10, roadY - 10);
    ctx.font = '10px Arial';
    ctx.fillText('ВЫЕЗД', endX - 25, roadY + 15);
    
    // Рисуем машины с учетом zoom и offset
    for (let v of state.vehicles) {
        // Координата X машины с учетом zoom и offset
        let x = (v.x / 100) * (endX - startX) + startX;
        x = x * zoom + offsetX;
        
        // Если машина в пределах экрана
        if (x > -50 && x < canvas.width + 50) {
            let symbol = '🚗';
            if (v.vehicle_type === 'Truck') symbol = '🚚';
            if (v.vehicle_type === 'Bus') symbol = '🚌';
            
            ctx.fillStyle = '#00d4ff';
            ctx.font = '24px Arial';
            ctx.fillText(symbol, x - 10, roadY - 5);
        }
    }
    
    // Информация
    ctx.fillStyle = '#888';
    ctx.font = '12px Arial';
    ctx.fillText(`Zoom: ${zoom.toFixed(1)}x`, canvas.width - 80, canvas.height - 10);
    ctx.fillText(`Offset: (${offsetX.toFixed(0)}, ${offsetY.toFixed(0)})`, canvas.width - 180, canvas.height - 10);
    ctx.fillText(`Машин: ${state.vehicles.length}`, canvas.width - 180, canvas.height - 25);
    ctx.fillText(`Время: ${state.current_time.toFixed(1)}с`, canvas.width - 180, canvas.height - 40);
}

function updateUI() {
    document.getElementById('totalVehicles').textContent = state.total_vehicles || 0;
    document.getElementById('activeVehicles').textContent = state.vehicles?.length || 0;
    document.getElementById('avgSpeed').textContent = `${(state.avg_speed || 0).toFixed(1)} км/ч`;
    document.getElementById('throughput').textContent = `${(state.throughput || 0).toFixed(0)}/мин`;
    document.getElementById('simTime').textContent = (state.current_time || 0).toFixed(1);
    document.getElementById('simSpeed').textContent = `${(state.simulation_speed || 0.5).toFixed(1)}x`;
    document.getElementById('speedValue').textContent = `${(state.simulation_speed || 0.5).toFixed(1)}x`;
    document.getElementById('scenarioName').textContent = state.scenario_name || 'Базовое движение';
    
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
    
    draw();
}

function showNotification(msg) {
    console.log(msg);
}

async function initSimulation() {
    try {
        await init();
        sim = new TrafficSimulation();
        console.log('✅ Симуляция создана!');
        
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
        
        document.getElementById('scenario1Btn').onclick = () => {
            sim.load_scenario(0);
            showNotification('📊 Базовое движение');
        };
        document.getElementById('scenario2Btn').onclick = () => {
            sim.load_scenario(1);
            showNotification('📈 Увеличение интенсивности');
        };
        document.getElementById('scenario3Btn').onclick = () => {
            sim.load_scenario(2);
            showNotification('🚧 Перекрытие дороги');
        };
        document.getElementById('scenario4Btn').onclick = () => {
            sim.load_scenario(3);
            showNotification('🚦 Оптимизация светофоров');
        };
        
        document.getElementById('reportBtn').onclick = () => {
            console.log('ОТЧЕТ:', {
                время: state.current_time,
                машины: state.total_vehicles,
                скорость: state.avg_speed,
                сценарий: state.scenario_name
            });
            alert('📄 Отчет в консоли (F12)');
        };
        
        document.getElementById('clearStatsBtn').onclick = () => { sim.reset(); };
        
        function gameLoop() {
            const newState = sim.step();
            state = newState;
            updateUI();
            requestAnimationFrame(gameLoop);
        }
        
        gameLoop();
        
    } catch (error) {
        console.error('❌ Ошибка:', error);
        document.getElementById('status').innerHTML = '❌ ОШИБКА ЗАГРУЗКИ';
    }
}

initSimulation();