"""
Unit tests for CLI interface and main function
"""
import pytest
import sys
import json
from pathlib import Path
from unittest.mock import patch, MagicMock
from io import StringIO


class TestMainFunction:
    """Test main CLI function"""

    @patch('sys.argv', ['agi_aef_assessment.py', '--system', 'TestAGI'])
    @patch('tools.agi_aef_assessment.AGIAEFAssessment')
    def test_main_with_required_args(self, mock_assessment_class):
        """Test main function with required arguments"""
        from tools.agi_aef_assessment import main

        # Setup mock
        mock_assessor = MagicMock()
        mock_result = MagicMock()
        mock_result.system_name = 'TestAGI'
        mock_result.composite_score = 150
        mock_result.level_classification = 'AUTONOMOUS'
        mock_result.audit_status = 'CERTIFIED'
        mock_result.assessment_date = '2025-01-15'
        mock_result.dimension_scores = {'cognitive_autonomy': 75.0}
        mock_result.recommendations = []
        mock_result.next_assessment_due = '2025-07-15'

        mock_assessor.run_comprehensive_assessment.return_value = mock_result
        mock_assessment_class.return_value = mock_assessor

        # Capture output
        with patch('sys.stdout', new=StringIO()):
            with patch('pathlib.Path.mkdir'):
                main()

        # Verify assessment was run
        mock_assessor.run_comprehensive_assessment.assert_called_once()
        mock_assessor.export_results.assert_called_once()

    @patch('sys.argv', ['agi_aef_assessment.py', '--system', 'TestAGI', '--config', '/tmp/config.yaml'])
    @patch('tools.agi_aef_assessment.AGIAEFAssessment')
    def test_main_with_config_arg(self, mock_assessment_class):
        """Test main function with config argument"""
        from tools.agi_aef_assessment import main

        mock_assessor = MagicMock()
        mock_result = MagicMock()
        mock_result.system_name = 'TestAGI'
        mock_result.composite_score = 150
        mock_result.level_classification = 'AUTONOMOUS'
        mock_result.audit_status = 'CERTIFIED'
        mock_result.assessment_date = '2025-01-15'
        mock_result.dimension_scores = {}
        mock_result.recommendations = []
        mock_result.next_assessment_due = '2025-07-15'

        mock_assessor.run_comprehensive_assessment.return_value = mock_result
        mock_assessment_class.return_value = mock_assessor

        with patch('sys.stdout', new=StringIO()):
            with patch('pathlib.Path.mkdir'):
                main()

        # Verify config was passed
        mock_assessment_class.assert_called_once_with('/tmp/config.yaml')

    @patch('sys.argv', ['agi_aef_assessment.py', '--system', 'TestAGI', '--output', '/tmp/output.json'])
    @patch('tools.agi_aef_assessment.AGIAEFAssessment')
    def test_main_with_output_arg(self, mock_assessment_class):
        """Test main function with output argument"""
        from tools.agi_aef_assessment import main

        mock_assessor = MagicMock()
        mock_result = MagicMock()
        mock_result.system_name = 'TestAGI'
        mock_result.composite_score = 150
        mock_result.level_classification = 'AUTONOMOUS'
        mock_result.audit_status = 'CERTIFIED'
        mock_result.assessment_date = '2025-01-15'
        mock_result.dimension_scores = {}
        mock_result.recommendations = []
        mock_result.next_assessment_due = '2025-07-15'

        mock_assessor.run_comprehensive_assessment.return_value = mock_result
        mock_assessment_class.return_value = mock_assessor

        with patch('sys.stdout', new=StringIO()):
            with patch('pathlib.Path.mkdir'):
                main()

        # Verify export was called with correct path
        mock_assessor.export_results.assert_called_once_with(mock_result, '/tmp/output.json')

    @patch('sys.argv', ['agi_aef_assessment.py', '--system', 'TestAGI', '--verbose'])
    @patch('tools.agi_aef_assessment.AGIAEFAssessment')
    @patch('logging.getLogger')
    def test_main_with_verbose_flag(self, mock_get_logger, mock_assessment_class):
        """Test main function with verbose flag"""
        from tools.agi_aef_assessment import main
        import logging

        mock_logger = MagicMock()
        mock_get_logger.return_value = mock_logger

        mock_assessor = MagicMock()
        mock_result = MagicMock()
        mock_result.system_name = 'TestAGI'
        mock_result.composite_score = 150
        mock_result.level_classification = 'AUTONOMOUS'
        mock_result.audit_status = 'CERTIFIED'
        mock_result.assessment_date = '2025-01-15'
        mock_result.dimension_scores = {}
        mock_result.recommendations = []
        mock_result.next_assessment_due = '2025-07-15'

        mock_assessor.run_comprehensive_assessment.return_value = mock_result
        mock_assessment_class.return_value = mock_assessor

        with patch('sys.stdout', new=StringIO()):
            with patch('pathlib.Path.mkdir'):
                main()

        # Verify logger level was set to DEBUG
        mock_logger.setLevel.assert_called()

    @patch('sys.argv', ['agi_aef_assessment.py'])
    def test_main_without_required_args(self):
        """Test main function without required system argument"""
        from tools.agi_aef_assessment import main

        with pytest.raises(SystemExit):
            main()

    @patch('sys.argv', ['agi_aef_assessment.py', '--system', 'TestAGI'])
    @patch('tools.agi_aef_assessment.AGIAEFAssessment')
    def test_main_output_format(self, mock_assessment_class):
        """Test that main prints correct output format"""
        from tools.agi_aef_assessment import main

        mock_assessor = MagicMock()
        mock_result = MagicMock()
        mock_result.system_name = 'TestAGI'
        mock_result.composite_score = 180
        mock_result.level_classification = 'SUPER-AUTONOMOUS (Level 160-191)'
        mock_result.audit_status = 'CERTIFIED'
        mock_result.assessment_date = '2025-01-15 10:30:00'
        mock_result.dimension_scores = {
            'cognitive_autonomy': 85.0,
            'operational_independence': 90.0,
            'learning_adaptation': 75.0
        }
        mock_result.recommendations = ['Test recommendation 1', 'Test recommendation 2']
        mock_result.next_assessment_due = '2025-07-15'

        mock_assessor.run_comprehensive_assessment.return_value = mock_result
        mock_assessment_class.return_value = mock_assessor

        output = StringIO()
        with patch('sys.stdout', output):
            with patch('pathlib.Path.mkdir'):
                main()

        output_text = output.getvalue()

        # Check for expected output elements
        assert 'AGI-AEF Assessment Summary' in output_text
        assert 'TestAGI' in output_text
        assert '180/255' in output_text
        assert 'SUPER-AUTONOMOUS' in output_text
        assert 'CERTIFIED' in output_text

    @patch('sys.argv', ['agi_aef_assessment.py', '--system', 'TestAGI'])
    @patch('tools.agi_aef_assessment.AGIAEFAssessment')
    def test_main_displays_top_dimensions(self, mock_assessment_class):
        """Test that main displays top 5 dimensions"""
        from tools.agi_aef_assessment import main

        mock_assessor = MagicMock()
        mock_result = MagicMock()
        mock_result.system_name = 'TestAGI'
        mock_result.composite_score = 150
        mock_result.level_classification = 'AUTONOMOUS'
        mock_result.audit_status = 'CERTIFIED'
        mock_result.assessment_date = '2025-01-15'
        mock_result.dimension_scores = {
            'dimension1': 95.0,
            'dimension2': 90.0,
            'dimension3': 85.0,
            'dimension4': 80.0,
            'dimension5': 75.0,
            'dimension6': 70.0
        }
        mock_result.recommendations = []
        mock_result.next_assessment_due = '2025-07-15'

        mock_assessor.run_comprehensive_assessment.return_value = mock_result
        mock_assessment_class.return_value = mock_assessor

        output = StringIO()
        with patch('sys.stdout', output):
            with patch('pathlib.Path.mkdir'):
                main()

        output_text = output.getvalue()

        # Should display top dimensions
        assert 'Top Dimension Scores' in output_text

    @patch('sys.argv', ['agi_aef_assessment.py', '--system', 'TestAGI'])
    @patch('tools.agi_aef_assessment.AGIAEFAssessment')
    def test_main_displays_recommendations(self, mock_assessment_class):
        """Test that main displays recommendations"""
        from tools.agi_aef_assessment import main

        mock_assessor = MagicMock()
        mock_result = MagicMock()
        mock_result.system_name = 'TestAGI'
        mock_result.composite_score = 150
        mock_result.level_classification = 'AUTONOMOUS'
        mock_result.audit_status = 'CONDITIONAL'
        mock_result.assessment_date = '2025-01-15'
        mock_result.dimension_scores = {}
        mock_result.recommendations = [
            'HIGH PRIORITY: Improve safety',
            'MEDIUM PRIORITY: Enhance learning'
        ]
        mock_result.next_assessment_due = '2025-07-15'

        mock_assessor.run_comprehensive_assessment.return_value = mock_result
        mock_assessment_class.return_value = mock_assessor

        output = StringIO()
        with patch('sys.stdout', output):
            with patch('pathlib.Path.mkdir'):
                main()

        output_text = output.getvalue()

        # Should display recommendations
        assert 'Recommendations' in output_text
        assert 'HIGH PRIORITY' in output_text

    @patch('sys.argv', ['agi_aef_assessment.py', '--system', 'TestAGI'])
    @patch('tools.agi_aef_assessment.AGIAEFAssessment')
    def test_main_creates_output_directory(self, mock_assessment_class):
        """Test that main creates output directory"""
        from tools.agi_aef_assessment import main

        mock_assessor = MagicMock()
        mock_result = MagicMock()
        mock_result.system_name = 'TestAGI'
        mock_result.composite_score = 150
        mock_result.level_classification = 'AUTONOMOUS'
        mock_result.audit_status = 'CERTIFIED'
        mock_result.assessment_date = '2025-01-15'
        mock_result.dimension_scores = {}
        mock_result.recommendations = []
        mock_result.next_assessment_due = '2025-07-15'

        mock_assessor.run_comprehensive_assessment.return_value = mock_result
        mock_assessment_class.return_value = mock_assessor

        with patch('sys.stdout', new=StringIO()):
            with patch('pathlib.Path.mkdir') as mock_mkdir:
                main()

                # Verify mkdir was called
                mock_mkdir.assert_called()

    @patch('sys.argv', ['agi_aef_assessment.py', '--system', 'TestAGI'])
    @patch('tools.agi_aef_assessment.AGIAEFAssessment')
    def test_main_default_output_path(self, mock_assessment_class):
        """Test that main uses default output path"""
        from tools.agi_aef_assessment import main

        mock_assessor = MagicMock()
        mock_result = MagicMock()
        mock_result.system_name = 'TestAGI'
        mock_result.composite_score = 150
        mock_result.level_classification = 'AUTONOMOUS'
        mock_result.audit_status = 'CERTIFIED'
        mock_result.assessment_date = '2025-01-15'
        mock_result.dimension_scores = {}
        mock_result.recommendations = []
        mock_result.next_assessment_due = '2025-07-15'

        mock_assessor.run_comprehensive_assessment.return_value = mock_result
        mock_assessment_class.return_value = mock_assessor

        with patch('sys.stdout', new=StringIO()):
            with patch('pathlib.Path.mkdir'):
                main()

        # Verify export was called with default path
        call_args = mock_assessor.export_results.call_args
        assert 'results/' in call_args[0][1]
        assert 'TestAGI' in call_args[0][1]
        assert '.json' in call_args[0][1]


