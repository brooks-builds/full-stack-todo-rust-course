// Update with your config settings.

console.log(process.env.DB_CONNECTION)

module.exports = {
  client: 'postgresql',
  connection: process.env.DB_CONNECTION
};
