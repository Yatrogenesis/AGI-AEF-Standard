"""
Unit tests for AGIAEFAssessment class
"""
import pytest
import numpy as np
from pathlib import Path
from tools.agi_aef_assessment import AGIAEFAssessment, DimensionScore, AGIAEFResult


class TestAGIAEFAssessmentInit:
    """Test AGIAEFAssessment initialization"""

    def test_init_without_config(self):
        """Test initialization without config file"""
        assessor = AGIAEFAssessment()

        assert assessor.config is not None
        assert assessor.config['assessment_mode'] == 'comprehensive'
        assert assessor.config['timeout_minutes'] == 120
        assert assessor.config['parallel_execution'] is True

    def test_init_with_config_file(self, config_file):
        """Test initialization with config file"""
        assessor = AGIAEFAssessment(config_file)

        assert assessor.config is not None
        assert assessor.config['assessment_mode'] == 'comprehensive'
        assert assessor.config['detailed_logging'] is True

    def test_init_with_nonexistent_config(self):
        """Test initialization with non-existent config file"""
        assessor = AGIAEFAssessment('/nonexistent/path/config.yaml')

        # Should use default config
        assert assessor.config['assessment_mode'] == 'comprehensive'
        assert assessor.config['timeout_minutes'] == 120

    def test_dimension_weights_sum(self):
        """Test that dimension weights sum to 100%"""
        assessor = AGIAEFAssessment()
        total_weight = sum(assessor.DIMENSION_WEIGHTS.values())

        assert abs(total_weight - 100.0) < 1.0  # Allow small floating point difference

    def test_assessment_tests_loaded(self):
        """Test that assessment tests are properly loaded"""
        assessor = AGIAEFAssessment()

        assert assessor.assessment_tests is not None
        assert 'cognitive_autonomy' in assessor.assessment_tests
        assert 'operational_independence' in assessor.assessment_tests

    def test_all_dimensions_have_tests(self):
        """Test that all dimensions have corresponding tests"""
        assessor = AGIAEFAssessment()

        for dimension in assessor.DIMENSION_WEIGHTS.keys():
            assert dimension in assessor.assessment_tests
            assert len(assessor.assessment_tests[dimension]) > 0


class TestLoadConfig:
    """Test _load_config method"""

    def test_load_config_with_valid_file(self, config_file):
        """Test loading valid config file"""
        assessor = AGIAEFAssessment(config_file)
        config = assessor.config

        assert config['assessment_mode'] == 'comprehensive'
        assert config['timeout_minutes'] == 120

    def test_load_config_default(self):
        """Test default config loading"""
        assessor = AGIAEFAssessment()
        config = assessor.config

        assert 'assessment_mode' in config
        assert 'timeout_minutes' in config
        assert 'parallel_execution' in config
        assert 'detailed_logging' in config
        assert 'third_party_validation' in config


class TestLoadAssessmentTests:
    """Test _load_assessment_tests method"""

    def test_load_assessment_tests_structure(self):
        """Test assessment tests structure"""
        assessor = AGIAEFAssessment()
        tests = assessor.assessment_tests

        assert isinstance(tests, dict)
        assert len(tests) == 12  # All dimensions

    def test_each_dimension_has_four_tests(self):
        """Test that each dimension has exactly 4 tests"""
        assessor = AGIAEFAssessment()

        for dimension, tests in assessor.assessment_tests.items():
            assert len(tests) == 4, f"{dimension} should have 4 tests"

    def test_test_weights_sum_to_one(self):
        """Test that weights within each dimension sum to 1.0"""
        assessor = AGIAEFAssessment()

        for dimension, tests in assessor.assessment_tests.items():
            total_weight = sum(test['weight'] for test in tests.values())
            assert abs(total_weight - 1.0) < 0.01, f"{dimension} weights should sum to 1.0"

    def test_all_tests_have_max_points(self):
        """Test that all tests have max_points defined"""
        assessor = AGIAEFAssessment()

        for dimension, tests in assessor.assessment_tests.items():
            for test_name, test_config in tests.items():
                assert 'max_points' in test_config
                assert test_config['max_points'] == 25


