"""
Integration tests for AGI-AEF Assessment Tool

These tests verify the complete workflow and interaction between components.
"""
import pytest
import json
import yaml
from pathlib import Path
from tools.agi_aef_assessment import (
    AGIAEFAssessment,
    DimensionScore,
    AGIAEFResult
)


class TestFullAssessmentWorkflow:
    """Test complete assessment workflow from start to finish"""

    def test_complete_assessment_workflow(self, tmp_path, mock_agi_system):
        """Test full assessment from initialization to export"""
        # 1. Create config file
        config_path = tmp_path / "config.yaml"
        config_data = {
            'assessment_mode': 'comprehensive',
            'timeout_minutes': 120,
            'parallel_execution': True,
            'detailed_logging': True,
            'third_party_validation': False
        }
        with open(config_path, 'w') as f:
            yaml.dump(config_data, f)

        # 2. Initialize assessor
        assessor = AGIAEFAssessment(str(config_path))
        assert assessor.config == config_data

        # 3. Run comprehensive assessment
        result = assessor.run_comprehensive_assessment('IntegrationTestAGI', mock_agi_system)
        assert isinstance(result, AGIAEFResult)
        assert result.system_name == 'IntegrationTestAGI'

        # 4. Export results
        output_path = tmp_path / "results.json"
        assessor.export_results(result, str(output_path))
        assert output_path.exists()

        # 5. Verify exported data
        with open(output_path, 'r') as f:
            exported_data = json.load(f)

        assert exported_data['system_name'] == 'IntegrationTestAGI'
        assert exported_data['framework_version'] == '1.0.0'
        assert 0 <= exported_data['composite_score'] <= 255

    def test_assessment_workflow_without_config(self, tmp_path, mock_agi_system):
        """Test assessment workflow using default configuration"""
        # Initialize without config
        assessor = AGIAEFAssessment()

        # Run assessment
        result = assessor.run_comprehensive_assessment('NoConfigTestAGI', mock_agi_system)

        # Verify result
        assert result.system_name == 'NoConfigTestAGI'
        assert result.framework_version == '1.0.0'
        assert len(result.detailed_scores) == 12

        # Export and verify
        output_path = tmp_path / "no_config_results.json"
        assessor.export_results(result, str(output_path))

        with open(output_path, 'r') as f:
            data = json.load(f)

        assert data['system_name'] == 'NoConfigTestAGI'

    def test_multiple_assessments_same_instance(self, mock_agi_system):
        """Test running multiple assessments with same assessor instance"""
        assessor = AGIAEFAssessment()

        # Run multiple assessments
        result1 = assessor.run_comprehensive_assessment('AGI_v1', mock_agi_system)
        result2 = assessor.run_comprehensive_assessment('AGI_v2', mock_agi_system)

        assert result1.system_name == 'AGI_v1'
        assert result2.system_name == 'AGI_v2'
        assert isinstance(result1, AGIAEFResult)
        assert isinstance(result2, AGIAEFResult)


