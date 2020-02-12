import { Before } from "cucumber";
import { Pool } from "pg";
import debug from "debug";

const LOG = debug("universe:e2e:browser");

/** The raw database connection */
let _pool: Pool | undefined;

/**
 * Get the Web Driver, creating a new one if needed
 * @return The Web Driver
 */
function getDatabaseConnection() {
  if (_pool === undefined) {
    const url = process.env.DATABASE_URL;
    LOG("Creating new Database Connection: %s", url);
    _pool = new Pool({
      connectionString: url
    });
  }

  return _pool;
}

/**
 * Actually execute a query against the database
 *
 * @param sql The SQL to execute
 * @param binds Any binds for the SQL
 */
async function query(sql: string, binds?: any[]) {
  const pool = getDatabaseConnection();
  LOG("Executing query: %s with binds: %o", sql, binds);
  return await pool.query(sql, binds);
}

Before(async () => {
  const result = await query(
    "SELECT table_name FROM information_schema.tables WHERE table_schema=$1",
    ["public"]
  );

  const tables = result.rows
    .map(row => row.table_name)
    .filter(table => table !== "__migrations");
  LOG("Tables to truncate: %o", tables);
  if (tables.length > 0) {
    await query("TRUNCATE " + tables.join(","));
  }
});
