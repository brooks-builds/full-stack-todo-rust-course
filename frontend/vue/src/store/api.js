import axios from 'axios';

const baseUrl = 'http://localhost:3000/api/v1';

export async function createAccount(newAccount) {
  try {
    const result = await axios.post(`${baseUrl}/users`, newAccount);
    return result.data;
  } catch(error) {
    console.error("error creating new account", error);
    throw error;
  }
}