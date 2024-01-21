const { invoke } = require('@tauri-apps/api/tauri');

// Placeholder for initializing the clock view
function initClockView() {
    const clockElement = document.getElementById('clock-view');
    // Initialize the SVG or Canvas for the clock here
}

// Placeholder function for adding a task to the list
function addTask(taskContent) {
    invoke('add_new_task', { taskDescription: taskContent })
        .then(response => {
            const taskList = document.getElementById('task-list');
            const taskItem = document.createElement('li');
            taskItem.className = 'task-item';
            taskItem.textContent = taskContent;
            taskList.appendChild(taskItem);
        })
        .catch(error => {
            // Handle the error
        });
}

// Placeholder function for shuffling the order of tasks
function shuffleTasks() {
    const taskList = document.getElementById('task-list');
    for (let i = taskList.children.length; i >= 0; i--) {
        taskList.appendChild(taskList.children[Math.random() * i | 0]);
    }
}

// Event listeners for buttons
document.getElementById('add-task').addEventListener('click', () => {
    // Implement the logic to add a new task
    addTask(`New Task ${Math.floor(Math.random() * 100)}`);
});

document.getElementById('shuffle').addEventListener('click', () => {
    // Implement the logic to shuffle tasks
    shuffleTasks();
});

// Initial setup calls
initClockView();
