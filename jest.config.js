const path = require("path");
const fs = require("fs");

// run jest on all packages
const packagesDir = path.join(__dirname, "packages");
const packageNames = fs
  .readdirSync(packagesDir)
  .filter((file) => fs.lstatSync(path.join(packagesDir, file)).isDirectory());
const projects = packageNames.map((displayName) => ({
  preset: "ts-jest",
  testEnvironment: "node",
  displayName,
  testMatch: [`<rootDir>/packages/${displayName}/**/*.test.ts`],
}));

module.exports = {
  verbose: true,
  projects,
};
