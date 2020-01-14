const { Pool } = require('pg');
const { Before, AfterAll } = require('cucumber');

let thePool;

/**
 * Get a connection to the database
 */
function getClient() {
  if (thePool === undefined) {
    const url = process.env.DATABASE_URL;
    thePool = new Pool({ connectionString: url });
  }
  return thePool;
}

/**
 * Seed the database with somedata
 * @param  {array|object} data The data to seed the database with
 */
async function seed(data) {
  if (Array.isArray(data)) {
    data.forEach(seed);
  } else {
    const sql = data.sql;
    const binds = data.binds || [];

    if (sql) {
      const client = getClient();
      await client.query(sql, binds);
    }
  }
}

Before(async function() {
  const client = getClient();
  const result = await client.query('SELECT table_name FROM information_schema.tables WHERE table_schema = $1', ['public']);
  const tableNames = result.rows.map(row => row.table_name).join(',');

  const truncateSql = 'TRUNCATE ' + tableNames;
  await client.query(truncateSql);
});

AfterAll(async function() {
  await getClient().end();
});

module.exports = { seed };
