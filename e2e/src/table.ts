/**
 * Process a single value, performing substitutions as needed
 * @param  input The input to process
 * @return       The result of processing the input
 */
export function processValue(input: string): string {
  let result = input.replace(/{space}/g, ' ');
  return result;
}

/**
 * Process an object, replacing all of the values with processed ones
 * @param  input The object to process
 * @return       The result of processing all the values
 */
export function processObject(input: { [key: string]: string }): { [key: string]: string } {
  const result: { [key: string]: string } = {};
  Object.keys(input).forEach(key => {
    const inputValue = input[key];
    if (inputValue) {
      result[key] = processValue(inputValue);
    }
  });
  return result;
}