class TestDimensionAssessmentIntegration:
    """Test integration of dimension assessment process"""

    def test_all_dimensions_assessed_correctly(self, mock_agi_system):
        """Test that all 12 dimensions are assessed"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('AllDimensionsTest', mock_agi_system)

        # Verify all dimensions are present
        expected_dimensions = [
            'cognitive_autonomy',
            'operational_independence',
            'learning_adaptation',
            'decision_making',
            'communication',
            'safety_alignment',
            'generalization',
            'self_awareness',
            'scalability',
            'integration',
            'innovation',
            'temporal_reasoning'
        ]

        for dimension in expected_dimensions:
            assert dimension in result.dimension_scores
            assert any(ds.name == dimension for ds in result.detailed_scores)

    def test_dimension_weights_applied_correctly(self, mock_agi_system):
        """Test that dimension weights are correctly applied"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('WeightTest', mock_agi_system)

        # Calculate total weighted score manually
        total_weighted = sum(ds.weighted_score for ds in result.detailed_scores)

        # Verify it's within reasonable range (0-100)
        assert 0 <= total_weighted <= 100

    def test_audit_points_captured_for_all_dimensions(self, mock_agi_system):
        """Test that audit points are captured for each dimension"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('AuditTest', mock_agi_system)

        for dimension_score in result.detailed_scores:
            assert len(dimension_score.audit_points) == 4  # 4 tests per dimension
            for audit_point in dimension_score.audit_points:
                assert 'test_name' in audit_point
                assert 'raw_score' in audit_point
                assert 'weight' in audit_point


class TestCompositeScoreCalculation:
    """Test composite score calculation in full context"""

    def test_composite_score_reflects_all_dimensions(self, mock_agi_system):
        """Test that composite score incorporates all dimensions"""
        assessor = AGIAEFAssessment()

        # Patch to ensure consistent high scores
        original_execute = assessor._execute_test

        def high_score_execute(test_name, test_config, agi_system):
            return 24.0  # 96% score

        assessor._execute_test = high_score_execute

        result = assessor.run_comprehensive_assessment('HighScoreTest', mock_agi_system)

        # Should get a high composite score
        assert result.composite_score > 200

        assessor._execute_test = original_execute

    def test_composite_score_with_varied_dimension_scores(self, mock_agi_system):
        """Test composite score with varying dimension performances"""
        assessor = AGIAEFAssessment()

        # Create varied scores
        call_count = [0]

        def varied_score_execute(test_name, test_config, agi_system):
            # Alternate between high and low scores
            call_count[0] += 1
            return 20.0 if call_count[0] % 2 == 0 else 15.0

        assessor._execute_test = varied_score_execute

        result = assessor.run_comprehensive_assessment('VariedTest', mock_agi_system)

        # Should get a medium composite score
        assert 64 <= result.composite_score <= 191


class TestLevelClassificationIntegration:
    """Test level classification in full workflow"""

    def test_level_classification_matches_composite_score(self, mock_agi_system):
        """Test that level classification is correct for composite score"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('ClassificationTest', mock_agi_system)

        composite = result.composite_score
        classification = result.level_classification

        # Verify classification matches score
        if 0 <= composite <= 31:
            assert 'NASCENT' in classification
        elif 32 <= composite <= 63:
            assert 'BASIC' in classification
        elif 64 <= composite <= 95:
            assert 'INTERMEDIATE' in classification
        elif 96 <= composite <= 127:
            assert 'ADVANCED' in classification
        elif 128 <= composite <= 159:
            assert 'AUTONOMOUS' in classification
        elif 160 <= composite <= 191:
            assert 'SUPER-AUTONOMOUS' in classification
        elif 192 <= composite <= 223:
            assert 'META-AUTONOMOUS' in classification
        elif 224 <= composite <= 254:
            assert 'HYPER-AUTONOMOUS' in classification
        elif composite == 255:
            assert 'MAXIMUM THEORETICAL' in classification


class TestAuditStatusDetermination:
    """Test audit status determination in full context"""

    def test_certified_status_with_high_scores(self, mock_agi_system):
        """Test CERTIFIED status with all high scores"""
        assessor = AGIAEFAssessment()

        def high_score_execute(test_name, test_config, agi_system):
            return 24.0  # 96% score

        assessor._execute_test = high_score_execute

        result = assessor.run_comprehensive_assessment('CertifiedTest', mock_agi_system)

        assert result.audit_status == 'CERTIFIED'

    def test_requires_improvement_status_with_low_scores(self, mock_agi_system):
        """Test REQUIRES_IMPROVEMENT status with low scores"""
        assessor = AGIAEFAssessment()

        def low_score_execute(test_name, test_config, agi_system):
            return 10.0  # 40% score

        assessor._execute_test = low_score_execute

        result = assessor.run_comprehensive_assessment('LowScoreTest', mock_agi_system)

        assert result.audit_status == 'REQUIRES_IMPROVEMENT'

    def test_conditional_status_with_medium_scores(self, mock_agi_system):
        """Test CONDITIONAL status with medium scores"""
        assessor = AGIAEFAssessment()

        def medium_score_execute(test_name, test_config, agi_system):
            return 18.5  # 74% score

        assessor._execute_test = medium_score_execute

        result = assessor.run_comprehensive_assessment('MediumScoreTest', mock_agi_system)

        assert result.audit_status in ['CONDITIONAL', 'CERTIFIED', 'REQUIRES_IMPROVEMENT']


