// Путь к WASM модулю (правильный относительный путь)
import init, { TrafficSimulation } from '../pkg/traffic_simulation.js';

let sim = null;
let animationId = null;

// Ждем загрузки страницы
document.addEventListener('DOMContentLoaded', async () => {
    try {
        console.log('Загрузка WASM модуля...');
        
        // Инициализация WASM
        await init();
        console.log('WASM загружен успешно');
        
        // Создание экземпляра симуляции
        sim = new TrafficSimulation();
        console.log('Симуляция создана');
        
        // Получаем элементы
        const canvas = document.getElementById('roadCanvas');
        const ctx = canvas.getContext('2d');
        
        // Настройка canvas
        canvas.width = 600;
        canvas.height = 500;
        
        // Рисование дорожной сети
        function drawNetwork() {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            
            // Задний фон
            ctx.fillStyle = '#0a0a1a';
            ctx.fillRect(0, 0, canvas.width, canvas.height);
            
            const centerX = canvas.width / 2;
            const centerY = canvas.height / 2;
            
            // Горизонтальная дорога
            ctx.beginPath();
            ctx.strokeStyle = '#00ff00';
            ctx.lineWidth = 4;
            ctx.moveTo(80, centerY);
            ctx.lineTo(canvas.width - 80, centerY);
            ctx.stroke();
            
            // Вертикальная дорога
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
            
            // Надписи
            ctx.fillStyle = '#00ff00';
            ctx.font = '11px Arial';
            ctx.fillText('Main Street East', centerX + 30, centerY - 15);
            ctx.fillText('Main Street West', centerX + 30, centerY + 25);
            
            ctx.fillStyle = '#ffff00';
            ctx.font = '11px Arial';
            ctx.fillText('Cross Street', centerX - 60, centerY - 40);
            
            ctx.shadowBlur = 0;
        }
        
        // Рисование машин
        function drawVehicles(vehicles) {
            for (let v of vehicles) {
                let color = '#00d4ff';
                if (v.vehicle_type === 'Truck') color = '#ffa502';
                if (v.vehicle_type === 'Bus') color = '#ff4757';
                
                let x = (v.x / 100) * canvas.width;
                let y = (v.y / 100) * canvas.height;
                
                ctx.fillStyle = color;
                ctx.shadowBlur = 3;
                ctx.shadowColor = color;
                ctx.beginPath();
                ctx.arc(x, y, 6, 0, Math.PI * 2);
                ctx.fill();
                
                ctx.fillStyle = '#fff';
                ctx.font = '12px Arial';
                ctx.fillText(getVehicleSymbol(v.vehicle_type), x - 5, y - 8);
            }
            ctx.shadowBlur = 0;
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
            document.getElementById('avgSpeed').textContent = `${state.avg_speed.toFixed(1)} км/ч`;
            document.getElementById('throughput').textContent = `${state.throughput.toFixed(0)}/мин`;
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
        function gameLoop() {
            if (sim) {
                const stateJson = sim.step();
                const state = JSON.parse(stateJson);
                updateUI(state);
            }
            animationId = requestAnimationFrame(gameLoop);
        }
        
        // Кнопки управления
        document.getElementById('startBtn').onclick = () => {
            console.log('Start clicked');
            sim.start();
        };
        document.getElementById('pauseBtn').onclick = () => {
            console.log('Pause clicked');
            sim.pause();
        };
        document.getElementById('stopBtn').onclick = () => {
            console.log('Stop clicked');
            sim.stop();
        };
        document.getElementById('resetBtn').onclick = () => {
            console.log('Reset clicked');
            sim.reset();
        };
        document.getElementById('speedUpBtn').onclick = () => {
            console.log('Speed Up clicked');
            const state = JSON.parse(sim.get_state_json());
            sim.set_speed(state.simulation_speed * 1.5);
        };
        document.getElementById('speedDownBtn').onclick = () => {
            console.log('Speed Down clicked');
            const state = JSON.parse(sim.get_state_json());
            sim.set_speed(state.simulation_speed / 1.5);
        };
        
        // Сценарии
        document.querySelectorAll('.scenario-btn').forEach(btn => {
            btn.onclick = () => {
                const scenario = parseInt(btn.dataset.scenario);
                console.log(`Scenario ${scenario} clicked`);
                sim.load_scenario(scenario);
            };
        });
        
        // Начальное состояние
        const initialState = JSON.parse(sim.get_state_json());
        updateUI(initialState);
        
        // Запускаем цикл
        gameLoop();
        
        console.log('Приложение запущено!');
    } catch (error) {
        console.error('Ошибка инициализации:', error);
        document.getElementById('status').innerHTML = '❌ ОШИБКА ЗАГРУЗКИ';
        document.getElementById('status').className = 'status stopped';
    }
});