import axios from 'axios';

const baseUrl = `${process.env.VUE_APP_API_URI}/api/v1`;

export async function createAccount(newAccount) {
  try {
    const result = await axios.post(`${baseUrl}/users`, newAccount);
    return result.data;
  } catch(error) {
    console.error("error creating new account", error);
    throw error;
  }
}

export async function login(account) {
  try {
    const result = await axios.post(`${baseUrl}/users/login`, account);
    return result.data;
  } catch(error) {
    console.error("Error logging in", error);
    throw new Error(error.response.data.error);
  }
}

export async function getTasks(token) {
  try {
    const result = await axios.get(`${baseUrl}/tasks`, createHeaders(token));
    return result.data.data;
  } catch(error) {
    if(error.response.status == 401) {
      const newError = new Error("Not authenticated, please log in and try again");
      newError.code = 401;
      throw newError
    } else {
      throw error;
    }
  }
}

export async function updateTask(task, token) {
  try {
    const result = await axios.patch(`${baseUrl}/tasks/${task.id}`, task, createHeaders(token));
    return result.data.data;
  } catch(error) {
    console.error("Error updating task", error);
    throw error;
  }
}

export async function createTask(task, token) {
  try {
    const result = await axios.post(`${baseUrl}/tasks`, task, createHeaders(token));
    return result.data.data;
  } catch(error) {
    console.error("Error creating task", error);
    throw error;
  }
}

export async function completeTask(taskId, token) {
  try {
    await axios.put(`${baseUrl}/tasks/${taskId}/completed`, {}, createHeaders(token));
  } catch(error) {
    console.error("Error completing a task", error);
    throw error;
  }
}

export async function unCompleteTask(taskId, token) {
  try {
    await axios.put(`${baseUrl}/tasks/${taskId}/uncompleted`, {}, createHeaders(token));
  } catch(error) {
    console.error("Error uncompleting a task", error);
    throw error;
  }
}

export async function logout(token) {
  try {
    await axios.post(`${baseUrl}/users/logout`, {}, createHeaders(token));
  } catch(error) {
    console.error("Error logging out", error);
    throw error;
  }
}

export async function deleteTask(taskId, token) {
  try {
    await axios.delete(`${baseUrl}/tasks/${taskId}`, createHeaders(token));
  } catch(error) {
    console.error("Error deleting task", error);
    throw error;
  }
}

function createHeaders(token) {
  return {headers: {"x-auth-token": token}}
}