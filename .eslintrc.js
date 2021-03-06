module.exports = {
  "env": {
    "browser": true,
    "es6": true
  },
  "extends": [
    "standard",
    "plugin:mocha/recommended"
  ],
  "globals": {
    "Atomics": "readonly",
    "SharedArrayBuffer": "readonly"
  },
  "plugins": [
    "mocha"
  ],
  "parserOptions": {
    "ecmaVersion": 2018,
    "sourceType": "module"
  },
  "rules": {
    "quotes": [2, "double" {"avoidEscape": true}]
  }
};
