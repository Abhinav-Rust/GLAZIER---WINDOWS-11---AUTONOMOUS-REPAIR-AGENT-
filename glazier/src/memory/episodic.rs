use anyhow::Result;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicCase {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub target: String,
    pub symptoms: Vec<String>,
    pub hypothesis: String,
    pub actions: Vec<String>,
    pub outcome: Outcome,
    pub guna_start: crate::wrl::ast::GunaType,
    pub guna_end: crate::wrl::ast::GunaType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
    Success,
    Failure,
    Partial,
}

pub struct EpisodicMemory {
    db: Surreal<Db>,
}

impl EpisodicMemory {
    pub async fn new(path: &str) -> Result<Self> {
        let db = Surreal::new::<RocksDb>(path).await?;
        db.use_ns("glazier").use_db("episodic").await?;
        Ok(Self { db })
    }

    pub async fn init_schema(&self) -> Result<()> {
        let schema_query = include_str!("schema.surql");
        self.db.query(schema_query).await?;
        Ok(())
    }

    pub async fn store_case(&self, case: EpisodicCase) -> Result<()> {
        let _created: Option<EpisodicCase> = self
            .db
            .create(("case", case.id.clone()))
            .content(case)
            .await?;
        Ok(())
    }

    pub async fn get_case(&self, id: &str) -> Result<Option<EpisodicCase>> {
        let case: Option<EpisodicCase> = self.db.select(("case", id)).await?;
        Ok(case)
    }

    pub async fn find_cases_by_target(&self, target: &str) -> Result<Vec<EpisodicCase>> {
        let mut response = self
            .db
            .query("SELECT * FROM case WHERE string::lowercase(target) = string::lowercase($target)")
            .bind(("target", target.to_string()))
            .await?;
        let cases: Vec<EpisodicCase> = response.take(0)?;
        Ok(cases)
    }
}
