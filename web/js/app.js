import init, { TrafficSimulation } from '../../pkg/traffic_simulation.js';

let sim = null;
let animationId = null;

const canvas = document.getElementById('roadCanvas');
const ctx = canvas.getContext('2d');

// Константы для рисования
const CANVAS_WIDTH = 600;
const CANVAS_HEIGHT = 500;
canvas.width = CANVAS_WIDTH;
canvas.height = CANVAS_HEIGHT;

// Рисование дорожной сети
function drawNetwork() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // Задний фон
    ctx.fillStyle = '#0a0a1a';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    
    const centerX = canvas.width / 2;
    const centerY = canvas.height / 2;
    
    // Главная дорога (горизонтальная) - Main Street East
    ctx.beginPath();
    ctx.strokeStyle = '#00ff00';
    ctx.lineWidth = 4;
    ctx.moveTo(80, centerY);
    ctx.lineTo(canvas.width - 80, centerY);
    ctx.stroke();
    
    // Вторая горизонтальная дорога - Main Street West
    ctx.beginPath();
    ctx.strokeStyle = '#00ff88';
    ctx.lineWidth = 4;
    ctx.moveTo(80, centerY);
    ctx.lineTo(canvas.width - 80, centerY);
    ctx.stroke();
    
    // Вертикальная дорога - Cross Street
    ctx.beginPath();
    ctx.strokeStyle = '#ffff00';
    ctx.lineWidth = 4;
    ctx.moveTo(centerX, 50);
    ctx.lineTo(centerX, canvas.height - 50);
    ctx.stroke();
    
    // Перекресток
    ctx.fillStyle = '#ffffff';
    ctx.shadowBlur = 5;
    ctx.shadowColor = '#00d4ff';
    ctx.beginPath();
    ctx.arc(centerX, centerY, 10, 0, Math.PI * 2);
    ctx.fill();
    
    // Точка въезда
    ctx.fillStyle = '#00ff00';
    ctx.font = '24px Arial';
    ctx.fillText('🚪', 55, centerY - 5);
    ctx.fillStyle = '#00ff00';
    ctx.font = '12px Arial';
    ctx.fillText('ВЪЕЗД', 50, centerY + 15);
    
    // Точка выезда
    ctx.fillStyle = '#ff0000';
    ctx.font = '24px Arial';
    ctx.fillText('🏁', canvas.width - 85, centerY - 5);
    ctx.fillStyle = '#ff0000';
    ctx.font = '12px Arial';
    ctx.fillText('ВЫЕЗД', canvas.width - 95, centerY + 15);
    
    // Надписи дорог
    ctx.fillStyle = '#00ff00';
    ctx.font = '11px Arial';
    ctx.fillText('Main Street East', centerX + 30, centerY - 15);
    ctx.fillText('Main Street West', centerX + 30, centerY + 25);
    
    ctx.fillStyle = '#ffff00';
    ctx.font = '11px Arial';
    ctx.fillText('Cross Street', centerX - 60, centerY - 40);
    
    // Стрелки направлений
    ctx.fillStyle = '#ffffff';
    ctx.font = '16px Arial';
    ctx.fillText('→', centerX + 20, centerY - 5);
    ctx.fillText('→', centerX + 20, centerY + 20);
    ctx.fillText('↓', centerX - 5, centerY + 40);
    ctx.fillText('↑', centerX - 5, centerY - 30);
    
    ctx.shadowBlur = 0;
}

// Рисование машин
function drawVehicles(vehicles) {
    const centerX = canvas.width / 2;
    const centerY = canvas.height / 2;
    
    for (let v of vehicles) {
        let color = '#00d4ff';
        let borderColor = '#ffffff';
        
        if (v.vehicle_type === 'Truck') {
            color = '#ffa502';
            borderColor = '#ffcc80';
        }
        if (v.vehicle_type === 'Bus') {
            color = '#ff4757';
            borderColor = '#ff9999';
        }
        
        // Конвертируем координаты из 0-100 в пиксели
        let x = (v.x / 100) * canvas.width;
        let y = (v.y / 120) * canvas.height; // y от 10 до 90, max 100-120
        
        ctx.fillStyle = color;
        ctx.shadowBlur = 3;
        ctx.shadowColor = color;
        
        // Рисуем машину в виде прямоугольника с закруглениями
        ctx.beginPath();
        ctx.roundRect(x - 8, y - 5, 16, 10, 3);
        ctx.fill();
        
        // Обводка
        ctx.strokeStyle = borderColor;
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.roundRect(x - 8, y - 5, 16, 10, 3);
        ctx.stroke();
        
        // Окна
        ctx.fillStyle = '#87CEEB';
        ctx.fillRect(x - 5, y - 4, 4, 3);
        ctx.fillRect(x + 1, y - 4, 4, 3);
        
        ctx.fillStyle = '#fff';
        ctx.font = '12px Arial';
        ctx.fillText(getVehicleSymbol(v.vehicle_type), x - 5, y - 8);
    }
    ctx.shadowBlur = 0;
}

