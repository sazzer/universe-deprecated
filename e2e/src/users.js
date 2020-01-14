const { Given } = require('cucumber');
const { v4 } = require('uuid');
const { seed } = require('./database');

class UserSeed {
  constructor(data) {
    this._data = Object.assign({
      ID: v4(),
      Version: v4(),
      Created: new Date(),
      Updated: new Date(),
      Username: 'testuser',
      ['Email Address']: 'test@example.com',
      ['Display Name']: 'Test User',
      Password: '',
    }, data);
  }

  get sql() {
    return 'INSERT INTO users(user_id, version, created, updated, username, email, display_name, password) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)';
  }

  get binds() {
    return [
      this._data.ID,
      this._data.Version,
      this._data.Created,
      this._data.Updated,
      this._data.Username,
      this._data['Email Address'],
      this._data['Display Name'],
      this._data.Password,
    ]
  }
}

Given('a user exists with details:', async function(dataTable) {
  const user = new UserSeed(dataTable.rowsHash());
  await seed(user);
});
