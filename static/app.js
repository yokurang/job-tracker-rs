// API Configuration
const API_BASE_URL = 'http://localhost:3000/api';

// State
let currentTask = null;
let currentYear = new Date().getFullYear();
let currentMonth = new Date().getMonth() + 1;

// DOM Elements
const taskForm = document.getElementById('task-form');
const taskFormContainer = document.getElementById('task-form-container');
const newTaskBtn = document.getElementById('new-task-btn');
const cancelFormBtn = document.getElementById('cancel-form');
const taskList = document.getElementById('task-list');
const calendarGrid = document.getElementById('calendar-grid');
const calendarTitle = document.getElementById('calendar-title');
const prevMonthBtn = document.getElementById('prev-month');
const nextMonthBtn = document.getElementById('next-month');
const taskFrequencySelect = document.getElementById('task-frequency');
const recurrenceGroup = document.getElementById('recurrence-group');

// Event Listeners
newTaskBtn.addEventListener('click', () => showTaskForm());
cancelFormBtn.addEventListener('click', () => hideTaskForm());
taskForm.addEventListener('submit', handleTaskSubmit);
prevMonthBtn.addEventListener('click', () => changeMonth(-1));
nextMonthBtn.addEventListener('click', () => changeMonth(1));
taskFrequencySelect.addEventListener('change', handleFrequencyChange);

// API Functions
async function fetchTasks() {
    try {
        const response = await fetch(`${API_BASE_URL}/tasks`);
        if (!response.ok) throw new Error('Failed to fetch tasks');
        return await response.json();
    } catch (error) {
        console.error('Error fetching tasks:', error);
        return [];
    }
}

async function createTask(taskData) {
    try {
        const response = await fetch(`${API_BASE_URL}/tasks`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(taskData)
        });
        if (!response.ok) throw new Error('Failed to create task');
        return await response.json();
    } catch (error) {
        console.error('Error creating task:', error);
        throw error;
    }
}

