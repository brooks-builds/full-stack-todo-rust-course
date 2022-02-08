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
    throw error;
  }
}

export async function getTasks(token) {
  try {
    const result = await axios.get(`${baseUrl}/tasks`, {headers: {"x-auth-token": token}});
    return result.data.data;
  } catch(error) {
    console.error("error getting all tasks", error);
    throw error
  }
}