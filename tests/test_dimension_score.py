"""
Unit tests for DimensionScore dataclass
"""
import pytest
from tools.agi_aef_assessment import DimensionScore
from dataclasses import asdict


class TestDimensionScore:
    """Test suite for DimensionScore dataclass"""

    def test_dimension_score_creation(self):
        """Test creating a DimensionScore instance"""
        audit_points = [
            {'test_name': 'test1', 'raw_score': 20.0, 'weight': 0.5}
        ]

        score = DimensionScore(
            name='cognitive_autonomy',
            score=75.5,
            weight=20.0,
            weighted_score=15.1,
            audit_points=audit_points,
            validation_status='validated'
        )

        assert score.name == 'cognitive_autonomy'
        assert score.score == 75.5
        assert score.weight == 20.0
        assert score.weighted_score == 15.1
        assert score.audit_points == audit_points
        assert score.validation_status == 'validated'

    def test_dimension_score_with_empty_audit_points(self):
        """Test DimensionScore with empty audit points"""
        score = DimensionScore(
            name='test_dimension',
            score=0.0,
            weight=10.0,
            weighted_score=0.0,
            audit_points=[],
            validation_status='requires_improvement'
        )

        assert score.audit_points == []
        assert len(score.audit_points) == 0

    def test_dimension_score_with_multiple_audit_points(self, sample_audit_points):
        """Test DimensionScore with multiple audit points"""
        score = DimensionScore(
            name='cognitive_autonomy',
            score=75.0,
            weight=20.0,
            weighted_score=15.0,
            audit_points=sample_audit_points,
            validation_status='validated'
        )

        assert len(score.audit_points) == 2
        assert score.audit_points[0]['test_name'] == 'novel_problem_solving'
        assert score.audit_points[1]['test_name'] == 'creative_solution_generation'

    def test_dimension_score_field_types(self):
        """Test that DimensionScore fields have correct types"""
        score = DimensionScore(
            name='test',
            score=50.0,
            weight=10.0,
            weighted_score=5.0,
            audit_points=[],
            validation_status='validated'
        )

        assert isinstance(score.name, str)
        assert isinstance(score.score, float)
        assert isinstance(score.weight, float)
        assert isinstance(score.weighted_score, float)
        assert isinstance(score.audit_points, list)
        assert isinstance(score.validation_status, str)

    def test_dimension_score_conversion_to_dict(self, sample_dimension_score):
        """Test converting DimensionScore to dictionary"""
        score_dict = asdict(sample_dimension_score)

        assert isinstance(score_dict, dict)
        assert 'name' in score_dict
        assert 'score' in score_dict
        assert 'weight' in score_dict
        assert 'weighted_score' in score_dict
        assert 'audit_points' in score_dict
        assert 'validation_status' in score_dict

    def test_dimension_score_validation_statuses(self):
        """Test different validation statuses"""
        validated_score = DimensionScore(
            name='test1', score=80.0, weight=10.0,
            weighted_score=8.0, audit_points=[],
            validation_status='validated'
        )

        needs_improvement = DimensionScore(
            name='test2', score=50.0, weight=10.0,
            weighted_score=5.0, audit_points=[],
            validation_status='requires_improvement'
        )

        assert validated_score.validation_status == 'validated'
        assert needs_improvement.validation_status == 'requires_improvement'

    def test_dimension_score_boundary_values(self):
        """Test DimensionScore with boundary values"""
        # Minimum score
        min_score = DimensionScore(
            name='min_test', score=0.0, weight=0.0,
            weighted_score=0.0, audit_points=[],
            validation_status='requires_improvement'
        )
        assert min_score.score == 0.0
        assert min_score.weighted_score == 0.0

        # Maximum score
        max_score = DimensionScore(
            name='max_test', score=100.0, weight=20.0,
            weighted_score=20.0, audit_points=[],
            validation_status='validated'
        )
        assert max_score.score == 100.0
        assert max_score.weighted_score == 20.0

    def test_dimension_score_equality(self):
        """Test equality of DimensionScore instances"""
        score1 = DimensionScore(
            name='test', score=75.0, weight=20.0,
            weighted_score=15.0, audit_points=[],
            validation_status='validated'
        )

        score2 = DimensionScore(
            name='test', score=75.0, weight=20.0,
            weighted_score=15.0, audit_points=[],
            validation_status='validated'
        )

        assert score1 == score2

    def test_dimension_score_immutability_of_dataclass(self):
        """Test that DimensionScore can be modified (it's not frozen)"""
        score = DimensionScore(
            name='test', score=75.0, weight=20.0,
            weighted_score=15.0, audit_points=[],
            validation_status='validated'
        )

        # Should be able to modify since dataclass is not frozen
        score.score = 80.0
        assert score.score == 80.0

    def test_dimension_score_with_complex_audit_points(self):
        """Test DimensionScore with complex nested audit point data"""
        complex_audit = [
            {
                'test_name': 'complex_test',
                'raw_score': 22.5,
                'weight': 0.3,
                'weighted_score': 6.75,
                'max_possible': 25,
                'percentage': 90.0,
                'metadata': {
                    'execution_time': 1.5,
                    'memory_used': '512MB'
                }
            }
        ]

        score = DimensionScore(
            name='complex_dimension',
            score=90.0,
            weight=15.0,
            weighted_score=13.5,
            audit_points=complex_audit,
            validation_status='validated'
        )

        assert score.audit_points[0]['metadata']['execution_time'] == 1.5
        assert 'memory_used' in score.audit_points[0]['metadata']

    def test_dimension_score_repr(self, sample_dimension_score):
        """Test string representation of DimensionScore"""
        repr_str = repr(sample_dimension_score)
        assert 'DimensionScore' in repr_str
        assert 'cognitive_autonomy' in repr_str

    def test_dimension_score_with_special_characters_in_name(self):
        """Test DimensionScore with special characters in name"""
        score = DimensionScore(
            name='test_dimension_with_underscores',
            score=75.0,
            weight=10.0,
            weighted_score=7.5,
            audit_points=[],
            validation_status='validated'
        )

        assert '_' in score.name
        assert score.name == 'test_dimension_with_underscores'
