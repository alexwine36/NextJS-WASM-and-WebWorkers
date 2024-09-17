const { resolve } = require('node:path');

const project = resolve(process.cwd(), 'tsconfig.json');
const base = require('./base.js');

/** @type {import("eslint").Linter.Config} */
module.exports = {
  extends: ['./base.js'],
};