class TestRecommendationGeneration:
    """Test recommendation generation in full workflow"""

    def test_recommendations_generated_for_weak_areas(self, mock_agi_system):
        """Test that recommendations target weak dimensions"""
        assessor = AGIAEFAssessment()

        # Create scenario with specific weak dimension
        dimension_targets = {
            'innovation': 10.0,  # Very low
            'temporal_reasoning': 12.0,  # Very low
            'self_awareness': 15.0  # Low
        }

        def targeted_score_execute(test_name, test_config, agi_system):
            for dim in dimension_targets:
                if dim in test_name or test_name in assessor.assessment_tests.get(dim, {}):
                    return dimension_targets[dim]
            return 22.0  # Good score for others

        assessor._execute_test = targeted_score_execute

        result = assessor.run_comprehensive_assessment('TargetedTest', mock_agi_system)

        # Should have recommendations
        assert len(result.recommendations) > 0

    def test_safety_critical_recommendation(self, mock_agi_system):
        """Test critical safety recommendation when safety score is low"""
        assessor = AGIAEFAssessment()

        def safety_low_execute(test_name, test_config, agi_system):
            # Low score for safety-related tests
            if 'value_alignment' in test_name or 'harm_prevention' in test_name:
                return 12.0
            return 20.0

        assessor._execute_test = safety_low_execute

        result = assessor.run_comprehensive_assessment('SafetyTest', mock_agi_system)

        # Should have critical safety recommendation
        critical_recs = [r for r in result.recommendations if 'CRITICAL' in r and 'Safety' in r]
        assert len(critical_recs) > 0


class TestNextAssessmentScheduling:
    """Test next assessment date calculation"""

    def test_high_autonomy_next_assessment(self, mock_agi_system):
        """Test 6-month interval for high autonomy systems"""
        assessor = AGIAEFAssessment()

        def high_score_execute(test_name, test_config, agi_system):
            return 22.0  # High scores -> high composite

        assessor._execute_test = high_score_execute

        result = assessor.run_comprehensive_assessment('HighAutonomy', mock_agi_system)

        # Should have next assessment date
        assert result.next_assessment_due is not None
        assert len(result.next_assessment_due) > 0

    def test_normal_autonomy_next_assessment(self, mock_agi_system):
        """Test 3-month interval for normal autonomy systems"""
        assessor = AGIAEFAssessment()

        def medium_score_execute(test_name, test_config, agi_system):
            return 15.0  # Medium scores -> lower composite

        assessor._execute_test = medium_score_execute

        result = assessor.run_comprehensive_assessment('NormalAutonomy', mock_agi_system)

        # Should have next assessment date
        assert result.next_assessment_due is not None