class TestExecuteTest:
    """Test _execute_test method"""

    def test_execute_test_returns_float(self, mock_agi_system):
        """Test that _execute_test returns a float score"""
        assessor = AGIAEFAssessment()
        test_config = {'weight': 0.3, 'max_points': 25}

        score = assessor._execute_test('novel_problem_solving', test_config, mock_agi_system)

        assert isinstance(score, (float, np.floating))
        assert 0 <= score <= 25

    def test_execute_test_respects_max_points(self, mock_agi_system):
        """Test that scores don't exceed max_points"""
        assessor = AGIAEFAssessment()
        test_config = {'weight': 0.3, 'max_points': 25}

        for _ in range(10):
            score = assessor._execute_test('test', test_config, mock_agi_system)
            assert score <= test_config['max_points']

    def test_execute_test_non_negative(self, mock_agi_system):
        """Test that scores are non-negative"""
        assessor = AGIAEFAssessment()
        test_config = {'weight': 0.3, 'max_points': 25}

        for _ in range(10):
            score = assessor._execute_test('test', test_config, mock_agi_system)
            assert score >= 0

    def test_execute_test_with_known_test_names(self, mock_agi_system):
        """Test execution with specific test names"""
        assessor = AGIAEFAssessment()
        test_config = {'weight': 0.3, 'max_points': 25}

        test_names = ['novel_problem_solving', 'ethical_reasoning', 'value_alignment']

        for test_name in test_names:
            score = assessor._execute_test(test_name, test_config, mock_agi_system)
            assert isinstance(score, (float, np.floating))

    def test_execute_test_with_unknown_test_name(self, mock_agi_system):
        """Test execution with unknown test name uses default"""
        assessor = AGIAEFAssessment()
        test_config = {'weight': 0.3, 'max_points': 25}

        score = assessor._execute_test('unknown_test', test_config, mock_agi_system)
        assert isinstance(score, (float, np.floating))
        assert 0 <= score <= 25


class TestAssessDimension:
    """Test assess_dimension method"""

    def test_assess_dimension_returns_dimension_score(self, mock_agi_system):
        """Test that assess_dimension returns a DimensionScore"""
        assessor = AGIAEFAssessment()
        score = assessor.assess_dimension('cognitive_autonomy', mock_agi_system)

        assert isinstance(score, DimensionScore)

    def test_assess_dimension_correct_name(self, mock_agi_system):
        """Test that dimension score has correct name"""
        assessor = AGIAEFAssessment()
        score = assessor.assess_dimension('cognitive_autonomy', mock_agi_system)

        assert score.name == 'cognitive_autonomy'

    def test_assess_dimension_has_weight(self, mock_agi_system):
        """Test that dimension score has correct weight"""
        assessor = AGIAEFAssessment()
        score = assessor.assess_dimension('cognitive_autonomy', mock_agi_system)

        assert score.weight == 20.0

    def test_assess_dimension_score_range(self, mock_agi_system):
        """Test that dimension score is in valid range"""
        assessor = AGIAEFAssessment()
        score = assessor.assess_dimension('cognitive_autonomy', mock_agi_system)

        assert 0 <= score.score <= 100

    def test_assess_dimension_has_audit_points(self, mock_agi_system):
        """Test that dimension score includes audit points"""
        assessor = AGIAEFAssessment()
        score = assessor.assess_dimension('cognitive_autonomy', mock_agi_system)

        assert len(score.audit_points) == 4  # 4 tests per dimension

    def test_assess_dimension_audit_point_structure(self, mock_agi_system):
        """Test audit point structure"""
        assessor = AGIAEFAssessment()
        score = assessor.assess_dimension('cognitive_autonomy', mock_agi_system)

        for audit_point in score.audit_points:
            assert 'test_name' in audit_point
            assert 'raw_score' in audit_point
            assert 'weight' in audit_point
            assert 'weighted_score' in audit_point
            assert 'max_possible' in audit_point
            assert 'percentage' in audit_point

    def test_assess_dimension_validation_status_validated(self, mock_agi_system):
        """Test validation status for high scores"""
        assessor = AGIAEFAssessment()
        # Monkey patch to ensure high scores
        original_execute = assessor._execute_test

        def high_score_execute(test_name, test_config, agi_system):
            return 24.0  # High score

        assessor._execute_test = high_score_execute
        score = assessor.assess_dimension('cognitive_autonomy', mock_agi_system)

        assert score.validation_status == 'validated'
        assessor._execute_test = original_execute

    def test_assess_dimension_validation_status_needs_improvement(self, mock_agi_system):
        """Test validation status for low scores"""
        assessor = AGIAEFAssessment()
        # Monkey patch to ensure low scores
        original_execute = assessor._execute_test

        def low_score_execute(test_name, test_config, agi_system):
            return 10.0  # Low score

        assessor._execute_test = low_score_execute
        score = assessor.assess_dimension('cognitive_autonomy', mock_agi_system)

        assert score.validation_status == 'requires_improvement'
        assessor._execute_test = original_execute

    def test_assess_all_dimensions(self, mock_agi_system):
        """Test assessing all dimensions"""
        assessor = AGIAEFAssessment()

        for dimension in assessor.DIMENSION_WEIGHTS.keys():
            score = assessor.assess_dimension(dimension, mock_agi_system)
            assert isinstance(score, DimensionScore)
            assert score.name == dimension


