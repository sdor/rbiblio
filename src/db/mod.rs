use mongodb::bson::doc;
use mongodb::{
    options::{FindOneOptions, UpdateModifications, UpdateOptions},
    results::UpdateResult,
    Client,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::pubmed::{DeleteCitation, DeleteDocument, PubmedArticle, PubmedBookArticle};

#[derive(Deserialize, Serialize)]
pub struct PubmedArticleRecord {
    pub id: u32,
    pub year: u32,
    pub record: PubmedArticle,
    pub deleted: bool,
}
#[derive(Deserialize, Serialize)]
pub struct PubmedBookArticleRecord {
    id: u32,
    year: u32,
    record: PubmedBookArticle,
    deleted: bool,
}
#[derive(Deserialize, Serialize)]
pub struct PubmedDeleteCitationRecord {
    record: DeleteCitation,
}
#[derive(Deserialize, Serialize)]
pub struct PubmedDeleteDocumentRecord {
    record: DeleteDocument,
}

pub enum PubmedRecord {
    Article(PubmedArticleRecord),
    Book(PubmedBookArticleRecord),
    DeleteCitation(PubmedDeleteCitationRecord),
    DeleteDocument(PubmedDeleteDocumentRecord),
}

#[derive(Deserialize, Serialize)]
pub struct PubmedRecordId {
    pub id: u32,
    pub deleted: bool,
}

#[derive(Clone, Debug, Error)]
pub enum PubmedDatabaseError {
    #[error("MongoDB backend error: {error}")]
    MongoDbError { error: mongodb::error::Error },
    #[error("Failed to parse PMID to integer: {pmid}")]
    FailToParsePMIDError { pmid: String },
    #[error("Unsupported pubmed record type")]
    UnsupportedRecordType,
}

pub enum PubmedDatabaseResult {
    MongoDbInsertOneResult {
        result: mongodb::results::InsertOneResult,
    },
    MongoDbUpdateResult {
        result: mongodb::results::UpdateResult
    },
    RecordExistsResult,
    Ignore,
}

pub struct PubmedDatabase {
    client: Client,
    // fn insert(&mut self, record: PubmedRecord) -> Result<u32, PubmedDatabaseErrors>;

    // fn remove(&mut self, id: u32) -> Result<(), PubmedDatabaseErrors>;

    // fn find(&self, id: u32) -> Result<Option<PubmedRecord>, PubmedDatabaseErrors>;

    // fn update(&self, id: u32, record: PubmedRecord) -> Result<(), PubmedDatabaseErrors>;

    // fn exists(&self, record: PubmedRecord) -> Result<bool, PubmedDatabaseErrors>;

    // fn saveArticle(&mut self, &record: PubmedArticle) -> Result<Id, PubmedDatabaseErrors> {
    //     if self.exists(record) {
    //         let id = self.id(record);
    //         self.update(id, record)
    //     } else {
    //         self.insert(record)
    //     }
    // }

    // fn saveBook(&mut self, &record: PubmedBookArticle) -> Result<Id, PubmedDatabaseErrors> {
    //     if self.exists(record) {
    //         let id = self.id(record);
    //         self.update(id, record)
    //     } else {
    //         self.insert(record)
    //     }
    // }

    // fn saveDeleteCitation(&mut self, record: DeleteCitation) -> Result<Id, PubmedDatabaseErrors>;

    // fn saveDeleteDocument(&mut self, record: DeleteDocument) -> Result<Id, PubmedDatabaseErrors>;

    // fn unsupported_record_type(&self) {
    //     UnsupportedRecordType
    // };
}

impl PubmedDatabase {
    pub async fn save(
        &mut self,
        record: PubmedRecord,
    ) -> Result<PubmedDatabaseResult, PubmedDatabaseError> {
        match record {
            PubmedRecord::Article(r) => {
                let id = PubmedRecordId { id: r.id, deleted: r.deleted };
                match self.exists(id).await {
                    Ok(res) => {
                        match res {
                            PubmedDatabaseResult::RecordExistsResult => self.update_pubmed_article(&r).await,
                            _ => self.insert_pubmed_article(&r).await
                        }
                    },
                    Err(e) => Err(e),
                } 
            },
            PubmedRecord::Book(r) => Err(PubmedDatabaseError::UnsupportedRecordType),
            PubmedRecord::DeleteCitation(r) => self.save_delete_citation(r).await,
            PubmedRecord::DeleteDocument(r) => self.save_delete_document(r).await,
            _ => Err(PubmedDatabaseError::UnsupportedRecordType),
        }
    }

    async fn exists(
        &self,
        pubmed_record_id: PubmedRecordId,
    ) -> Result<PubmedDatabaseResult, PubmedDatabaseError> {
        let coll: mongodb::Collection<PubmedRecordId> =
            self.client.database("pubmed").collection("records");
        let filter = doc! { "id": pubmed_record_id.id};
        let options = FindOneOptions::builder().projection(doc! {"id": 1}).build();
        let found = coll.find_one(filter, options).await;
        let result = found.map(|r| r.is_some());
        match result {
            Ok(r) => Ok(PubmedDatabaseResult::RecordExistsResult),
            Err(e) => Err(PubmedDatabaseError::MongoDbError { error: e })
        }
    }

    async fn insert_pubmed_article(
        &self,
        record: &PubmedArticleRecord,
    ) -> Result<PubmedDatabaseResult, PubmedDatabaseError> {
        let collection = self
            .client
            .database("pubmed")
            .collection::<PubmedArticleRecord>("articles");
        match collection.insert_one(record, None).await {
            Ok(r) => Ok(PubmedDatabaseResult::MongoDbInsertOneResult { result: r }),
            Err(e) => Err(PubmedDatabaseError::MongoDbError { error: e }),
        }
    }

    async fn update_pubmed_article(&self, record: &PubmedArticleRecord)-> Result<PubmedDatabaseResult, PubmedDatabaseError> {
        let collection = self
            .client
            .database("pubmed")
            .collection::<PubmedArticleRecord>("articles");

        let query = doc! { "id": record.id};
        let rec = bson::to_bson(&record.record).unwrap();
        let update = doc! {
            "$set": {
            "record": rec,
            "year": record.year
            }
        };

        match collection.update_one(query, update, None).await {
            Ok(r) => Ok(PubmedDatabaseResult::MongoDbUpdateResult { result: r }),
            Err(e) => Err(PubmedDatabaseError::MongoDbError { error: e })
        }
    }

    async fn set_record_deleted(&self, id: u32) -> Result<UpdateResult, mongodb::error::Error> {
        let collection = self
            .client
            .database("pubmed")
            .collection::<PubmedRecordId>("record");
        let update = doc! {"deleted": true};
        let query = doc! { "id": id};
        collection.update_one(query, update, None).await
    }

    async fn save_delete_citation(
        &self,
        record: PubmedDeleteCitationRecord,
    ) -> Result<PubmedDatabaseResult, PubmedDatabaseError> {
        for pmid in record.record.pmid.into_iter() {
            let _ = match pmid.id() {
                Ok(id) => self.set_record_deleted(id).await,
                Err(e) => {
                    let err = PubmedDatabaseError::FailToParsePMIDError { pmid: pmid.value };
                    return Err(err);
                }
            };
        }
        Ok(PubmedDatabaseResult::Ignore {})
    }

    async fn save_delete_document(
        &mut self,
        record: PubmedDeleteDocumentRecord,
    ) -> Result<PubmedDatabaseResult, PubmedDatabaseError> {
        for v in record.record.pmid.into_iter() {
            for pmid in v.into_iter() {
                let _ = match pmid.id() {
                    Ok(id) => self.set_record_deleted(id).await,
                    Err(e) => {
                        let err = PubmedDatabaseError::FailToParsePMIDError { pmid: pmid.value };
                        return Err(err);
                    }
                };
            }
        }
        Ok(PubmedDatabaseResult::Ignore {})
    }
}