async function updateTask(id, taskData) {
    try {
        const response = await fetch(`${API_BASE_URL}/tasks/${id}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(taskData)
        });
        if (!response.ok) throw new Error('Failed to update task');
        return await response.json();
    } catch (error) {
        console.error('Error updating task:', error);
        throw error;
    }
}

async function deleteTask(id) {
    try {
        const response = await fetch(`${API_BASE_URL}/tasks/${id}`, {
            method: 'DELETE'
        });
        if (!response.ok) throw new Error('Failed to delete task');
    } catch (error) {
        console.error('Error deleting task:', error);
        throw error;
    }
}

async function fetchCalendar(year, month) {
    try {
        const response = await fetch(`${API_BASE_URL}/calendar/${year}/${month}`);
        if (!response.ok) throw new Error('Failed to fetch calendar');
        return await response.json();
    } catch (error) {
        console.error('Error fetching calendar:', error);
        return null;
    }
}

// UI Functions
function showTaskForm(task = null) {
    currentTask = task;
    const formTitle = document.getElementById('form-title');

    if (task) {
        formTitle.textContent = 'Edit Task';
        document.getElementById('task-name').value = task.name;
        document.getElementById('task-description').value = task.description || '';

        if (task.due_date) {
            const date = new Date(task.due_date);
            document.getElementById('task-due-date').value = date.toISOString().slice(0, 16);
        }

        document.getElementById('task-frequency').value = task.frequency;

        if (task.recurrence_date) {
            const date = new Date(task.recurrence_date);
            document.getElementById('task-recurrence-date').value = date.toISOString().slice(0, 16);
        }

        handleFrequencyChange();
    } else {
        formTitle.textContent = 'Create New Task';
        taskForm.reset();
    }

    taskFormContainer.classList.remove('hidden');
}

function hideTaskForm() {
    taskFormContainer.classList.add('hidden');
    currentTask = null;
    taskForm.reset();
}

function handleFrequencyChange() {
    const frequency = taskFrequencySelect.value;
    recurrenceGroup.style.display = frequency !== 'None' ? 'block' : 'none';
}

async function handleTaskSubmit(e) {
    e.preventDefault();

    const formData = new FormData(taskForm);
    const taskData = {
        name: formData.get('name'),
        description: formData.get('description') || null,
        frequency: formData.get('frequency')
    };

    // Handle dates
    const dueDate = formData.get('due_date');
    if (dueDate) {
        taskData.due_date = new Date(dueDate).toISOString();
    }

    const recurrenceDate = formData.get('recurrence_date');
    if (recurrenceDate && taskData.frequency !== 'None') {
        taskData.recurrence_date = new Date(recurrenceDate).toISOString();
    }

    try {
        if (currentTask) {
            await updateTask(currentTask.id, taskData);
        } else {
            await createTask(taskData);
        }

        hideTaskForm();
        await loadTasks();
        await loadCalendar();
    } catch (error) {
        alert('Failed to save task. Please try again.');
    }
}

async function handleStatusChange(taskId, newStatus) {
    try {
        await updateTask(taskId, { status: newStatus });
        await loadTasks();
        await loadCalendar();
    } catch (error) {
        alert('Failed to update task status.');
    }
}

async function handleDeleteTask(taskId) {
    if (!confirm('Are you sure you want to delete this task?')) return;

    try {
        await deleteTask(taskId);
        await loadTasks();
        await loadCalendar();
    } catch (error) {
        alert('Failed to delete task.');
    }
}

// Render Functions
async function loadTasks() {
    const tasks = await fetchTasks();

    if (tasks.length === 0) {
        taskList.innerHTML = '<div class="empty-state">No tasks yet. Create your first task!</div>';
        return;
    }

    taskList.innerHTML = tasks.map(task => `
        <div class="task-item">
            <div class="task-header">
                <div class="task-content">
                    <div class="task-name">${escapeHtml(task.name)}</div>
                    ${task.description ? `<div class="task-description">${escapeHtml(task.description)}</div>` : ''}
                    <div class="task-meta">
                        ${task.due_date ? `<span>Due: ${formatDate(task.due_date)}</span>` : ''}
                        <span class="task-status status-${task.status.toLowerCase()}">${formatStatus(task.status)}</span>
                        ${task.frequency !== 'None' ? `<span class="frequency-badge">🔁 ${task.frequency}</span>` : ''}
                    </div>
                </div>
                <div class="task-actions">
                    <select onchange="handleStatusChange('${task.id}', this.value)">
                        <option value="Pending" ${task.status === 'Pending' ? 'selected' : ''}>Pending</option>
                        <option value="InProgress" ${task.status === 'InProgress' ? 'selected' : ''}>In Progress</option>
                        <option value="Completed" ${task.status === 'Completed' ? 'selected' : ''}>Completed</option>
                        <option value="Cancelled" ${task.status === 'Cancelled' ? 'selected' : ''}>Cancelled</option>
                    </select>
                    <button class="btn btn-secondary btn-small" onclick="editTask('${task.id}')">Edit</button>
                    <button class="btn btn-secondary btn-small" onclick="handleDeleteTask('${task.id}')">Delete</button>
                </div>
            </div>
        </div>
    `).join('');
}

async function loadCalendar() {
    const calendarData = await fetchCalendar(currentYear, currentMonth);
    if (!calendarData) return;

    // Update title
    const monthNames = ['January', 'February', 'March', 'April', 'May', 'June',
        'July', 'August', 'September', 'October', 'November', 'December'];
    calendarTitle.textContent = `${monthNames[currentMonth - 1]} ${currentYear}`;

    // Build calendar grid
    let html = '';

    // Day headers
    const dayHeaders = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    dayHeaders.forEach(day => {
        html += `<div class="calendar-day-header">${day}</div>`;
    });

    // Calendar days
    calendarData.weeks.forEach(week => {
        week.days.forEach(day => {
            const classes = ['calendar-day'];
            if (day.is_today) classes.push('is-today');
            if (day.is_current_week && day.is_current_month) classes.push('is-current-week');
            if (!day.is_current_month) classes.push('is-other-month');

            html += `
                <div class="${classes.join(' ')}">
                    <div class="calendar-day-number">${day.date.split('-')[2]}</div>
                    <div class="calendar-day-tasks">
                        ${day.tasks.slice(0, 3).map(task => `
                            <div class="calendar-task" onclick="editTask('${task.id}')">${escapeHtml(task.name)}</div>
                        `).join('')}
                        ${day.tasks.length > 3 ? `<div class="calendar-more-tasks">+${day.tasks.length - 3} more</div>` : ''}
                    </div>
                </div>
            `;
        });
    });

    calendarGrid.innerHTML = html;
}

function changeMonth(delta) {
    currentMonth += delta;
    if (currentMonth > 12) {
        currentMonth = 1;
        currentYear++;
    }
}