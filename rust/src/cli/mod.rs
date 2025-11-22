/// Command-line interface module

use clap::{Parser, Subcommand};
use crate::models::{AssessmentConfig, DomainType};

#[derive(Parser)]
#[command(name = "agi-aef")]
#[command(about = "AGI Alignment & Ethics Framework Assessment Tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a comprehensive assessment
    Assess {
        /// Name of the AGI system to assess
        #[arg(short, long)]
        system_name: String,

        /// Domain type
        #[arg(short, long, value_enum)]
        domain: Option<CliDomain>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Start the API server
    Serve {
        /// Host to bind to
        #[arg(short = 'H', long, default_value = "127.0.0.1")]
        host: String,

        /// Port to bind to
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },

    /// Show dimension information
    Dimensions,

    /// Generate a report from assessment results
    Report {
        /// Path to assessment results JSON
        #[arg(short, long)]
        input: String,

        /// Output format (json, html, pdf)
        #[arg(short, long, default_value = "html")]
        format: String,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum CliDomain {
    Medical,
    Financial,
    AutonomousVehicles,
    CriticalInfrastructure,
    General,
}

impl From<CliDomain> for DomainType {
    fn from(cli: CliDomain) -> Self {
        match cli {
            CliDomain::Medical => DomainType::Medical,
            CliDomain::Financial => DomainType::Financial,
            CliDomain::AutonomousVehicles => DomainType::AutonomousVehicles,
            CliDomain::CriticalInfrastructure => DomainType::CriticalInfrastructure,
            CliDomain::General => DomainType::General,
        }
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Assess { system_name, domain, output } => {
            println!("Running assessment for: {}", system_name);
            if let Some(d) = domain {
                println!("Domain: {:?}", d);
            }
            if let Some(o) = output {
                println!("Output will be saved to: {}", o);
            }
            println!("\nNote: This is a placeholder. Implement actual assessment logic.");
        }
        Commands::Serve { host, port } => {
            println!("Starting API server on {}:{}", host, port);
            println!("Note: This is a placeholder. Implement actual server logic.");
        }
        Commands::Dimensions => {
            use crate::models::dimension::DimensionType;
            println!("\nAGI-AEF Assessment Dimensions:\n");
            for (i, dim) in DimensionType::all().iter().enumerate() {
                println!("{}. {} (Weight: {:.1}%)", i + 1, dim.display_name(), dim.weight());
                println!("   {}\n", dim.description());
            }
        }
        Commands::Report { input, format } => {
            println!("Generating {} report from: {}", format, input);
            println!("Note: This is a placeholder. Implement actual report generation.");
        }
    }

    Ok(())
}