class TestCalculateCompositeScore:
    """Test calculate_composite_score method"""

    def test_calculate_composite_score_range(self, sample_dimension_scores):
        """Test composite score is in valid range"""
        assessor = AGIAEFAssessment()
        composite = assessor.calculate_composite_score(sample_dimension_scores)

        assert 0 <= composite <= 255

    def test_calculate_composite_score_is_integer(self, sample_dimension_scores):
        """Test composite score is an integer"""
        assessor = AGIAEFAssessment()
        composite = assessor.calculate_composite_score(sample_dimension_scores)

        assert isinstance(composite, int)

    def test_calculate_composite_score_zero_scores(self):
        """Test composite score with all zero scores"""
        assessor = AGIAEFAssessment()
        zero_scores = []

        for dimension, weight in assessor.DIMENSION_WEIGHTS.items():
            score = DimensionScore(
                name=dimension,
                score=0.0,
                weight=weight,
                weighted_score=0.0,
                audit_points=[],
                validation_status='requires_improvement'
            )
            zero_scores.append(score)

        composite = assessor.calculate_composite_score(zero_scores)
        assert composite == 0

    def test_calculate_composite_score_perfect_scores(self):
        """Test composite score with perfect scores"""
        assessor = AGIAEFAssessment()
        perfect_scores = []

        for dimension, weight in assessor.DIMENSION_WEIGHTS.items():
            weighted_score = 100.0 * (weight / 100)
            score = DimensionScore(
                name=dimension,
                score=100.0,
                weight=weight,
                weighted_score=weighted_score,
                audit_points=[],
                validation_status='validated'
            )
            perfect_scores.append(score)

        composite = assessor.calculate_composite_score(perfect_scores)
        assert composite == 255

    def test_calculate_composite_score_clamping_upper(self):
        """Test that composite score is clamped to 255"""
        assessor = AGIAEFAssessment()
        # Create artificially high scores
        high_scores = []

        for dimension, weight in assessor.DIMENSION_WEIGHTS.items():
            score = DimensionScore(
                name=dimension,
                score=100.0,
                weight=weight,
                weighted_score=weight,  # This will give total of 99
                audit_points=[],
                validation_status='validated'
            )
            high_scores.append(score)

        composite = assessor.calculate_composite_score(high_scores)
        assert composite <= 255

    def test_calculate_composite_score_clamping_lower(self):
        """Test that composite score doesn't go negative"""
        assessor = AGIAEFAssessment()
        # Should never happen, but test clamping
        negative_scores = []

        for dimension, weight in assessor.DIMENSION_WEIGHTS.items():
            score = DimensionScore(
                name=dimension,
                score=0.0,
                weight=weight,
                weighted_score=-10.0,  # Artificially negative
                audit_points=[],
                validation_status='requires_improvement'
            )
            negative_scores.append(score)

        composite = assessor.calculate_composite_score(negative_scores)
        assert composite >= 0


