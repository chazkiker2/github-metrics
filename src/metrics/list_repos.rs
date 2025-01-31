use async_trait::async_trait;
use stable_eyre::eyre;
use tokio::sync::mpsc::Sender;
use toml::value::Datetime;

use super::{util, Graphql, Producer};

#[derive(Debug)]
pub struct ListReposForOrg {
    graphql: Graphql,
    org_name: String,
    repo_names: Vec<String>,
    start_date: Datetime,
    end_date: Datetime,
}

impl ListReposForOrg {
    pub fn new(
        graphql: Graphql,
        org_name: String,
        repo_names: Vec<String>,
        start_date: Datetime,
        end_date: Datetime,
    ) -> Self {
        ListReposForOrg {
            graphql,
            org_name,
            repo_names,
            start_date,
            end_date,
        }
    }
}

#[async_trait]
impl Producer for ListReposForOrg {
    fn column_names(&self) -> Vec<String> {
        vec![String::from("Repository Name"), String::from("# of PRs")]
    }

    async fn producer_task(mut self, tx: Sender<Vec<String>>) -> Result<(), eyre::Error> {
        for repo in &self.repo_names {
            let count_prs = util::count_pull_requests(
                &mut self.graphql,
                &self.org_name,
                &repo,
                &self.start_date,
                &self.end_date,
            )
            .await?;
            tx.send(vec![repo.to_owned(), count_prs.to_string()])
                .await?;
        }

        Ok(())
    }
}
