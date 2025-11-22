#!/bin/bash
# Test runner script for AGI-AEF Assessment Tool

echo "=========================================="
echo "AGI-AEF Assessment Tool - Test Suite"
echo "=========================================="
echo ""

# Check if virtual environment exists
if [ ! -d "venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv venv
fi

# Activate virtual environment
echo "Activating virtual environment..."
source venv/bin/activate

# Install dependencies
echo "Installing dependencies..."
pip install -q -r requirements-dev.txt

echo ""
echo "Running tests with coverage..."
echo "=========================================="

# Run pytest with coverage
pytest tests/ -v --cov=tools --cov-report=term-missing --cov-report=html --cov-branch

# Check exit code
if [ $? -eq 0 ]; then
    echo ""
    echo "=========================================="
    echo "✓ All tests passed!"
    echo "=========================================="
    echo ""
    echo "Coverage report generated in htmlcov/index.html"
else
    echo ""
    echo "=========================================="
    echo "✗ Some tests failed"
    echo "=========================================="
    exit 1
fi
