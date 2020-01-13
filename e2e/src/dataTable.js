const transformations = [
  (value) => value.replace(/\[space]/g, ' '),
];

function parseValue(value) {
  return transformations.reduce((accumulator, current) => {
    return current(accumulator);
  }, value);
}

module.exports = { parseValue };