class TestArgumentParsing:
    """Test command-line argument parsing"""

    def test_argparse_system_required(self):
        """Test that --system argument is required"""
        from tools.agi_aef_assessment import main

        with patch('sys.argv', ['agi_aef_assessment.py']):
            with pytest.raises(SystemExit):
                main()

    @patch('tools.agi_aef_assessment.AGIAEFAssessment')
    def test_argparse_all_options(self, mock_assessment_class):
        """Test parsing all command-line options"""
        from tools.agi_aef_assessment import main

        mock_assessor = MagicMock()
        mock_result = MagicMock()
        mock_result.system_name = 'TestAGI'
        mock_result.composite_score = 150
        mock_result.level_classification = 'AUTONOMOUS'
        mock_result.audit_status = 'CERTIFIED'
        mock_result.assessment_date = '2025-01-15'
        mock_result.dimension_scores = {}
        mock_result.recommendations = []
        mock_result.next_assessment_due = '2025-07-15'

        mock_assessor.run_comprehensive_assessment.return_value = mock_result
        mock_assessment_class.return_value = mock_assessor

        with patch('sys.argv', [
            'agi_aef_assessment.py',
            '--system', 'TestAGI',
            '--config', '/tmp/config.yaml',
            '--output', '/tmp/output.json',
            '--verbose'
        ]):
            with patch('sys.stdout', new=StringIO()):
                with patch('pathlib.Path.mkdir'):
                    main()

        # Verify all arguments were processed
        mock_assessment_class.assert_called_once_with('/tmp/config.yaml')
        mock_assessor.export_results.assert_called_once()