// Вспомогательная функция для roundRect
if (!CanvasRenderingContext2D.prototype.roundRect) {
    CanvasRenderingContext2D.prototype.roundRect = function(x, y, w, h, r) {
        if (w < 2 * r) r = w / 2;
        if (h < 2 * r) r = h / 2;
        this.moveTo(x+r, y);
        this.lineTo(x+w-r, y);
        this.quadraticCurveTo(x+w, y, x+w, y+r);
        this.lineTo(x+w, y+h-r);
        this.quadraticCurveTo(x+w, y+h, x+w-r, y+h);
        this.lineTo(x+r, y+h);
        this.quadraticCurveTo(x, y+h, x, y+h-r);
        this.lineTo(x, y+r);
        this.quadraticCurveTo(x, y, x+r, y);
        return this;
    };
}

function getVehicleSymbol(type) {
    switch(type) {
        case 'Car': return '🚗';
        case 'Truck': return '🚚';
        case 'Bus': return '🚌';
        default: return '🚗';
    }
}

// Обновление UI
function updateUI(state) {
    document.getElementById('totalVehicles').textContent = state.total_vehicles;
    document.getElementById('activeVehicles').textContent = state.vehicles.length;
    document.getElementById('avgSpeed').textContent = `${state.statistics.average_speed.toFixed(1)} км/ч`;
    document.getElementById('throughput').textContent = `${state.statistics.throughput.toFixed(0)}/мин`;
    document.getElementById('simTime').textContent = `${state.current_time.toFixed(1)} с`;
    document.getElementById('simSpeed').textContent = `${state.simulation_speed.toFixed(1)}x`;
    document.getElementById('speedValue').textContent = `${state.simulation_speed.toFixed(1)}x`;
    
    // Статус
    const statusDiv = document.getElementById('status');
    if (state.is_running) {
        statusDiv.className = 'status running';
        statusDiv.innerHTML = '▶ ЗАПУЩЕНА';
    } else if (state.is_paused) {
        statusDiv.className = 'status paused';
        statusDiv.innerHTML = '⏸ НА ПАУЗЕ';
    } else {
        statusDiv.className = 'status stopped';
        statusDiv.innerHTML = '⏹ ОСТАНОВЛЕНА';
    }
    
    // Список машин
    const vehicleList = document.getElementById('vehicleList');
    if (state.vehicles.length === 0) {
        vehicleList.innerHTML = '<div style="text-align: center; color: #888;">Нет активных ТС</div>';
    } else {
        vehicleList.innerHTML = state.vehicles.map(v => `
            <div class="vehicle-item">
                <strong>${getVehicleSymbol(v.vehicle_type)} ${v.vehicle_type}</strong> - ${v.current_road}
                <div class="progress-bar">
                    <div class="progress-fill" style="width: ${v.progress}%"></div>
                </div>
                <small>${v.progress.toFixed(0)}% пути</small>
            </div>
        `).join('');
    }
    
    // Рисуем сеть и машины
    drawNetwork();
    drawVehicles(state.vehicles);
}

// Основной цикл
async function gameLoop() {
    if (sim) {
        const stateJson = sim.step();
        const state = JSON.parse(stateJson);
        updateUI(state);
    }
    animationId = requestAnimationFrame(gameLoop);
}

// Инициализация
async function initSimulation() {
    try {
        await init();
        sim = new TrafficSimulation();
        
        // Кнопки управления
        document.getElementById('startBtn').onclick = () => {
            sim.start();
        };
        document.getElementById('pauseBtn').onclick = () => {
            sim.pause();
        };
        document.getElementById('stopBtn').onclick = () => {
            sim.stop();
        };
        document.getElementById('resetBtn').onclick = () => {
            sim.reset();
        };
        document.getElementById('speedUpBtn').onclick = () => {
            const state = JSON.parse(sim.get_state());
            sim.set_speed(state.simulation_speed * 1.5);
        };
        document.getElementById('speedDownBtn').onclick = () => {
            const state = JSON.parse(sim.get_state());
            sim.set_speed(state.simulation_speed / 1.5);
        };
        
        // Сценарии
        document.querySelectorAll('.scenario-btn').forEach(btn => {
            btn.onclick = () => {
                const scenario = parseInt(btn.dataset.scenario);
                sim.load_scenario(scenario);
            };
        });
        
        // Начальное состояние
        const initialState = JSON.parse(sim.get_state());
        updateUI(initialState);
        
        // Запускаем цикл
        gameLoop();
    } catch (error) {
        console.error('Ошибка инициализации:', error);
        document.getElementById('status').innerHTML = '❌ ОШИБКА ЗАГРУЗКИ';
        document.getElementById('status').className = 'status stopped';
    }
}

// Запуск
initSimulation();