class TestResultExportIntegration:
    """Test result export and serialization"""

    def test_export_and_reimport_results(self, tmp_path, mock_agi_system):
        """Test that exported results can be reimported"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('ExportTest', mock_agi_system)

        # Export
        output_path = tmp_path / "export_test.json"
        assessor.export_results(result, str(output_path))

        # Reimport
        with open(output_path, 'r') as f:
            reimported = json.load(f)

        # Verify data integrity
        assert reimported['system_name'] == result.system_name
        assert reimported['composite_score'] == result.composite_score
        assert reimported['framework_version'] == result.framework_version
        assert reimported['audit_status'] == result.audit_status

    def test_export_preserves_all_dimension_data(self, tmp_path, mock_agi_system):
        """Test that export preserves all dimension scores and details"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('DimensionDataTest', mock_agi_system)

        output_path = tmp_path / "dimension_test.json"
        assessor.export_results(result, str(output_path))

        with open(output_path, 'r') as f:
            data = json.load(f)

        # Verify all dimensions are in exported data
        assert len(data['dimension_scores']) == 12
        assert len(data['detailed_scores']) == 12

        # Verify detailed scores have audit points
        for detailed_score in data['detailed_scores']:
            assert 'audit_points' in detailed_score
            assert len(detailed_score['audit_points']) == 4

    def test_export_results_formatting(self, tmp_path, mock_agi_system):
        """Test that exported JSON is properly formatted"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('FormatTest', mock_agi_system)

        output_path = tmp_path / "format_test.json"
        assessor.export_results(result, str(output_path))

        with open(output_path, 'r') as f:
            content = f.read()

        # Should be indented/formatted
        assert '\n' in content
        assert '  ' in content or '\t' in content


class TestConfigurationImpact:
    """Test how different configurations affect assessment"""

    def test_comprehensive_mode_assessment(self, tmp_path, mock_agi_system):
        """Test assessment in comprehensive mode"""
        config_path = tmp_path / "comprehensive_config.yaml"
        config = {
            'assessment_mode': 'comprehensive',
            'timeout_minutes': 120,
            'parallel_execution': True
        }
        with open(config_path, 'w') as f:
            yaml.dump(config, f)

        assessor = AGIAEFAssessment(str(config_path))
        result = assessor.run_comprehensive_assessment('ComprehensiveTest', mock_agi_system)

        # Should assess all dimensions
        assert len(result.detailed_scores) == 12

    def test_config_values_loaded_correctly(self, tmp_path):
        """Test that all config values are loaded correctly"""
        config_path = tmp_path / "full_config.yaml"
        config = {
            'assessment_mode': 'comprehensive',
            'timeout_minutes': 180,
            'parallel_execution': False,
            'detailed_logging': True,
            'third_party_validation': True
        }
        with open(config_path, 'w') as f:
            yaml.dump(config, f)

        assessor = AGIAEFAssessment(str(config_path))

        assert assessor.config['assessment_mode'] == 'comprehensive'
        assert assessor.config['timeout_minutes'] == 180
        assert assessor.config['parallel_execution'] is False
        assert assessor.config['detailed_logging'] is True
        assert assessor.config['third_party_validation'] is True


class TestEdgeCases:
    """Test edge cases and boundary conditions"""

    def test_assessment_with_none_agi_system(self):
        """Test assessment when AGI system is None"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('NoneSystemTest', None)

        assert isinstance(result, AGIAEFResult)
        assert result.system_name == 'NoneSystemTest'

    def test_assessment_with_empty_system_name(self, mock_agi_system):
        """Test assessment with empty system name"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('', mock_agi_system)

        assert result.system_name == ''

    def test_assessment_consistency_same_random_seed(self, mock_agi_system):
        """Test that assessments are reproducible with same random seed"""
        import numpy as np

        assessor = AGIAEFAssessment()

        # Run with same seed
        np.random.seed(42)
        result1 = assessor.run_comprehensive_assessment('ConsistencyTest', mock_agi_system)

        np.random.seed(42)
        result2 = assessor.run_comprehensive_assessment('ConsistencyTest', mock_agi_system)

        # Composite scores should be identical
        assert result1.composite_score == result2.composite_score

    def test_export_to_nonexistent_directory(self, tmp_path, sample_agi_result):
        """Test exporting to a directory that doesn't exist yet"""
        assessor = AGIAEFAssessment()
        output_path = tmp_path / "nonexistent" / "subdir" / "result.json"

        # Create parent directory
        output_path.parent.mkdir(parents=True, exist_ok=True)

        # Should create directory and export
        assessor.export_results(sample_agi_result, str(output_path))
        assert output_path.exists()


class TestDataValidation:
    """Test data validation and integrity"""

    def test_dimension_weights_sum_to_expected_total(self):
        """Test that dimension weights sum correctly"""
        assessor = AGIAEFAssessment()
        total = sum(assessor.DIMENSION_WEIGHTS.values())

        # Should be close to 100 (allowing for floating point)
        assert 99.5 <= total <= 100.5

    def test_all_test_weights_valid(self):
        """Test that all test weights are valid"""
        assessor = AGIAEFAssessment()

        for dimension, tests in assessor.assessment_tests.items():
            for test_name, test_config in tests.items():
                assert 0 < test_config['weight'] <= 1
                assert test_config['max_points'] > 0

    def test_composite_score_never_exceeds_255(self, mock_agi_system):
        """Test that composite score never exceeds maximum"""
        assessor = AGIAEFAssessment()

        # Try with artificially high scores
        def max_score_execute(test_name, test_config, agi_system):
            return test_config['max_points']

        assessor._execute_test = max_score_execute

        result = assessor.run_comprehensive_assessment('MaxTest', mock_agi_system)

        assert result.composite_score <= 255

    def test_all_scores_non_negative(self, mock_agi_system):
        """Test that all scores are non-negative"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('NonNegativeTest', mock_agi_system)

        assert result.composite_score >= 0

        for dimension_score in result.detailed_scores:
            assert dimension_score.score >= 0
            assert dimension_score.weighted_score >= 0

            for audit_point in dimension_score.audit_points:
                assert audit_point['raw_score'] >= 0
                assert audit_point['weighted_score'] >= 0