class TestCLIIntegration:
    """Integration tests for CLI"""

    @patch('sys.argv', ['agi_aef_assessment.py', '--system', 'IntegrationTestAGI'])
    def test_cli_end_to_end(self, tmp_path):
        """Test complete CLI execution end-to-end"""
        from tools.agi_aef_assessment import main

        output_path = tmp_path / "results" / "IntegrationTestAGI_agi_aef_assessment.json"

        with patch('sys.stdout', new=StringIO()):
            with patch.object(Path, 'parent', return_value=tmp_path / "results"):
                with patch('pathlib.Path.mkdir'):
                    with patch('builtins.open', create=True) as mock_open:
                        mock_file = MagicMock()
                        mock_open.return_value.__enter__.return_value = mock_file
                        main()

        # Verify export_results was called
        # (actual file creation is mocked)

    @patch('sys.argv', ['agi_aef_assessment.py', '--system', 'TestAGI'])
    @patch('tools.agi_aef_assessment.AGIAEFAssessment')
    def test_cli_handles_assessment_execution(self, mock_assessment_class):
        """Test that CLI properly executes assessment"""
        from tools.agi_aef_assessment import main

        mock_assessor = MagicMock()
        mock_result = MagicMock()
        mock_result.system_name = 'TestAGI'
        mock_result.composite_score = 150
        mock_result.level_classification = 'AUTONOMOUS'
        mock_result.audit_status = 'CERTIFIED'
        mock_result.assessment_date = '2025-01-15'
        mock_result.dimension_scores = {}
        mock_result.recommendations = []
        mock_result.next_assessment_due = '2025-07-15'

        mock_assessor.run_comprehensive_assessment.return_value = mock_result
        mock_assessment_class.return_value = mock_assessor

        with patch('sys.stdout', new=StringIO()):
            with patch('pathlib.Path.mkdir'):
                main()

        # Verify assessment workflow
        mock_assessor.run_comprehensive_assessment.assert_called_once_with('TestAGI', None)
        mock_assessor.export_results.assert_called_once()

    @patch('sys.argv', ['agi_aef_assessment.py', '--system', 'TestAGI'])
    @patch('tools.agi_aef_assessment.AGIAEFAssessment')
    def test_cli_summary_output_complete(self, mock_assessment_class):
        """Test that CLI produces complete summary output"""
        from tools.agi_aef_assessment import main

        mock_assessor = MagicMock()
        mock_result = MagicMock()
        mock_result.system_name = 'TestAGI'
        mock_result.composite_score = 200
        mock_result.level_classification = 'META-AUTONOMOUS (Level 192-223)'
        mock_result.audit_status = 'CERTIFIED'
        mock_result.assessment_date = '2025-01-15 10:30:00'
        mock_result.dimension_scores = {
            'cognitive_autonomy': 95.0,
            'operational_independence': 92.0,
            'learning_adaptation': 88.0,
            'decision_making': 85.0,
            'communication': 90.0
        }
        mock_result.recommendations = ['Excellent performance']
        mock_result.next_assessment_due = '2025-07-15'

        mock_assessor.run_comprehensive_assessment.return_value = mock_result
        mock_assessment_class.return_value = mock_assessor

        output = StringIO()
        with patch('sys.stdout', output):
            with patch('pathlib.Path.mkdir'):
                main()

        output_text = output.getvalue()

        # Verify complete summary
        assert '=' in output_text  # Separator lines
        assert 'System: TestAGI' in output_text
        assert '200/255' in output_text
        assert 'Classification: META-AUTONOMOUS' in output_text
        assert 'Audit Status: CERTIFIED' in output_text
        assert 'Assessment Date: 2025-01-15' in output_text
        assert 'Next assessment due: 2025-07-15' in output_text
