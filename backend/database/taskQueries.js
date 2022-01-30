const db = require("./index");
const dbTasks = db("tasks");

async function insertTask(priority = null, title, completed_at = false, description = null, userId) {
  return (await dbTasks.insert({
    priority,
    title, completed_at,
    description,
    user_id: userId
  }).returning(["id", "priority", "title", "completed_at", "description"]))[0];
}

async function getAllUsersTasks(userId) {
  const tasks = await db.select().from("tasks").where({user_id: userId, deleted_at: null});
  return tasks;
}

function getOneUsersTask(userId, taskId) {
  return db.select().from("tasks").where({user_id: userId, deleted_at: null, id: taskId}).first();
}

function markTaskAsCompleted(userId, taskId) {
  const now = new Date();
  return updateCompletedStatus(userId, taskId, now.toUTCString());
}

function markTaskAsUncompleted(userId, taskId) {
  return updateCompletedStatus(userId, taskId);
}

function updateCompletedStatus(user_id, id, completed_at = null) {
  return db("tasks").update({completed_at}).where({user_id, deleted_at: null, id});
}

function updateTask(userId, taskId, task) {
  return db("tasks").update(task).where({id: taskId, user_id: userId, deleted_at: null});
}

module.exports = {
  insertTask,
  getAllUsersTasks,
  getOneUsersTask,
  markTaskAsCompleted,
  markTaskAsUncompleted,
  updateTask
}