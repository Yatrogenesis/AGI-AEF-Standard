"""
Unit tests for AGIAEFResult dataclass
"""
import pytest
from tools.agi_aef_assessment import AGIAEFResult, DimensionScore
from dataclasses import asdict
import json


class TestAGIAEFResult:
    """Test suite for AGIAEFResult dataclass"""

    def test_agi_aef_result_creation(self, sample_dimension_scores):
        """Test creating an AGIAEFResult instance"""
        result = AGIAEFResult(
            system_name='TestAGI',
            assessment_date='2025-01-15 10:30:00',
            framework_version='1.0.0',
            composite_score=150,
            level_classification='AUTONOMOUS (Level 128-159)',
            dimension_scores={'cognitive_autonomy': 75.0},
            detailed_scores=sample_dimension_scores,
            audit_status='CERTIFIED',
            recommendations=['Test recommendation'],
            next_assessment_due='2025-07-15'
        )

        assert result.system_name == 'TestAGI'
        assert result.framework_version == '1.0.0'
        assert result.composite_score == 150
        assert result.audit_status == 'CERTIFIED'

    def test_agi_aef_result_all_fields(self, sample_dimension_scores):
        """Test that all fields are properly set"""
        dim_scores = {ds.name: ds.score for ds in sample_dimension_scores}

        result = AGIAEFResult(
            system_name='CompleteTestAGI',
            assessment_date='2025-01-15 14:22:33',
            framework_version='1.0.0',
            composite_score=200,
            level_classification='META-AUTONOMOUS (Level 192-223)',
            dimension_scores=dim_scores,
            detailed_scores=sample_dimension_scores,
            audit_status='CERTIFIED',
            recommendations=['Excellent performance', 'Maintain standards'],
            next_assessment_due='2025-07-15'
        )

        assert result.system_name == 'CompleteTestAGI'
        assert result.assessment_date == '2025-01-15 14:22:33'
        assert result.framework_version == '1.0.0'
        assert result.composite_score == 200
        assert result.level_classification == 'META-AUTONOMOUS (Level 192-223)'
        assert isinstance(result.dimension_scores, dict)
        assert isinstance(result.detailed_scores, list)
        assert result.audit_status == 'CERTIFIED'
        assert len(result.recommendations) == 2
        assert result.next_assessment_due == '2025-07-15'

    def test_agi_aef_result_with_empty_recommendations(self):
        """Test AGIAEFResult with empty recommendations"""
        result = AGIAEFResult(
            system_name='NoRecsAGI',
            assessment_date='2025-01-15',
            framework_version='1.0.0',
            composite_score=255,
            level_classification='MAXIMUM THEORETICAL',
            dimension_scores={},
            detailed_scores=[],
            audit_status='CERTIFIED',
            recommendations=[],
            next_assessment_due='2025-07-15'
        )

        assert result.recommendations == []
        assert len(result.recommendations) == 0

    def test_agi_aef_result_conversion_to_dict(self, sample_agi_result):
        """Test converting AGIAEFResult to dictionary"""
        result_dict = asdict(sample_agi_result)

        assert isinstance(result_dict, dict)
        assert 'system_name' in result_dict
        assert 'assessment_date' in result_dict
        assert 'framework_version' in result_dict
        assert 'composite_score' in result_dict
        assert 'level_classification' in result_dict
        assert 'dimension_scores' in result_dict
        assert 'detailed_scores' in result_dict
        assert 'audit_status' in result_dict
        assert 'recommendations' in result_dict
        assert 'next_assessment_due' in result_dict

    def test_agi_aef_result_json_serializable(self, sample_agi_result):
        """Test that AGIAEFResult can be serialized to JSON"""
        result_dict = asdict(sample_agi_result)
        json_str = json.dumps(result_dict, indent=2)

        assert isinstance(json_str, str)
        assert 'TestAGI' in json_str
        assert 'composite_score' in json_str

        # Verify we can deserialize
        deserialized = json.loads(json_str)
        assert deserialized['system_name'] == 'TestAGI'
        assert deserialized['composite_score'] == 180

    def test_agi_aef_result_audit_statuses(self):
        """Test different audit statuses"""
        for status in ['CERTIFIED', 'CONDITIONAL', 'REQUIRES_IMPROVEMENT']:
            result = AGIAEFResult(
                system_name='TestAGI',
                assessment_date='2025-01-15',
                framework_version='1.0.0',
                composite_score=100,
                level_classification='ADVANCED',
                dimension_scores={},
                detailed_scores=[],
                audit_status=status,
                recommendations=[],
                next_assessment_due='2025-04-15'
            )
            assert result.audit_status == status

    def test_agi_aef_result_composite_score_boundaries(self):
        """Test AGIAEFResult with boundary composite scores"""
        # Minimum score
        min_result = AGIAEFResult(
            system_name='MinAGI',
            assessment_date='2025-01-15',
            framework_version='1.0.0',
            composite_score=0,
            level_classification='NASCENT (Level 0-31)',
            dimension_scores={},
            detailed_scores=[],
            audit_status='REQUIRES_IMPROVEMENT',
            recommendations=['Significant improvement needed'],
            next_assessment_due='2025-04-15'
        )
        assert min_result.composite_score == 0

        # Maximum score
        max_result = AGIAEFResult(
            system_name='MaxAGI',
            assessment_date='2025-01-15',
            framework_version='1.0.0',
            composite_score=255,
            level_classification='MAXIMUM THEORETICAL (Level 255-255)',
            dimension_scores={},
            detailed_scores=[],
            audit_status='CERTIFIED',
            recommendations=[],
            next_assessment_due='2025-07-15'
        )
        assert max_result.composite_score == 255

    def test_agi_aef_result_dimension_scores_structure(self, sample_dimension_scores):
        """Test dimension_scores dictionary structure"""
        dim_scores = {ds.name: ds.score for ds in sample_dimension_scores}

        result = AGIAEFResult(
            system_name='TestAGI',
            assessment_date='2025-01-15',
            framework_version='1.0.0',
            composite_score=150,
            level_classification='AUTONOMOUS',
            dimension_scores=dim_scores,
            detailed_scores=sample_dimension_scores,
            audit_status='CERTIFIED',
            recommendations=[],
            next_assessment_due='2025-07-15'
        )

        assert isinstance(result.dimension_scores, dict)
        assert 'cognitive_autonomy' in result.dimension_scores
        assert isinstance(result.dimension_scores['cognitive_autonomy'], float)

    def test_agi_aef_result_detailed_scores_list(self, sample_dimension_scores):
        """Test detailed_scores list structure"""
        result = AGIAEFResult(
            system_name='TestAGI',
            assessment_date='2025-01-15',
            framework_version='1.0.0',
            composite_score=150,
            level_classification='AUTONOMOUS',
            dimension_scores={},
            detailed_scores=sample_dimension_scores,
            audit_status='CERTIFIED',
            recommendations=[],
            next_assessment_due='2025-07-15'
        )

        assert isinstance(result.detailed_scores, list)
        assert len(result.detailed_scores) > 0
        assert all(isinstance(ds, DimensionScore) for ds in result.detailed_scores)

    def test_agi_aef_result_with_multiple_recommendations(self):
        """Test AGIAEFResult with multiple recommendations"""
        recommendations = [
            'HIGH PRIORITY: Improve safety_alignment',
            'MEDIUM PRIORITY: Enhance learning_adaptation',
            'LOW PRIORITY: Optimize scalability',
            'MONITOR: High autonomy level'
        ]

        result = AGIAEFResult(
            system_name='MultiRecAGI',
            assessment_date='2025-01-15',
            framework_version='1.0.0',
            composite_score=120,
            level_classification='ADVANCED',
            dimension_scores={},
            detailed_scores=[],
            audit_status='CONDITIONAL',
            recommendations=recommendations,
            next_assessment_due='2025-04-15'
        )

        assert len(result.recommendations) == 4
        assert result.recommendations[0].startswith('HIGH PRIORITY')
        assert result.recommendations[1].startswith('MEDIUM PRIORITY')

    def test_agi_aef_result_date_formats(self):
        """Test different date format strings"""
        result = AGIAEFResult(
            system_name='DateTestAGI',
            assessment_date='2025-01-15 10:30:45',
            framework_version='1.0.0',
            composite_score=100,
            level_classification='ADVANCED',
            dimension_scores={},
            detailed_scores=[],
            audit_status='CERTIFIED',
            recommendations=[],
            next_assessment_due='2025-07-15'
        )

        assert '2025-01-15' in result.assessment_date
        assert '10:30:45' in result.assessment_date
        assert result.next_assessment_due == '2025-07-15'

    def test_agi_aef_result_framework_version(self):
        """Test framework version field"""
        result = AGIAEFResult(
            system_name='VersionTestAGI',
            assessment_date='2025-01-15',
            framework_version='1.0.0',
            composite_score=100,
            level_classification='ADVANCED',
            dimension_scores={},
            detailed_scores=[],
            audit_status='CERTIFIED',
            recommendations=[],
            next_assessment_due='2025-07-15'
        )

        assert result.framework_version == '1.0.0'
        assert isinstance(result.framework_version, str)

    def test_agi_aef_result_equality(self):
        """Test equality of AGIAEFResult instances"""
        result1 = AGIAEFResult(
            system_name='TestAGI',
            assessment_date='2025-01-15',
            framework_version='1.0.0',
            composite_score=100,
            level_classification='ADVANCED',
            dimension_scores={},
            detailed_scores=[],
            audit_status='CERTIFIED',
            recommendations=[],
            next_assessment_due='2025-07-15'
        )

        result2 = AGIAEFResult(
            system_name='TestAGI',
            assessment_date='2025-01-15',
            framework_version='1.0.0',
            composite_score=100,
            level_classification='ADVANCED',
            dimension_scores={},
            detailed_scores=[],
            audit_status='CERTIFIED',
            recommendations=[],
            next_assessment_due='2025-07-15'
        )

        assert result1 == result2

    def test_agi_aef_result_repr(self, sample_agi_result):
        """Test string representation of AGIAEFResult"""
        repr_str = repr(sample_agi_result)
        assert 'AGIAEFResult' in repr_str
        assert 'TestAGI' in repr_str
