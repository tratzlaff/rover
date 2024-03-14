use clap::Parser;
use rover_client::operations::subgraph::lint::{self, LintSubgraphInput};
use serde::Serialize;

use crate::options::{GraphRefOpt, LintOpts, ProfileOpt, SchemaOpt, SubgraphOpt};

use crate::utils::client::StudioClientConfig;
use crate::{RoverOutput, RoverResult};

#[derive(Debug, Serialize, Parser)]
pub struct Lint {
    #[clap(flatten)]
    graph: GraphRefOpt,

    #[clap(flatten)]
    subgraph: SubgraphOpt,

    #[clap(flatten)]
    profile: ProfileOpt,

    #[clap(flatten)]
    #[serde(skip_serializing)]
    schema: SchemaOpt,

    #[clap(flatten)]
    lint: LintOpts,
}

impl Lint {
    pub async fn run(&self, client_config: StudioClientConfig) -> RoverResult<RoverOutput> {
        let client = client_config.get_authenticated_client(&self.profile)?;

        let file_with_metadata = self
            .schema
            .read_file_descriptor_with_metadata("SDL", &mut std::io::stdin())?;

        let lint_result: rover_client::shared::LintResponse = lint::run(
            LintSubgraphInput {
                graph_ref: self.graph.graph_ref.clone(),
                file_name: file_with_metadata.file_path,
                proposed_schema: file_with_metadata.schema,
                subgraph_name: self.subgraph.subgraph_name.clone(),
                ignore_existing: self.lint.ignore_existing_lint_violations,
            },
            &client,
        ).await?;

        Ok(RoverOutput::LintResponse(lint_result))
    }
}