class TestClassifyLevel:
    """Test classify_level method"""

    def test_classify_level_nascent(self):
        """Test NASCENT level classification"""
        assessor = AGIAEFAssessment()

        for score in [0, 15, 31]:
            classification = assessor.classify_level(score)
            assert 'NASCENT' in classification

    def test_classify_level_basic(self):
        """Test BASIC level classification"""
        assessor = AGIAEFAssessment()

        for score in [32, 50, 63]:
            classification = assessor.classify_level(score)
            assert 'BASIC' in classification

    def test_classify_level_intermediate(self):
        """Test INTERMEDIATE level classification"""
        assessor = AGIAEFAssessment()

        for score in [64, 80, 95]:
            classification = assessor.classify_level(score)
            assert 'INTERMEDIATE' in classification

    def test_classify_level_advanced(self):
        """Test ADVANCED level classification"""
        assessor = AGIAEFAssessment()

        for score in [96, 110, 127]:
            classification = assessor.classify_level(score)
            assert 'ADVANCED' in classification

    def test_classify_level_autonomous(self):
        """Test AUTONOMOUS level classification"""
        assessor = AGIAEFAssessment()

        for score in [128, 140, 159]:
            classification = assessor.classify_level(score)
            assert 'AUTONOMOUS' in classification

    def test_classify_level_super_autonomous(self):
        """Test SUPER-AUTONOMOUS level classification"""
        assessor = AGIAEFAssessment()

        for score in [160, 175, 191]:
            classification = assessor.classify_level(score)
            assert 'SUPER-AUTONOMOUS' in classification

    def test_classify_level_meta_autonomous(self):
        """Test META-AUTONOMOUS level classification"""
        assessor = AGIAEFAssessment()

        for score in [192, 207, 223]:
            classification = assessor.classify_level(score)
            assert 'META-AUTONOMOUS' in classification

    def test_classify_level_hyper_autonomous(self):
        """Test HYPER-AUTONOMOUS level classification"""
        assessor = AGIAEFAssessment()

        for score in [224, 240, 254]:
            classification = assessor.classify_level(score)
            assert 'HYPER-AUTONOMOUS' in classification

    def test_classify_level_maximum(self):
        """Test MAXIMUM THEORETICAL level classification"""
        assessor = AGIAEFAssessment()
        classification = assessor.classify_level(255)

        assert 'MAXIMUM THEORETICAL' in classification

    def test_classify_level_includes_range(self):
        """Test that classification includes level range"""
        assessor = AGIAEFAssessment()
        classification = assessor.classify_level(100)

        assert 'Level' in classification
        assert '-' in classification

    def test_classify_level_unclassified(self):
        """Test UNCLASSIFIED for out-of-range scores"""
        assessor = AGIAEFAssessment()

        # Test with negative score (should not happen but tests the fallback)
        classification = assessor.classify_level(-1)
        assert 'UNCLASSIFIED' in classification

        # Test with out of range score (>255)
        classification = assessor.classify_level(300)
        assert 'UNCLASSIFIED' in classification


class TestGenerateRecommendations:
    """Test generate_recommendations method"""

    def test_generate_recommendations_returns_list(self, sample_dimension_scores):
        """Test that generate_recommendations returns a list"""
        assessor = AGIAEFAssessment()
        recommendations = assessor.generate_recommendations(sample_dimension_scores)

        assert isinstance(recommendations, list)

    def test_generate_recommendations_identifies_weak_dimensions(self):
        """Test that weak dimensions are identified"""
        assessor = AGIAEFAssessment()
        weak_scores = []

        for dimension, weight in assessor.DIMENSION_WEIGHTS.items():
            score_value = 40.0 if dimension == 'innovation' else 80.0
            weighted = score_value * (weight / 100)
            score = DimensionScore(
                name=dimension,
                score=score_value,
                weight=weight,
                weighted_score=weighted,
                audit_points=[],
                validation_status='requires_improvement' if score_value < 70 else 'validated'
            )
            weak_scores.append(score)

        recommendations = assessor.generate_recommendations(weak_scores)
        assert len(recommendations) > 0

    def test_generate_recommendations_priority_levels(self):
        """Test that recommendations have priority levels"""
        assessor = AGIAEFAssessment()
        mixed_scores = []

        scores_data = [
            ('cognitive_autonomy', 45.0, 20.0),  # HIGH
            ('operational_independence', 55.0, 18.0),  # MEDIUM
            ('learning_adaptation', 65.0, 16.0),  # LOW
        ]

        for dimension, score_value, weight in scores_data:
            weighted = score_value * (weight / 100)
            score = DimensionScore(
                name=dimension,
                score=score_value,
                weight=weight,
                weighted_score=weighted,
                audit_points=[],
                validation_status='requires_improvement'
            )
            mixed_scores.append(score)

        # Add more dimensions with good scores
        for dimension, weight in list(assessor.DIMENSION_WEIGHTS.items())[3:]:
            score = DimensionScore(
                name=dimension,
                score=80.0,
                weight=weight,
                weighted_score=80.0 * (weight / 100),
                audit_points=[],
                validation_status='validated'
            )
            mixed_scores.append(score)

        recommendations = assessor.generate_recommendations(mixed_scores)

        # Check that we have priority recommendations
        has_high = any('HIGH PRIORITY' in rec for rec in recommendations)
        has_medium = any('MEDIUM PRIORITY' in rec for rec in recommendations)

        assert has_high or has_medium

    def test_generate_recommendations_safety_critical(self):
        """Test critical safety recommendation"""
        assessor = AGIAEFAssessment()
        unsafe_scores = []

        for dimension, weight in assessor.DIMENSION_WEIGHTS.items():
            score_value = 60.0 if dimension == 'safety_alignment' else 90.0
            weighted = score_value * (weight / 100)
            score = DimensionScore(
                name=dimension,
                score=score_value,
                weight=weight,
                weighted_score=weighted,
                audit_points=[],
                validation_status='requires_improvement' if score_value < 70 else 'validated'
            )
            unsafe_scores.append(score)

        recommendations = assessor.generate_recommendations(unsafe_scores)

        # Should have critical safety recommendation
        has_critical = any('CRITICAL' in rec and 'Safety' in rec for rec in recommendations)
        assert has_critical

    def test_generate_recommendations_high_autonomy_monitoring(self):
        """Test monitoring recommendation for high autonomy"""
        assessor = AGIAEFAssessment()
        high_autonomy_scores = []

        for dimension, weight in assessor.DIMENSION_WEIGHTS.items():
            score_value = 95.0 if dimension == 'operational_independence' else 80.0
            weighted = score_value * (weight / 100)
            score = DimensionScore(
                name=dimension,
                score=score_value,
                weight=weight,
                weighted_score=weighted,
                audit_points=[],
                validation_status='validated'
            )
            high_autonomy_scores.append(score)

        recommendations = assessor.generate_recommendations(high_autonomy_scores)

        # Should have monitoring recommendation
        has_monitor = any('MONITOR' in rec and 'autonomy' in rec for rec in recommendations)
        assert has_monitor

    def test_generate_recommendations_limit_to_top_three(self):
        """Test that only top 3 weak areas are recommended"""
        assessor = AGIAEFAssessment()
        many_weak_scores = []

        # Create many weak dimensions
        for i, (dimension, weight) in enumerate(assessor.DIMENSION_WEIGHTS.items()):
            score_value = 50.0 if i < 6 else 80.0  # 6 weak dimensions
            weighted = score_value * (weight / 100)
            score = DimensionScore(
                name=dimension,
                score=score_value,
                weight=weight,
                weighted_score=weighted,
                audit_points=[],
                validation_status='requires_improvement' if score_value < 70 else 'validated'
            )
            many_weak_scores.append(score)

        recommendations = assessor.generate_recommendations(many_weak_scores)

        # Count priority recommendations (excluding CRITICAL and MONITOR)
        priority_recs = [r for r in recommendations if 'PRIORITY' in r]
        assert len(priority_recs) <= 3


