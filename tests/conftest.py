"""
Pytest configuration and shared fixtures for AGI-AEF tests
"""
import pytest
import tempfile
import yaml
from pathlib import Path
from datetime import datetime
from typing import Dict, Any


@pytest.fixture
def sample_config():
    """Sample configuration for tests"""
    return {
        'assessment_mode': 'comprehensive',
        'timeout_minutes': 120,
        'parallel_execution': True,
        'detailed_logging': True,
        'third_party_validation': False
    }


@pytest.fixture
def config_file(tmp_path, sample_config):
    """Create a temporary config file"""
    config_path = tmp_path / "test_config.yaml"
    with open(config_path, 'w') as f:
        yaml.dump(sample_config, f)
    return str(config_path)


@pytest.fixture
def dimension_weights():
    """Standard dimension weights"""
    return {
        'cognitive_autonomy': 20.0,
        'operational_independence': 18.0,
        'learning_adaptation': 16.0,
        'decision_making': 14.0,
        'communication': 10.0,
        'safety_alignment': 8.0,
        'generalization': 6.0,
        'self_awareness': 4.0,
        'scalability': 2.0,
        'integration': 1.0,
        'innovation': 0.5,
        'temporal_reasoning': 0.5
    }


@pytest.fixture
def sample_audit_points():
    """Sample audit points for a dimension"""
    return [
        {
            'test_name': 'novel_problem_solving',
            'raw_score': 20.0,
            'weight': 0.3,
            'weighted_score': 6.0,
            'max_possible': 25,
            'percentage': 80.0
        },
        {
            'test_name': 'creative_solution_generation',
            'raw_score': 18.0,
            'weight': 0.25,
            'weighted_score': 4.5,
            'max_possible': 25,
            'percentage': 72.0
        }
    ]


@pytest.fixture
def sample_dimension_score(sample_audit_points):
    """Sample DimensionScore object"""
    from tools.agi_aef_assessment import DimensionScore
    return DimensionScore(
        name='cognitive_autonomy',
        score=75.0,
        weight=20.0,
        weighted_score=15.0,
        audit_points=sample_audit_points,
        validation_status='validated'
    )


@pytest.fixture
def sample_dimension_scores():
    """Sample list of dimension scores for testing"""
    from tools.agi_aef_assessment import DimensionScore

    dimensions_data = [
        ('cognitive_autonomy', 75.0, 20.0),
        ('operational_independence', 80.0, 18.0),
        ('learning_adaptation', 70.0, 16.0),
        ('decision_making', 65.0, 14.0),
        ('communication', 85.0, 10.0),
        ('safety_alignment', 90.0, 8.0),
        ('generalization', 60.0, 6.0),
        ('self_awareness', 55.0, 4.0),
        ('scalability', 72.0, 2.0),
        ('integration', 68.0, 1.0),
        ('innovation', 45.0, 0.5),
        ('temporal_reasoning', 58.0, 0.5),
    ]

    scores = []
    for name, score, weight in dimensions_data:
        weighted_score = score * (weight / 100)
        scores.append(DimensionScore(
            name=name,
            score=score,
            weight=weight,
            weighted_score=weighted_score,
            audit_points=[],
            validation_status='validated' if score >= 70 else 'requires_improvement'
        ))

    return scores


@pytest.fixture
def sample_agi_result(sample_dimension_scores):
    """Sample AGIAEFResult object"""
    from tools.agi_aef_assessment import AGIAEFResult

    return AGIAEFResult(
        system_name='TestAGI',
        assessment_date='2025-01-15 10:30:00',
        framework_version='1.0.0',
        composite_score=180,
        level_classification='SUPER-AUTONOMOUS (Level 160-191)',
        dimension_scores={ds.name: ds.score for ds in sample_dimension_scores},
        detailed_scores=sample_dimension_scores,
        audit_status='CERTIFIED',
        recommendations=['Maintain current standards'],
        next_assessment_due='2025-07-15'
    )


@pytest.fixture
def mock_agi_system():
    """Mock AGI system for testing"""
    class MockAGISystem:
        def __init__(self):
            self.name = "MockAGI"
            self.capabilities = {}

    return MockAGISystem()


@pytest.fixture
def assessment_tests():
    """Standard assessment tests structure"""
    return {
        'cognitive_autonomy': {
            'novel_problem_solving': {'weight': 0.3, 'max_points': 25},
            'creative_solution_generation': {'weight': 0.25, 'max_points': 25},
            'abstract_reasoning': {'weight': 0.25, 'max_points': 25},
            'meta_cognitive_awareness': {'weight': 0.2, 'max_points': 25}
        },
        'operational_independence': {
            'self_maintenance': {'weight': 0.3, 'max_points': 25},
            'resource_management': {'weight': 0.25, 'max_points': 25},
            'error_recovery': {'weight': 0.25, 'max_points': 25},
            'continuous_operation': {'weight': 0.2, 'max_points': 25}
        }
    }


@pytest.fixture
def output_dir(tmp_path):
    """Temporary output directory for test results"""
    output = tmp_path / "results"
    output.mkdir()
    return output


@pytest.fixture(autouse=True)
def reset_random_seed():
    """Reset numpy random seed for reproducibility"""
    import numpy as np
    np.random.seed(42)
