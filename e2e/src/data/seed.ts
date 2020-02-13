import debug from "debug";
import { Given, TableDefinition } from "cucumber";
import { query } from "../database";

const LOG = debug("universe:e2e:data:seed");

/**
 * Interface for all Seed Data types to extend
 */
export interface SeedData {
  /** The SQL to insert data into the database */
  readonly sql: string;
  /** The binds for the SQL */
  readonly binds: any[];
}

/**
 * Constructor for a Seed Data type
 */
export interface SeedDataConstructor<T extends SeedData> {
  new (data: { [key: string]: string }): T;
}

/**
 * Decorator for Seed Data to set up the Cucumber steps
 */
export function Seed(name: string) {
  return function<T extends SeedData>(constructor: SeedDataConstructor<T>) {
    LOG("Building seed data for name %s and type: %o", name, constructor);

    Given(
      `a ${name} already exists with details:`,
      async (dataTable: TableDefinition) => {
        const data = dataTable.rowsHash();
        LOG("Seeding %s with data: %o", name, data);
        const seedData = new constructor(data);
        await query(seedData.sql, seedData.binds);
      }
    );
  };
}

/**
 * Helper to extract some data from an input map, supporting defaults and converstions
 * @param input the input map
 * @param field the field in the map
 * @param fallback the fallback value
 * @param converter the optional converter
 */
export function extractData<T>(
  input: { [key: string]: string },
  field: string,
  fallback: T,
  converter?: (input: string) => T
): T {
  const inputField: unknown = input[field];
  if (inputField === undefined) {
    return fallback;
  } else if (converter === undefined) {
    return inputField as T;
  } else {
    return converter(inputField as string);
  }
}
