const { invoke } = window.__TAURI__.tauri;

let currentTime;
let nextTask;
let allTasks;
let newTaskTitle;
let newTaskDescription;

async function update_current_time() {
  currentTime.textContent = await invoke("update_current_time");
}

async function show_next_task() {
  nextTask.textContent = await invoke("show_next_task", { name: nextTask.value });
}

async function fetchTasks() {
  try {
    let tasks = await invoke('list_all_tasks');
    console.log(tasks[0]);

    renderTable(tasks);
  } catch (error) {
    console.error('Error fetching tasks:', error);
  }
}

function renderTable(tasks) {
  const tableBody = document.getElementById('task-table-body');

  tasks.forEach((task) => {
    const row = document.createElement('tr');
    const idCell = document.createElement('td');
    const titleCell = document.createElement('td');
    const descriptionCell = document.createElement('td');

    idCell.textContent = task.id;
    titleCell.textContent = task.title;
    descriptionCell.textContent = task.description;

    row.appendChild(idCell);
    row.appendChild(titleCell);
    row.appendChild(descriptionCell);
    tableBody.appendChild(row);
  });
}

async function addNewTask() {
  try {
    let response = await invoke("add_new_task", {title: newTaskTitle.value, description: newTaskDescription.value});
  } catch (error) {
    console.log(error);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  newTaskTitle = document.querySelector("#new-task-title-input");
  newTaskDescription = document.querySelector("#new-task-description-input");

  currentTime = document.querySelector("#current-time");
  nextTask = document.querySelector("#next-task");
  allTasks = document.querySelector("#task-list");

  document.querySelector("#add-new-task-form").addEventListener("submit", (e) => {
    e.preventDefault();
    addNewTask();
  });

  // show_next_task();
  fetchTasks();

  setInterval(update_current_time, 1000);
});
