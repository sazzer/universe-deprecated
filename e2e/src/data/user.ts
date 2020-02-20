import { Seed, extractData } from "./seed";
import { Loadable } from "./load";
import uuid from "uuid/v4";
import * as bcrypt from "bcrypt";

/**
 * Seed Data for a User
 */
@Seed("user")
export class UserSeed {
  private _id: string;
  private _version: string;
  private _created: Date;
  private _updated: Date;
  private _username: string;
  private _email: string;
  private _displayName: string;
  private _password: string;

  constructor(data: { [key: string]: string }) {
    this._id = extractData(data, "User ID", uuid());
    this._version = extractData(data, "Version", uuid());
    this._created = extractData(
      data,
      "Created",
      new Date(),
      input => new Date(input)
    );
    this._updated = extractData(
      data,
      "Updated",
      new Date(),
      input => new Date(input)
    );

    this._username = extractData(data, "Username", "testuser");
    this._email = extractData(data, "Email Address", "testuser@example.com");
    this._displayName = extractData(data, "Display Name", "Test User");
    this._password = extractData(data, "Password", "Password");
  }

  get sql() {
    return "INSERT INTO users(user_id, version, created, updated, username, email, display_name, password) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";
  }

  get binds() {
    const hash = bcrypt.hashSync(this._password, 10);

    return [
      this._id,
      this._version,
      this._created,
      this._updated,
      this._username,
      this._email,
      this._displayName,
      hash
    ];
  }
}

Loadable("user", "SELECT * FROM users", (input: { [field: string]: any }) => {
  return {
    "User ID": input.user_id,
    Version: input.version,
    Created: input.created,
    Updated: input.updated,
    Username: input.username,
    "Email Address": input.email,
    "Display Name": input.display_name
  };
});