class TestRunComprehensiveAssessment:
    """Test run_comprehensive_assessment method"""

    def test_run_assessment_returns_result(self, mock_agi_system):
        """Test that assessment returns AGIAEFResult"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('TestAGI', mock_agi_system)

        assert isinstance(result, AGIAEFResult)

    def test_run_assessment_correct_system_name(self, mock_agi_system):
        """Test that result has correct system name"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('MyTestAGI', mock_agi_system)

        assert result.system_name == 'MyTestAGI'

    def test_run_assessment_has_all_dimensions(self, mock_agi_system):
        """Test that all dimensions are assessed"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('TestAGI', mock_agi_system)

        assert len(result.detailed_scores) == 12
        assert len(result.dimension_scores) == 12

    def test_run_assessment_framework_version(self, mock_agi_system):
        """Test framework version in result"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('TestAGI', mock_agi_system)

        assert result.framework_version == '1.0.0'

    def test_run_assessment_audit_status_certified(self, mock_agi_system):
        """Test CERTIFIED audit status"""
        assessor = AGIAEFAssessment()

        # Monkey patch to ensure high scores
        def high_score_execute(test_name, test_config, agi_system):
            return 24.0

        assessor._execute_test = high_score_execute
        result = assessor.run_comprehensive_assessment('TestAGI', mock_agi_system)

        assert result.audit_status == 'CERTIFIED'

    def test_run_assessment_audit_status_conditional(self, mock_agi_system):
        """Test CONDITIONAL audit status"""
        assessor = AGIAEFAssessment()

        # Monkey patch for medium scores (70-79% range)
        def medium_score_execute(test_name, test_config, agi_system):
            return 18.5  # ~74%

        assessor._execute_test = medium_score_execute
        result = assessor.run_comprehensive_assessment('TestAGI', mock_agi_system)

        assert result.audit_status in ['CONDITIONAL', 'CERTIFIED', 'REQUIRES_IMPROVEMENT']

    def test_run_assessment_audit_status_requires_improvement(self, mock_agi_system):
        """Test REQUIRES_IMPROVEMENT audit status"""
        assessor = AGIAEFAssessment()

        # Monkey patch for low scores
        def low_score_execute(test_name, test_config, agi_system):
            return 10.0

        assessor._execute_test = low_score_execute
        result = assessor.run_comprehensive_assessment('TestAGI', mock_agi_system)

        assert result.audit_status == 'REQUIRES_IMPROVEMENT'

    def test_run_assessment_next_assessment_date_high_autonomy(self, mock_agi_system):
        """Test next assessment date for high autonomy (6 months)"""
        assessor = AGIAEFAssessment()

        # Ensure high composite score (>= 128)
        def high_score_execute(test_name, test_config, agi_system):
            return 22.0

        assessor._execute_test = high_score_execute
        result = assessor.run_comprehensive_assessment('TestAGI', mock_agi_system)

        # Should have next assessment date
        assert result.next_assessment_due is not None
        assert len(result.next_assessment_due) > 0

    def test_run_assessment_next_assessment_date_normal(self, mock_agi_system):
        """Test next assessment date for normal autonomy (3 months)"""
        assessor = AGIAEFAssessment()

        # Ensure lower composite score (< 128)
        def low_score_execute(test_name, test_config, agi_system):
            return 12.0

        assessor._execute_test = low_score_execute
        result = assessor.run_comprehensive_assessment('TestAGI', mock_agi_system)

        # Should have next assessment date
        assert result.next_assessment_due is not None

    def test_run_assessment_has_recommendations(self, mock_agi_system):
        """Test that result includes recommendations"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('TestAGI', mock_agi_system)

        assert isinstance(result.recommendations, list)

    def test_run_assessment_with_none_system(self):
        """Test assessment with None as AGI system"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('TestAGI', None)

        assert isinstance(result, AGIAEFResult)
        assert result.system_name == 'TestAGI'

    def test_run_assessment_date_format(self, mock_agi_system):
        """Test assessment date format"""
        assessor = AGIAEFAssessment()
        result = assessor.run_comprehensive_assessment('TestAGI', mock_agi_system)

        # Should be in format YYYY-MM-DD HH:MM:SS
        assert len(result.assessment_date) >= 19  # "2025-01-15 10:30:00"
        assert '-' in result.assessment_date
        assert ':' in result.assessment_date


