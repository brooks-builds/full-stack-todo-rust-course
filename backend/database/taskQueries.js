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

module.exports = {
  insertTask,
  getAllUsersTasks
}