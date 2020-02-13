import debug from "debug";
import { Then, TableDefinition } from "cucumber";
import { query } from "../database";
import { expect } from "chai";

const LOG = debug("universe:e2e:data:Loadable");

/**
 * Decorator for Loadable Data to set up the Cucumber steps
 */
export function Loadable(
  name: string,
  loadSql: string,
  rowParser: (row: { [field: string]: any }) => { [field: string]: any }
) {
  LOG("Building Loadable data for name %s", name);

  Then("a user exists with details:", async (dataTable: TableDefinition) => {
    const expected = dataTable.rowsHash();
    LOG("Looking for %s record matching %o", name, expected);
    const rows = await query(loadSql);
    expect(rows.rowCount, `Number of ${name} records`).to.be.at.least(1);
    const records = rows.rows.map(rowParser);
    LOG("Loaded %s records: %o", name, records);

    const found = records.find(record => {
      return Object.keys(expected).every(key => {
        const expectedValue = expected[key];
        const actualValue = record[key];
        LOG(
          "Matching actual value %o to expected value %o for key %s",
          actualValue,
          expectedValue,
          key
        );
        return expectedValue === actualValue;
      });
    });
    LOG("Found matching %s record: %o", name, found);
    expect(found, `Matching ${name} record`).not.to.be.undefined;
  });
}