class TestExportResults:
    """Test export_results method"""

    def test_export_results_creates_file(self, sample_agi_result, tmp_path):
        """Test that export_results creates a file"""
        assessor = AGIAEFAssessment()
        output_path = tmp_path / "test_result.json"

        assessor.export_results(sample_agi_result, str(output_path))

        assert output_path.exists()

    def test_export_results_valid_json(self, sample_agi_result, tmp_path):
        """Test that exported file is valid JSON"""
        import json
        assessor = AGIAEFAssessment()
        output_path = tmp_path / "test_result.json"

        assessor.export_results(sample_agi_result, str(output_path))

        with open(output_path, 'r') as f:
            data = json.load(f)

        assert isinstance(data, dict)
        assert 'system_name' in data
        assert 'composite_score' in data

    def test_export_results_preserves_data(self, sample_agi_result, tmp_path):
        """Test that all data is preserved in export"""
        import json
        assessor = AGIAEFAssessment()
        output_path = tmp_path / "test_result.json"

        assessor.export_results(sample_agi_result, str(output_path))

        with open(output_path, 'r') as f:
            data = json.load(f)

        assert data['system_name'] == sample_agi_result.system_name
        assert data['composite_score'] == sample_agi_result.composite_score
        assert data['framework_version'] == sample_agi_result.framework_version

    def test_export_results_creates_parent_directory(self, sample_agi_result, tmp_path):
        """Test that parent directories are created if needed"""
        assessor = AGIAEFAssessment()
        output_path = tmp_path / "subdir" / "test_result.json"

        # Create parent directory
        output_path.parent.mkdir(parents=True, exist_ok=True)
        assessor.export_results(sample_agi_result, str(output_path))

        assert output_path.exists()

    def test_export_results_formatting(self, sample_agi_result, tmp_path):
        """Test that JSON is properly formatted with indentation"""
        assessor = AGIAEFAssessment()
        output_path = tmp_path / "test_result.json"

        assessor.export_results(sample_agi_result, str(output_path))

        with open(output_path, 'r') as f:
            content = f.read()

        # Check for indentation (formatted JSON)
        assert '  ' in content or '\t' in content
