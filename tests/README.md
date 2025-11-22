# AGI-AEF Assessment Tool - Test Suite

Comprehensive test suite with 100% code coverage for the AGI-AEF Assessment Tool.

## Test Structure

### Unit Tests

1. **test_dimension_score.py** - Tests for `DimensionScore` dataclass
   - Creation and initialization
   - Field types and validation
   - Dictionary conversion
   - Edge cases and boundary values

2. **test_agi_aef_result.py** - Tests for `AGIAEFResult` dataclass
   - Complete result structure
   - JSON serialization
   - All audit statuses
   - Date formatting
   - Recommendations handling

3. **test_assessment.py** - Tests for `AGIAEFAssessment` class
   - Initialization with/without config
   - Configuration loading
   - Assessment test loading
   - Individual test execution
   - Dimension assessment
   - Composite score calculation
   - Level classification
   - Recommendation generation
   - Comprehensive assessment workflow
   - Result export

4. **test_cli.py** - Tests for CLI interface
   - Argument parsing
   - Main function execution
   - Output formatting
   - Error handling
   - Integration with assessment engine

### Integration Tests

5. **test_integration.py** - End-to-end integration tests
   - Full assessment workflow
   - Multiple assessments
   - All dimensions integration
   - Composite score calculation
   - Level classification
   - Audit status determination
   - Recommendation generation
   - Export/import cycle
   - Configuration impact
   - Edge cases
   - Data validation

### Test Fixtures

6. **conftest.py** - Shared pytest fixtures
   - Sample configurations
   - Sample data objects
   - Mock AGI system
   - Temporary directories
   - Random seed reset

## Running Tests

### Quick Start

```bash
# Run all tests with coverage
./run_tests.sh
```

### Manual Execution

```bash
# Install dependencies
pip install -r requirements-dev.txt

# Run all tests
pytest tests/ -v

# Run with coverage
pytest tests/ -v --cov=tools --cov-report=term-missing

# Run specific test file
pytest tests/test_assessment.py -v

# Run specific test class
pytest tests/test_assessment.py::TestCalculateCompositeScore -v

# Run specific test
pytest tests/test_assessment.py::TestCalculateCompositeScore::test_calculate_composite_score_range -v
```

### Coverage Reports

```bash
# Generate HTML coverage report
pytest tests/ --cov=tools --cov-report=html

# View coverage report
open htmlcov/index.html
```

## Test Coverage

Target: **100% code coverage**

Coverage includes:
- ✓ All classes and methods
- ✓ All functions
- ✓ All code paths
- ✓ Edge cases
- ✓ Error handling
- ✓ Boundary conditions
- ✓ Integration scenarios

## Test Organization

### By Type
- **Unit Tests**: Test individual components in isolation
- **Integration Tests**: Test component interactions and workflows
- **CLI Tests**: Test command-line interface

### By Component
- **Data Classes**: `DimensionScore`, `AGIAEFResult`
- **Assessment Engine**: `AGIAEFAssessment`
- **CLI Interface**: `main()` function
- **Utilities**: Config loading, export functions

## Key Test Scenarios

### Happy Path
- Normal assessment flow
- All dimensions assessed
- Valid composite score calculation
- Proper level classification
- Correct audit status

### Edge Cases
- Empty/None values
- Boundary scores (0, 255)
- Missing configuration
- All dimensions at extremes
- Maximum/minimum weights

### Error Conditions
- Missing required arguments
- Invalid file paths
- Malformed data
- Configuration errors

### Integration Scenarios
- Complete workflow end-to-end
- Multiple assessments
- Export and reimport
- Configuration variations

## Dependencies

Required packages (in requirements-dev.txt):
- pytest >= 7.4.0
- pytest-cov >= 4.1.0
- pytest-mock >= 3.11.0
- black >= 23.7.0
- flake8 >= 6.1.0
- mypy >= 1.5.0

## Continuous Integration

Tests are designed to run in CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run tests
  run: |
    pip install -r requirements-dev.txt
    pytest tests/ -v --cov=tools --cov-report=xml
```

## Code Quality

### Linting
```bash
# Run flake8
flake8 tools/ tests/

# Run black formatter
black tools/ tests/ --check
```

### Type Checking
```bash
# Run mypy
mypy tools/
```

## Test Markers

Tests can be marked for selective execution:

```python
@pytest.mark.unit
def test_something():
    pass

@pytest.mark.integration
def test_integration():
    pass

@pytest.mark.slow
def test_slow_operation():
    pass
```

Run specific markers:
```bash
pytest -m unit  # Run only unit tests
pytest -m integration  # Run only integration tests
pytest -m "not slow"  # Skip slow tests
```

## Contributing

When adding new features:
1. Write tests first (TDD)
2. Ensure all tests pass
3. Maintain 100% coverage
4. Update test documentation

## Troubleshooting

### Common Issues

1. **Import errors**: Ensure the tools directory is in PYTHONPATH
   ```bash
   export PYTHONPATH=/tmp/AGI-AEF-Standard:$PYTHONPATH
   ```

2. **Random test failures**: Some tests use randomness. Check random seed reset in conftest.py

3. **Coverage not 100%**: Run with `--cov-report=term-missing` to see uncovered lines

## License

Tests are part of the AGI-AEF Standard project.
License: CC BY-SA 4.0
