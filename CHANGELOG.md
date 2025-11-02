## [0.2.0] - 2025-11-02

### 🚀 Features

- Use dedicated error type

### 📚 Documentation

- Improve README

### ⚙️ Miscellaneous Tasks

- Bump version to 0.2.0
- Add CHANGELOG.md
## [0.1.0] - 2025-10-30

### 🚀 Features

- Add Display impl to Epc (#2)
- Improve API
- Make builder easier to discover on the Epc struct
- Implement more verifications and tests
- Add iban module for validation (#4)
- Add rf validation

### 🐛 Bug Fixes

- Model remittance correctly
- Use string for amount and validate bounds

### 📚 Documentation

- Add example and README.md
- Rename example png
- Improve API docs

### ⚙️ Miscellaneous Tasks

- Add woodpecker.yaml
- Update .gitignore
- Move iban mod to dedicated file
- Move root epc mod to dedicated file and reexport
- Improve woodpecker ci
- Fix duplicate runs of ci pipelines
- Add package information to Cargo.toml
