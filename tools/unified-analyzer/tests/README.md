# Unified Analyzer Test Suite

This directory contains the test suite for the Unified Analyzer. The test suite includes unit tests and integration tests to ensure that the Unified Analyzer works correctly.

## Test Structure

The test suite is organized as follows:

```
tests/
├── main.rs                  # Test runner
├── test_utils.rs            # Test utilities
├── README.md                # This file
├── unit/                    # Unit tests
│   ├── config_test.rs       # Tests for the configuration module
│   └── generators/          # Tests for the generators
│       ├── enhanced_central_hub_generator_test.rs
│       └── error_test.rs
└── integration/             # Integration tests
    └── unified_analyzer_test.rs
```

## Test Utilities

The `test_utils.rs` file contains utilities for testing:

- `create_temp_dir()`: Creates a temporary directory for testing
- `create_test_file(dir, filename, content)`: Creates a test file with the given content
- `create_test_directory_structure(base_dir)`: Creates a test directory structure
- `create_mock_analysis_result()`: Creates a mock analysis result for testing
- `assert_file_contains(path, expected_content)`: Asserts that a file contains the expected content

## Unit Tests

The unit tests test individual components of the Unified Analyzer:

### Configuration Tests

- `test_config_from_file()`: Tests loading configuration from a file
- `test_config_default()`: Tests using the default configuration

### Generator Tests

- `test_generate_enhanced_central_hub()`: Tests the enhanced central hub generator
- `test_generator_error_display()`: Tests the error display for the generator error
- `test_generator_error_from_io_error()`: Tests converting IO errors to generator errors

## Integration Tests

The integration tests test the Unified Analyzer as a whole:

- `test_unified_analyzer_end_to_end()`: Tests the entire Unified Analyzer workflow, from analysis to documentation generation

## Running the Tests

You can run the tests using the following command:

```bash
cargo test
```

Or you can use the provided scripts:

- `run_tests.bat` (Windows)
- `run_tests.ps1` (Windows PowerShell)

## Adding New Tests

To add a new unit test:

1. Create a new file in the appropriate directory (e.g., `tests/unit/generators/new_generator_test.rs`)
2. Add the test module to `tests/main.rs`
3. Write your test functions

To add a new integration test:

1. Create a new file in the `tests/integration/` directory
2. Add the test module to `tests/main.rs`
3. Write your test functions

## Test Coverage

The test suite aims to cover all the functionality of the Unified Analyzer. However, there are still some areas that need more tests:

- Error handling in the generators
- Configuration validation
- Command-line argument parsing

## Future Improvements

- Add more tests for edge cases
- Add property-based tests
- Add benchmarks
- Add code coverage reporting
