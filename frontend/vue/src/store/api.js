import axios from 'axios';

const baseUrl = `${process.env.VUE_APP_API_URI}/api/v1`;
console.log(baseUrl)

export async function createAccount(newAccount) {
  try {
    const result = await axios.post(`${baseUrl}/users`, newAccount);
    return result.data;
  } catch(error) {
    console.error("error creating new account", error);
    throw error;
  }
}