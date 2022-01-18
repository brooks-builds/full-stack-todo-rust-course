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

module.exports = {
  insertTask,
}