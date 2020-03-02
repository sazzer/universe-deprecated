/**
 * Representation of a validation error as returned by the API
 */
export interface ValidationError {
  /** The type of validation error */
  type: string;
  /** The field that was in error */
  field: string;
}

/**
 * Wrapper around validation errors from the API
 */
export class ValidationErrors {
  /** The actual errors */
  readonly errors: ValidationError[];

  /**
   * Constructor
   * @param errors The actual errors
   */
  constructor(errors: ValidationError[]) {
    this.errors = errors;
  }

  /**
   * Get all of the errors for a single field
   * @param field the field to get the errors for
   * @return the errors for this field
   */
  errorsForField(field: string): ValidationError[] {
    return this.errors.filter(error => error.field === field);
  }
}
