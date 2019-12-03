use log::info;
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use speculate::speculate;
use testcontainers::*;

speculate! {
    before {
        let _ = env_logger::try_init();

        let docker = clients::Cli::default();
        let node = docker.run(images::postgres::Postgres::default());

        let host_port = node.get_host_port(5432).unwrap();
        let url = format!("postgres://postgres:postgres@localhost:{}", host_port);
        info!("Running postgres on {}", url);

        let manager = PostgresConnectionManager::new(url, TlsMode::None).unwrap();
        let pool = Pool::new(manager).unwrap();

        let conn = pool.get().unwrap();
    }

    describe "new" {
        test "Empty migrations directory" {
            let result = universe_migrations::Migrations::new(conn, "tests/migrations/empty");
            assert!(result.is_ok());
        }
        test "Populated migrations directory" {
            let result = universe_migrations::Migrations::new(conn, "tests/migrations/working");
            assert!(result.is_ok());
        }
        test "Unknown migrations directory" {
            let result = universe_migrations::Migrations::new(conn, "tests/migrations/unknown");
            assert!(result.is_err());
            assert_eq!(Some(universe_migrations::MigrationsError::UnknownDirectory), result.err());
        }
    }

    describe "migrate" {
        test "Empty migrations directory" {
            let result = universe_migrations::Migrations::new(conn, "tests/migrations/empty").unwrap();

            let applied = result.migrate();
            assert_eq!(Some(0), applied.ok());

            let new_conn = pool.get().unwrap();
            let applied_migrations: Vec<String> = new_conn
                .query("SELECT migration_file FROM __migrations ORDER BY sequence ASC", &[])
                .unwrap()
                .iter()
                .map(|row| {
                    let value: String = row.get("migration_file");
                    value
                })
                .collect::<Vec<String>>();
            assert_eq!(0, applied_migrations.len());
        }
        test "Populated migrations directory - single migration" {
            let result = universe_migrations::Migrations::new(conn, "tests/migrations/working").unwrap();

            let applied = result.migrate();
            assert_eq!(Some(1), applied.ok());

            let new_conn = pool.get().unwrap();
            let applied_migrations: Vec<String> = new_conn
                .query("SELECT migration_file FROM __migrations ORDER BY sequence ASC", &[])
                .unwrap()
                .iter()
                .map(|row| {
                    let value: String = row.get("migration_file");
                    value
                })
                .collect::<Vec<String>>();
            assert_eq!(1, applied_migrations.len());
            assert_eq!(Some(&"tests/migrations/working/0001_table.sql".to_owned()), applied_migrations.get(0));

            // If the table doesn't exist then this panics. That means that the migration didn't run.
            let rows = new_conn.query("SELECT * FROM people", &[]).unwrap();
            assert_eq!(0, rows.len());
        }
        test "Populated migrations directory - multiple migrations" {
            let result = universe_migrations::Migrations::new(conn, "tests/migrations/multiple").unwrap();

            let applied = result.migrate();
            assert_eq!(Some(2), applied.ok());

            let new_conn = pool.get().unwrap();
            let applied_migrations: Vec<String> = new_conn
                .query("SELECT migration_file FROM __migrations ORDER BY sequence ASC", &[])
                .unwrap()
                .iter()
                .map(|row| {
                    let value: String = row.get("migration_file");
                    value
                })
                .collect::<Vec<String>>();
            assert_eq!(2, applied_migrations.len());
            assert_eq!(Some(&"tests/migrations/multiple/0001_table.sql".to_owned()), applied_migrations.get(0));
            assert_eq!(Some(&"tests/migrations/multiple/0002_data.sql".to_owned()), applied_migrations.get(1));

            // If the table doesn't exist then this panics. That means that the migration didn't run.
            let rows = new_conn.query("SELECT * FROM people", &[]).unwrap();
            assert_eq!(1, rows.len());
        }
        test "Populated migrations directory - invalid migrations" {
            let result = universe_migrations::Migrations::new(conn, "tests/migrations/invalid").unwrap();

            let applied = result.migrate();
            assert_eq!(
                Some(universe_migrations::MigrationsError::MigrationError("database error: ERROR: relation \"peoples\" does not exist".to_owned())),
                applied.err());

            let new_conn = pool.get().unwrap();
            let applied_migrations = new_conn
                .query("SELECT migration_file FROM __migrations ORDER BY sequence ASC", &[]);
            assert!(applied_migrations.is_err());
        }
        test "Populated migrations directory - multiple migrations - run twice" {
            let result = universe_migrations::Migrations::new(conn, "tests/migrations/multiple").unwrap();

            let applied = result.migrate();
            assert_eq!(Some(2), applied.ok());

            let reapplied = result.migrate();
            assert_eq!(Some(0), reapplied.ok());

            let new_conn = pool.get().unwrap();
            let applied_migrations: Vec<String> = new_conn
                .query("SELECT migration_file FROM __migrations ORDER BY sequence ASC", &[])
                .unwrap()
                .iter()
                .map(|row| {
                    let value: String = row.get("migration_file");
                    value
                })
                .collect::<Vec<String>>();
            assert_eq!(2, applied_migrations.len());
            assert_eq!(Some(&"tests/migrations/multiple/0001_table.sql".to_owned()), applied_migrations.get(0));
            assert_eq!(Some(&"tests/migrations/multiple/0002_data.sql".to_owned()), applied_migrations.get(1));

            // If the table doesn't exist then this panics. That means that the migration didn't run.
            let rows = new_conn.query("SELECT * FROM people", &[]).unwrap();
            assert_eq!(1, rows.len());
        }
    }
}
