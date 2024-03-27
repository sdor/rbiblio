// extern crate dotenv;
use quick_xml::de::Deserializer;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

use quick_xml::reader::Reader;
use quick_xml::Error;
use quick_xml::Writer;
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;
use std::path::Path;
use tokio::sync::mpsc;
use tokio::task;

use std::fs::FileType;
// use std::io::{self, prelude::*};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PMID {
    #[serde(rename = "@Version")]
    version: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Title {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Abbreviation {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Journal {
    #[serde(rename = "ISSN")]
    issn: Option<ISSN>,
    #[serde(rename = "JournalIssue")]
    journal_issue: JournalIssue,
    #[serde(rename = "Title")]
    title: Option<Title>,
    #[serde(rename = "ISOAbbreviation")]
    abbreviation: Option<Abbreviation>,
}

impl Journal {
    fn year(&self) -> Result<u32, std::num::ParseIntError> {
        match self.journal_issue.year() {
            Some(v) => str::parse::<u32>(&v),
            _ => str::parse::<u32>(""),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ISSN {
    #[serde(rename = "@IssnType")]
    issn_type: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JournalIssue {
    #[serde(rename = "@CitedMedium")]
    cited_medium: String,
    #[serde(rename = "Volume")]
    volume: Option<Volume>,
    #[serde(rename = "Issue")]
    issue: Option<Issue>,
    #[serde(rename = "PubDate")]
    pubdate: PubDate,
}

impl JournalIssue {
    fn year(&self) -> Option<String> {
        self.pubdate.year()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Volume {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Issue {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PubDate {
    #[serde(rename = "Year")]
    year_op: Option<String>,
    #[serde(rename = "MedlineDate")]
    medline_date_op: Option<MedlineDate>,
}

impl PubDate {
    fn year(&self) -> Option<String> {
        if self.year_op.is_some() {
            self.year_op.clone()
        } else if self.medline_date_op.is_some() {
            let op = self.medline_date_op.clone();
            op.map(|v| v.year())
        } else {
            self.year_op.clone()
        }
    }
}

// <MedlineDate>1998 Mar-Apr</MedlineDate>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MedlineDate {
    #[serde(rename = "$value")]
    value: String,
}

impl MedlineDate {
    fn year(&self) -> String {
        let it = self.value.chars();
        String::from_iter(
            it.skip_while(|c| !c.is_ascii_digit())
                .take_while(|c| c.is_ascii_digit()),
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtticleTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AbstractText {
    #[serde(rename = "@Label")]
    label: Option<String>,
    #[serde(rename = "@NlmCategory")]
    nlm_category: Option<String>,
    #[serde(rename = "$value")]
    value: String,
}

impl AbstractText {
    pub fn text(&self) -> String {
        self.value.clone()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CopyrightInformation {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Abstract {
    //TODO: AbstractText missing
    #[serde(rename = "AbstractText")]
    abstract_text: Option<Vec<AbstractText>>,
    #[serde(rename = "CopyrightInformation")]
    copyright_information: Option<CopyrightInformation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MedlinePgn {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pagination {
    #[serde(rename = "MedlinePgn")]
    medline_pgn: Option<MedlinePgn>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LastName {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForeName {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Initials {
    // #[serde(rename = "$value")]
    #[serde(alias = "value")]
    #[serde(rename(serialize = "value", deserialize = "$value"))]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectiveName {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    #[serde(rename = "@ValidYN")]
    valid_yn: Option<String>,
    #[serde(rename = "@Type")]
    author_type: Option<String>,
    #[serde(rename = "LastName")]
    last_name: Option<LastName>,
    #[serde(rename = "ForeName")]
    fore_name: Option<ForeName>,
    #[serde(rename = "Initials")]
    initials: Option<Initials>,
    #[serde(rename = "CollectiveName")]
    collective_name: Option<CollectiveName>,
    #[serde(rename = "AffiliationInfo")]
    affiliation_info: Option<Vec<AffiliationInfo>>,
}

impl Author {
    // fn affiliations(&self) -> Vec<String> {
    //     match &self.affiliation_info {
    //         Some(info) => {
    //             let it = info.affiliation.iter().map(|a| a.value.clone());
    //             Vec::from_iter(it)
    //         }
    //         None => Vec::new(),
    //     }
    // }

    // fn is_validated(&self) -> bool {
    //     self.valid_flag == "Y"
    // }
}

//AffiliationInfo that contain single empty Affiliation will be
//deserialized as Option<Vec<Affiliation>>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AffiliationInfo {
    #[serde(rename = "Affiliation")]
    affiliation: Option<Vec<Affiliation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Affiliation {
    #[serde(rename = "$value")]
    value: String,
}

impl Author {
    // fn last_name(&self) -> String { self.the_last_name.value.clone() }
    // fn fore_name(&self) -> Option<String> { self.the_fore_name.value.clone() }
    // fn initials(&self) -> String { self.the_initials.value.clone() }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorList {
    #[serde(rename = "@CompleteYN")]
    complete_flag: String,
    #[serde(rename = "Author")]
    authors: Vec<Author>,
}

impl AuthorList {
    // fn is_complete(&self) -> bool {
    //     self.complete_flag == "Y"
    // }
}

// <Language>eng</Language>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Language {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicationType {
    #[serde(rename = "@UI")]
    ui: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicationTypeList {
    #[serde(rename = "PublicationType")]
    publication_type: Vec<PublicationType>,
}

// <ELocationID EIdType="doi" ValidYN="Y">10.1093/ndt/gfw079</ELocationID>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ELocationID {
    #[serde(rename = "@EIdType")]
    eid_type: String,
    #[serde(rename = "@ValidYN")]
    valid_flag: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GrantID {
    #[serde(rename = "$value")]
    value: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Acronym {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Agency {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grant {
    #[serde(rename = "GrantID")]
    grant_id: Option<GrantID>,
    #[serde(rename = "Acronym")]
    acronym: Option<Acronym>,
    //TODO: Agency is missing
    #[serde(rename = "Agency")]
    agency: Option<Agency>,
    //TODO: Contry missing
    #[serde(rename = "Country")]
    country: Option<Country>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GrantList {
    #[serde(rename = "@CompleteYN")]
    complete_yn: Option<String>,
    #[serde(rename = "Grant")]
    grant: Vec<Grant>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VernacularTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticleDate {
    #[serde(rename = "Year")]
    year: String,
    #[serde(rename = "Month")]
    month: String,
    #[serde(rename = "Day")]
    day: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    #[serde(rename = "@PubModel")]
    pub_model: String,
    #[serde(rename = "Journal")]
    journal: Journal,
    #[serde(rename = "ArticleTitle")]
    article_title: Option<AtticleTitle>,
    #[serde(rename = "Pagination")]
    pagination: Option<Pagination>,
    #[serde(rename = "ELocationID")]
    elocation_id: Option<Vec<ELocationID>>,
    #[serde(rename = "Abstract")]
    summary: Option<Abstract>,
    #[serde(rename = "AuthorList")]
    author_list: Option<AuthorList>,
    #[serde(rename = "Language")]
    language: Vec<Language>,
    #[serde(rename = "DataBankList")]
    data_bank_list: Option<DataBankList>,
    #[serde(rename = "GrantList")]
    grant_list: Option<GrantList>,
    #[serde(rename = "PublicationTypeList")]
    publication_type_list: Option<PublicationTypeList>,
    #[serde(rename = "VernacularTitle")]
    vernacular_title: Option<VernacularTitle>,
    #[serde(rename = "ArticleDate")]
    article_date: Option<Vec<ArticleDate>>,
}

impl Article {
    fn year(&self) -> Result<u32, std::num::ParseIntError> {
        self.journal.year()
    }

    // fn authors(&self) -> Vec<Author> {
    //     match &self.author_list {
    //         Some(list) => list.authors.clone(),
    //         _ => Vec::new(),
    //     }
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Country {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MedlineTA {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NlmUniqueID {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ISSNLinking {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MedlineJournalInfo {
    #[serde(rename = "Country")]
    country: Option<Country>,
    #[serde(rename = "MedlineTA")]
    medline_ta: Option<MedlineTA>,
    #[serde(rename = "NlmUniqueID")]
    nlm_unique_id: NlmUniqueID,
    #[serde(rename = "ISSNLinking")]
    issn_linking: Option<ISSNLinking>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DescriptorName {
    #[serde(rename = "@UI")]
    ui: String,
    #[serde(rename = "@MajorTopicYN")]
    major_topic_yn: Option<String>,
    #[serde(rename = "@Type")]
    descriptor_type: Option<String>,

    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QualifierName {
    #[serde(rename = "@UI")]
    ui: String,
    #[serde(rename = "@MajorTopicYN")]
    major_topic_yn: Option<String>,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshHeading {
    #[serde(rename = "DescriptorName")]
    descriptor_name: DescriptorName,
    #[serde(rename = "QualifierName")]
    qualifier_name: Option<Vec<QualifierName>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshHeadingList {
    #[serde(rename = "MeshHeading")]
    mesh_heading: Vec<MeshHeading>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistryNumber {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameOfSubstance {
    #[serde(rename = "@UI")]
    ui: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chemical {
    #[serde(rename = "RegistryNumber")]
    registry_number: RegistryNumber,
    #[serde(rename = "NameOfSubstance")]
    name_of_substance: NameOfSubstance,
}

impl Chemical {
    pub fn get_registry_number(&self) -> String {
        self.registry_number.value.clone()
    }

    pub fn get_name_of_substance(&self) -> String {
        self.name_of_substance.value.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticleChemical {
   pub pmid: u32,
   pub registry_number: String,
   pub name_of_substance: String,
   pub year: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChemicalList {
    #[serde(rename = "Chemical")]
    chemical: Vec<Chemical>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keyword {
    #[serde(rename = "@MajorTopicYN")]
    major_topic_yn: Option<String>,
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeywordList {
    #[serde(rename = "@Owner")]
    owner: Option<String>,
    #[serde(rename = "Keyword")]
    keyword: Option<Vec<Keyword>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataBankName {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessionNumber {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessionNumberList {
    #[serde(rename = "AccessionNumber")]
    accession_number: Option<Vec<AccessionNumber>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataBank {
    #[serde(rename = "DataBankName")]
    data_bank_name: Option<DataBankName>,
    #[serde(rename = "AccessionNumberList")]
    accession_number_list: AccessionNumberList,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataBankList {
    #[serde(rename = "@CompleteYN")]
    complete_flag: String,
    #[serde(rename = "DataBank")]
    data_bank: Vec<DataBank>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Year {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Month {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Day {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateCompleted {
    #[serde(rename = "Year")]
    year: Year,
    #[serde(rename = "Month")]
    month: Month,
    #[serde(rename = "Day")]
    day: Day,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateRevised {
    #[serde(rename = "Year")]
    year: Year,
    #[serde(rename = "Month")]
    month: Month,
    #[serde(rename = "Day")]
    day: Day,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupplMeshName {
    #[serde(rename = "Type")]
    supp_mesh_name_type: Option<String>,
    #[serde(rename = "@UI")]
    ui: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupplMeshList {
    #[serde(rename = "SupplMeshName")]
    suppl_mesh_name: Vec<SupplMeshName>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CitationSubset {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefSource {
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    #[serde(rename = "$value")]
    note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentsCorrections {
    #[serde(rename = "@RefType")]
    ref_type: String,

    #[serde(rename = "RefSource")]
    ref_source: Option<RefSource>,
    #[serde(rename = "PMID")]
    pmid: Option<PMID>,
    #[serde(rename = "Note")]
    note: Option<Note>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentsCorrectionsList {
    #[serde(rename = "CommentsCorrections")]
    comments_corrections: Vec<CommentsCorrections>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneSymbol {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneSymbolList {
    #[serde(rename = "GeneSymbol")]
    gene_symbol: Vec<GeneSymbol>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersonalNameSubject {
    #[serde(rename = "LastName")]
    last_name: LastName,
    #[serde(rename = "ForeName")]
    fore_name: Option<String>,
    #[serde(rename = "Initials")]
    initials: Option<String>,
    #[serde(rename = "Suffix")]
    suffix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersonalNameSubjectList {
    #[serde(rename = "PersonalNameSubject")]
    personal_name_subject: Vec<PersonalNameSubject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OtherID {
    #[serde(rename = "@Source")]
    source: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OtherAbstract {
    #[serde(rename = "@Type")]
    other_abstract_type: String,
    #[serde(rename = "@Language")]
    language: Option<String>,
    //TODO: AbstractText missing in some records
    #[serde(rename = "AbstractText")]
    abstract_text: Option<Vec<AbstractText>>,
    #[serde(rename = "CopyrightInformation")]
    copyright_information: Option<CopyrightInformation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MedlineCitation {
    #[serde(rename = "@Status")]
    status: String,

    #[serde(rename = "@VersionID")]
    version_id: Option<String>,

    #[serde(rename = "@VersionDate")]
    version_date: Option<String>,

    #[serde(rename = "@IndexingMethod")]
    indexing_method: Option<String>,

    #[serde(rename = "@Owner")]
    owner: Option<String>,

    #[serde(rename = "PMID")]
    pmid: PMID,

    #[serde(rename = "DateCompleted")]
    date_completed: Option<DateCompleted>,

    #[serde(rename = "DateRevised")]
    date_revised: Option<DateCompleted>,

    #[serde(rename = "Article")]
    article: Article,

    #[serde(rename = "MedlineJournalInfo")]
    medline_journal_info: MedlineJournalInfo,

    #[serde(rename = "ChemicalList")]
    chemical_list: Option<ChemicalList>,

    #[serde(rename = "SupplMeshList")]
    suppl_mesh_list: Option<SupplMeshList>,

    #[serde(rename = "CitationSubset")]
    citation_subset: Option<Vec<CitationSubset>>,

    #[serde(rename = "CommentsCorrectionsList")]
    comments_corrections_list: Option<CommentsCorrectionsList>,

    #[serde(rename = "GeneSymbolList")]
    gene_symbol_list: Option<GeneSymbolList>,

    #[serde(rename = "MeshHeadingList")]
    mesh_heading_list: Option<MeshHeadingList>,

    #[serde(rename = "NumberOfReferences")]
    number_of_references: Option<String>,

    #[serde(rename = "PersonalNameSubjectList")]
    personal_name_subject_list: Option<PersonalNameSubjectList>,

    #[serde(rename = "OtherID")]
    other_id: Option<Vec<OtherID>>,

    #[serde(rename = "OtherAbstract")]
    other_abstract: Option<Vec<OtherAbstract>>,

    #[serde(rename = "KeywordList")]
    keyword_list: Option<Vec<KeywordList>>,
    //TODO: CoiStatement?, SpaceFlightMission*, InvestigatorList?, GeneralNote*
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticleId {
    #[serde(rename = "@IdType")]
    id_type: String,
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticleIdList {
    #[serde(rename = "ArticleId")]
    article_ids: Option<Vec<ArticleId>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reference {
    #[serde(rename = "Citation")]
    citation: Option<String>,
    #[serde(rename = "ArticleIdList")]
    article_id_list: Option<ArticleIdList>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReferenceList {
    #[serde(rename = "@Title")]
    title: Option<String>,
    #[serde(rename = "Reference")]
    reference: Option<Vec<Reference>>,
    #[serde(rename = "ReferenceList")]
    reference_list: Option<Vec<ReferenceList>>,
}

impl ReferenceList {
    // fn pubmed_ids(&self) -> Vec<u32> {
    //     if self.references.is_some() {

    //         let it = self.references.as_ref().unwrap().iter();
    //         let found = it
    //             .map(|i| i.pubmed_id())
    //             .filter(|op| op.is_some())
    //             .map(|op| op.unwrap());
    //         Vec::from_iter(found)
    //     } else {
    //         Vec::new()
    //     }
    // }
}

// <PublicationStatus>ppublish</PublicationStatus>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicationStatus {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hour {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Minute {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Second {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PubMedPubDate {
    #[serde(rename = "@PubStatus")]
    pub_status: String,
    #[serde(rename = "Year")]
    year: Year,
    #[serde(rename = "Month")]
    month: Month,
    #[serde(rename = "Day")]
    day: Day,
    #[serde(rename = "Hour")]
    hour: Option<Hour>,
    #[serde(rename = "Minute")]
    minute: Option<Minute>,
    #[serde(rename = "Second")]
    second: Option<Second>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct History {
    #[serde(rename = "PubMedPubDate")]
    pubmed_pub_date: Vec<PubMedPubDate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Param {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    #[serde(rename = "Type")]
    object_type: String,
    #[serde(rename = "Param")]
    param: Option<Vec<Param>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjectList {
    #[serde(rename = "Object")]
    object: Vec<Object>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PubmedData {
    #[serde(rename = "History")]
    history: Option<History>,
    #[serde(rename = "ReferenceList")]
    reference_list: Option<Vec<ReferenceList>>,
    #[serde(rename = "PublicationStatus")]
    publication_status: PublicationStatus,
    #[serde(rename = "ArticleIdList")]
    article_id_list: ArticleIdList,
    #[serde(rename = "ObjectList")]
    object_list: Option<ObjectList>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PubmedArticle {
    #[serde(rename = "MedlineCitation")]
    medline_citation: MedlineCitation,
    #[serde(rename = "PubmedData")]
    pubmed_data: Option<PubmedData>,
}

impl PubmedArticle {
    /// Returns the pmid of this [`PubmedArticle`].
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn pmid(&self) -> Result<u32, ParseIntError> {
        self.medline_citation.pmid.value.parse::<u32>()
    }
    /// Returns the title of this [`PubmedArticle`].
    pub fn title(&self) -> Option<String> {
        let t = self.medline_citation.article.article_title.clone();
        t.map(|v| v.value)
    }

    pub fn abstact_parts(&self) -> Vec<AbstractText> {
        let summary = self.medline_citation.article.summary.clone();
        let vopt = summary.map(|sum| {
            let text = sum.abstract_text;
            match text {
                Some(v) => v,
                _ => Vec::new(),
            }
        });
        match vopt {
            Some(v) => v,
            _ => Vec::new(),
        }
    }

    pub fn year(&self) -> u32 {
        self.medline_citation.article.year().unwrap()
    }

    pub fn chemicals(&self) -> Vec<ArticleChemical> {
        match self.medline_citation.chemical_list.clone() {
            Some(chemical_list) => {
                let mut result: Vec<ArticleChemical> = Vec::new();
                for c in chemical_list.chemical {
                    let pmid = self.pmid().unwrap();
                    let article_chemical = ArticleChemical {
                        pmid: pmid,
                        registry_number: c.registry_number.value,
                        name_of_substance: c.name_of_substance.value,
                        year: self.year() 
                    };
                    result.push(article_chemical);
                }
                result
            },
            _  => Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublisherName {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublisherLocation {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Publisher {
    #[serde(rename = "PublisherName")]
    publisher_name: PublisherName,
    publisher_location: Option<PublisherLocation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Season {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndingDate {
    #[serde(rename = "Year")]
    year: Year,
    #[serde(rename = "Month")]
    month: Month,
    #[serde(rename = "Day")]
    day: Option<Day>,
    #[serde(rename = "Season")]
    season: Option<Season>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeginningDate {
    #[serde(rename = "Year")]
    year: Year,
    #[serde(rename = "Month")]
    month: Month,
    #[serde(rename = "Day")]
    day: Option<Day>,
    #[serde(rename = "Season")]
    season: Option<Season>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Suffix {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identifier {
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Investigator {
    #[serde(rename = "LastName")]
    lastname: LastName,
    #[serde(rename = "ForeName")]
    fore_name: Option<ForeName>,
    #[serde(rename = "Initials")]
    initials: Option<Initials>,
    #[serde(rename = "Suffix")]
    suffix: Option<Suffix>,
    #[serde(rename = "")]
    identifier: Option<Vec<Identifier>>,
    #[serde(rename = "")]
    affilication_info: Option<Vec<AffiliationInfo>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvestigatorList {
    #[serde(rename = "Investigator")]
    publisher: Investigator,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VolumeTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edition {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Medium {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Isbn {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportNumber {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    //#[serde(rename="")]
    #[serde(rename = "Publisher")]
    publisher: Publisher,
    #[serde(rename = "BookTitle")]
    book_title: BookTitle,
    #[serde(rename = "PubDate")]
    pub_date: PubDate,
    #[serde(rename = "BeginningDate")]
    beginning_date: Option<BeginningDate>,
    #[serde(rename = "EndingDate")]
    ending_date: Option<EndingDate>,
    #[serde(rename = "AuthorList")]
    author_list: Option<Vec<AuthorList>>,
    #[serde(rename = "InvestigatorList")]
    investigator_list: Option<InvestigatorList>,
    #[serde(rename = "Volume")]
    volume: Option<Volume>,
    #[serde(rename = "VolumeTitle")]
    volume_title: Option<VolumeTitle>,
    #[serde(rename = "Edition")]
    edition: Option<Edition>,
    #[serde(rename = "CollectionTitle")]
    collection_title: Option<CollectionTitle>,
    #[serde(rename = "Isbn")]
    isbn: Option<Vec<Isbn>>,
    #[serde(rename = "ELocationID")]
    elocation_id: Option<Vec<ELocationID>>,
    #[serde(rename = "Medium")]
    medium: Option<Medium>,
    #[serde(rename = "ReportNumber")]
    report_number: Option<ReportNumber>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationLabel {
    #[serde(rename = "@Type")]
    location_label_type: Option<String>,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticleTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SectionTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Section {
    location_label: Option<LocationLabel>,
    section_title: SectionTitle,
    section: Option<Vec<Section>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemList {
    #[serde(rename = "Item")]
    item: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sections {
    #[serde(rename = "Section")]
    section: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContributionDate {
    #[serde(rename = "Year")]
    year: Year,

    #[serde(rename = "Month")]
    month: Option<Month>,

    #[serde(rename = "Day")]
    day: Option<Day>,
    #[serde(rename = "Season")]
    season: Option<Season>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookDocument {
    #[serde(rename = "PMID")]
    pmid: PMID,
    #[serde(rename = "ArticleIdList")]
    article_id_list: ArticleIdList,
    #[serde(rename = "Book")]
    book: Book,
    #[serde(rename = "LocationLabel")]
    location_label: Option<Vec<LocationLabel>>,
    #[serde(rename = "ArticleTitle")]
    article_title: Option<ArticleTitle>,
    #[serde(rename = "VernacularTitle")]
    vernacular_title: Option<VernacularTitle>,
    #[serde(rename = "Pagination")]
    pagination: Option<Pagination>,
    #[serde(rename = "Language")]
    language: Option<Vec<Language>>,
    #[serde(rename = "AuthorList")]
    author_list: Option<Vec<AuthorList>>,
    #[serde(rename = "InvestigatorList")]
    investigator_list: Option<InvestigatorList>,
    #[serde(rename = "PublicationType")]
    publication_type: Option<Vec<PublicationType>>,
    #[serde(rename = "Abstract")]
    summary: Option<Abstract>,
    #[serde(rename = "Sections")]
    sections: Option<Sections>,
    #[serde(rename = "KeywordList")]
    keyword_list: Option<Vec<KeywordList>>,
    #[serde(rename = "ContributionDate")]
    contribution_date: Option<ContributionDate>,
    #[serde(rename = "DateRevised")]
    date_revised: Option<DateRevised>,
    #[serde(rename = "GrantList")]
    grant_list: Option<GrantList>,
    #[serde(rename = "ItemList")]
    item_list: Option<Vec<ItemList>>,
    #[serde(rename = "ReferenceList")]
    reference_list: Option<Vec<ReferenceList>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PubmedBookData {
    #[serde(rename = "History")]
    history: Option<History>,

    #[serde(rename = "PublicationStatus")]
    publication_status: PublicationStatus,

    #[serde(rename = "ArticleIdList")]
    article_id_list: ArticleIdList,

    #[serde(rename = "ObjectList")]
    object_list: ObjectList,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PubmedBookArticle {
    #[serde(rename = "BookDocument")]
    book_document: BookDocument,
    #[serde(rename = "PubmedBookData")]
    pubmed_book_data: Option<PubmedBookData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteCitation {
    #[serde(rename = "PMID")]
    pmid: Vec<PMID>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteDocument {
    #[serde(rename = "PMID")]
    pmid: Option<Vec<PMID>>,
}

pub async fn articles(path: String, tx: mpsc::Sender<PubmedArticle>) -> Result<(), Error> {
    // Your function body here...
    let path = Path::new(path.as_str());
    let file = tokio::fs::File::open(path).await?;
    let breader = tokio::io::BufReader::new(file);
    let gz = async_compression::tokio::bufread::GzipDecoder::new(breader);
    let gzreader = tokio::io::BufReader::new(gz);
    let mut reader = Reader::from_reader(gzreader);
    let mut output: Vec<u8> = Vec::new();
    let mut writer = Writer::new(&mut output);
    let mut buf: Vec<u8> = Vec::new();
    let pubmed_article_start_tag = BytesStart::new("PubmedArticle");
    let pubmed_article_end_tag = BytesEnd::new("PubmedArticle");
    let mut depth = 0;
    loop {
        match reader.read_event_into_async(&mut buf).await {
            Ok(Event::Eof) => {
                return Ok({});
            }
            Ok(Event::Start(e)) if e == pubmed_article_start_tag => {
                depth += 1;
                match writer.write_event(Event::Start(e)) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::End(e)) if e == pubmed_article_end_tag => {
                depth -= 1;
                match writer.write_event(Event::End(e)) {
                    Ok(_) => {
                        let mut deserializer =
                            Deserializer::from_str(std::str::from_utf8(writer.get_ref()).unwrap());
                        let article = PubmedArticle::deserialize(&mut deserializer).unwrap();
                        buf.clear();
                        writer.get_mut().clear();
                        tx.send(article).await.expect("Failed to send value");
                        task::yield_now().await; // Yield control back to scheduler
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::Text(t)) if depth > 0 => {
                match writer.write_event(Event::Text(t)) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }

            Ok(Event::Start(e)) if depth > 0 && e.local_name().as_ref() == b"i" => {
                let t = Event::Text(BytesText::new("<i>"));
                match writer.write_event(t) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::Start(e)) if depth > 0 && e.local_name().as_ref() == b"b" => {
                let t = Event::Text(BytesText::new("<b>"));
                match writer.write_event(t) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::Start(e)) if depth > 0 && e.local_name().as_ref() == b"sup" => {
                let t = Event::Text(BytesText::new("<sup>"));
                match writer.write_event(t) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::Start(e)) if depth > 0 && e.local_name().as_ref() == b"sub" => {
                let t = Event::Text(BytesText::new("<sub>"));
                match writer.write_event(t) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::Start(e)) if depth > 0 && e.local_name().as_ref() == b"u" => {
                let t = Event::Text(BytesText::new("<u>"));
                match writer.write_event(t) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::End(e)) if depth > 0 && e.local_name().as_ref() == b"i" => {
                let t = Event::Text(BytesText::new("</i>"));
                match writer.write_event(t) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::End(e)) if depth > 0 && e.local_name().as_ref() == b"b" => {
                let t = Event::Text(BytesText::new("</b>"));
                match writer.write_event(t) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::End(e)) if depth > 0 && e.local_name().as_ref() == b"sup" => {
                let t = Event::Text(BytesText::new("</sup>"));
                match writer.write_event(t) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::End(e)) if depth > 0 && e.local_name().as_ref() == b"sub" => {
                let t = Event::Text(BytesText::new("</sub>"));
                match writer.write_event(t) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::End(e)) if depth > 0 && e.local_name().as_ref() == b"u" => {
                let t = Event::Text(BytesText::new("</u>"));
                match writer.write_event(t) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::Start(e))
                if e.name().prefix().as_ref().is_some()
                    && e.name().prefix().unwrap().as_ref() == b"mml"
                    && depth > 0 =>
            {
                let t = format!("<{}>", std::str::from_utf8(e.name().as_ref()).unwrap());
                match writer.write_event(Event::Text(BytesText::new(&t))) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::End(e))
                if e.name().prefix().as_ref().is_some()
                    && e.name().prefix().unwrap().as_ref() == b"mml"
                    && depth > 0 =>
            {
                let t = format!("&lt/{}>", std::str::from_utf8(e.name().as_ref()).unwrap());
                match writer.write_event(Event::Text(BytesText::new(&t))) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::Start(e)) if depth > 0 && e.local_name().as_ref() == b"DispFormula" => {
                let t = Event::Text(BytesText::new("<DispFormula>"));
                match writer.write_event(t) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::End(e)) if depth > 0 && e.local_name().as_ref() == b"DispFormula" => {
                let t = Event::Text(BytesText::new("</DispFormula>"));
                match writer.write_event(t) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }

            Ok(Event::Start(e)) if depth > 0 => {
                depth += 1;
                match writer.write_event(Event::Start(e)) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(Event::End(e)) if depth > 0 => {
                depth -= 1;
                match writer.write_event(Event::End(e)) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }
}

pub async fn directory_articles(
    path: String,
    tx: mpsc::Sender<PubmedArticle>,
) -> Result<(), std::io::Error> {
    let fspath: &Path = Path::new(path.as_str());
    let mut stream: tokio::fs::ReadDir = tokio::fs::read_dir(&fspath).await.unwrap();

    while let Some(entry) = stream.next_entry().await? {
        let file_type: FileType = entry.file_type().await?;
        if file_type.is_file() {
            let file_path = entry.path();
            let file_extension: &std::ffi::OsStr = file_path.extension().unwrap_or_default();
            if file_extension == "gz" {
                let (producer, mut consumer) = mpsc::channel(10);
                let pubmed_file_path = String::from(entry.path().to_str().unwrap());
                let producer_handle = tokio::spawn(articles(pubmed_file_path, producer));

                while let Some(article) = consumer.recv().await {
                    tx.send(article).await.expect("Failed to send value");
                    task::yield_now().await; // Yield control back to scheduler
                }
                match producer_handle.await {
                    Ok(_) => {}
                    Err(e) => {
                        let cause = e.to_string();

                        // Create an io::Error with a custom error message
                        let err = std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Task join error: {}", cause),
                        );
                        return Err(err);
                    }
                }
            }
        }
    }
    Ok({})
}

//
// use tokio::sync::mpsc;
// use tokio::task;
//async fn producer(tx: mpsc::Sender<i32>) {
//     for i in 0..=5 {
//         tx.send(i).await.expect("Failed to send value");
//         task::yield_now().await; // Yield control back to scheduler
//     }
// }
// async fn main() {
//     let (tx, mut rx) = mpsc::channel(10);

//     // Spawn the producer task
//     let producer_handle = tokio::spawn(producer(tx));

//     // Consume values from the channel
//     while let Some(value) = rx.recv().await {
//         println!("Received: {}", value);
//     }

//     // Await the producer task to finish
//     producer_handle.await.expect("Producer task panicked");
// }

// In this example:

// producer is an asynchronous function that sends values to a channel. After sending each value, it yields control back to the scheduler using task::yield_now().
// In the main function, a channel is created, and the producer task is spawned.
// The main function consumes values from the channel using rx.recv().await. As values are received, they are printed.
// Finally, the main function awaits the completion of the producer task using producer_handle.await.
// This way, you achieve a similar effect to yielding values from a coroutine by using channels and yielding control back to the scheduler when needed.
