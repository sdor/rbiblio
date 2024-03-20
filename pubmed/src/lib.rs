// extern crate dotenv;
// use flate2::read::GzDecoder;
// use quick_xml::de::Deserializer;
// use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
// use quick_xml::name::QName;
// use quick_xml::reader::Reader;
// use quick_xml::Writer;
use serde::{Deserialize, Serialize};
// use std::fs::{read_dir, File};
// use std::io::{BufRead, BufReader, Write};
// use std::num::ParseIntError;
// use std::path::Path;
// use dotenv::dotenv;
// use std::env;

// extern crate directories;
// use directories::UserDirs;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PMID {
    #[serde(rename = "@Version")]
    version: String,
    #[serde(rename = "$value")]
    value: String,
}

impl PMID {
    fn id(&self) -> Result<u32, std::num::ParseIntError> {
        str::parse::<u32>(&self.value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Title {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Abbreviation {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Journal {
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
struct ISSN {
    #[serde(rename = "@IssnType")]
    issn_type: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct JournalIssue {
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
struct Volume {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Issue {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PubDate {
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
struct MedlineDate {
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
struct AtticleTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AbstractText {
    #[serde(rename = "@Label")]
    label: Option<String>,
    #[serde(rename = "@NlmCategory")]
    nlm_category: Option<String>,
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CopyrightInformation {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Abstract {
    //TODO: AbstractText missing
    #[serde(rename = "AbstractText")]
    abstract_text: Option<Vec<AbstractText>>,
    #[serde(rename = "CopyrightInformation")]
    copyright_information: Option<CopyrightInformation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MedlinePgn {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Pagination {
    #[serde(rename = "MedlinePgn")]
    medline_pgn: Option<MedlinePgn>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct LastName {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ForeName {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Initials {
    // #[serde(rename = "$value")]
    #[serde(alias = "value")]
    #[serde(rename(serialize = "value", deserialize = "$value"))]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CollectiveName {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Author {
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
struct AffiliationInfo {
    #[serde(rename = "Affiliation")]
    affiliation: Option<Vec<Affiliation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Affiliation {
    #[serde(rename = "$value")]
    value: String,
}

impl Author {
    // fn last_name(&self) -> String { self.the_last_name.value.clone() }
    // fn fore_name(&self) -> Option<String> { self.the_fore_name.value.clone() }
    // fn initials(&self) -> String { self.the_initials.value.clone() }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AuthorList {
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
struct Language {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PublicationType {
    #[serde(rename = "@UI")]
    ui: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PublicationTypeList {
    #[serde(rename = "PublicationType")]
    publication_type: Vec<PublicationType>,
}

// <ELocationID EIdType="doi" ValidYN="Y">10.1093/ndt/gfw079</ELocationID>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ELocationID {
    #[serde(rename = "@EIdType")]
    eid_type: String,
    #[serde(rename = "@ValidYN")]
    valid_flag: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GrantID {
    #[serde(rename = "$value")]
    value: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Acronym {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Agency {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Grant {
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
struct GrantList {
    #[serde(rename = "@CompleteYN")]
    complete_yn: Option<String>,
    #[serde(rename = "Grant")]
    grant: Vec<Grant>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct VernacularTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ArticleDate {
    #[serde(rename = "Year")]
    year: String,
    #[serde(rename = "Month")]
    month: String,
    #[serde(rename = "Day")]
    day: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Article {
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
    // fn year(&self) -> Result<u32, std::num::ParseIntError> {
    //     self.journal.year()
    // }

    // fn authors(&self) -> Vec<Author> {
    //     match &self.author_list {
    //         Some(list) => list.authors.clone(),
    //         _ => Vec::new(),
    //     }
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Country {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MedlineTA {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct NlmUniqueID {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ISSNLinking {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MedlineJournalInfo {
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
struct DescriptorName {
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
struct QualifierName {
    #[serde(rename = "@UI")]
    ui: String,
    #[serde(rename = "@MajorTopicYN")]
    major_topic_yn: Option<String>,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MeshHeading {
    #[serde(rename = "DescriptorName")]
    descriptor_name: DescriptorName,
    #[serde(rename = "QualifierName")]
    qualifier_name: Option<Vec<QualifierName>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MeshHeadingList {
    #[serde(rename = "MeshHeading")]
    mesh_heading: Vec<MeshHeading>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RegistryNumber {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NameOfSubstance {
    #[serde(rename = "@UI")]
    ui: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Chemical {
    #[serde(rename = "RegistryNumber")]
    registry_number: RegistryNumber,
    #[serde(rename = "NameOfSubstance")]
    name_of_substance: NameOfSubstance,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChemicalList {
    #[serde(rename = "Chemical")]
    chemical: Vec<Chemical>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Keyword {
    #[serde(rename = "@MajorTopicYN")]
    major_topic_yn: Option<String>,
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct KeywordList {
    #[serde(rename = "@Owner")]
    owner: Option<String>,
    #[serde(rename = "Keyword")]
    keyword: Option<Vec<Keyword>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataBankName {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccessionNumber {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccessionNumberList {
    #[serde(rename = "AccessionNumber")]
    accession_number: Option<Vec<AccessionNumber>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataBank {
    #[serde(rename = "DataBankName")]
    data_bank_name: Option<DataBankName>,
    #[serde(rename = "AccessionNumberList")]
    accession_number_list: AccessionNumberList,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataBankList {
    #[serde(rename = "@CompleteYN")]
    complete_flag: String,
    #[serde(rename = "DataBank")]
    data_bank: Vec<DataBank>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Year {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Month {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Day {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DateCompleted {
    #[serde(rename = "Year")]
    year: Year,
    #[serde(rename = "Month")]
    month: Month,
    #[serde(rename = "Day")]
    day: Day,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct DateRevised {
    #[serde(rename = "Year")]
    year: Year,
    #[serde(rename = "Month")]
    month: Month,
    #[serde(rename = "Day")]
    day: Day,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SupplMeshName {
    #[serde(rename = "Type")]
    supp_mesh_name_type: Option<String>,
    #[serde(rename = "@UI")]
    ui: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SupplMeshList {
    #[serde(rename = "SupplMeshName")]
    suppl_mesh_name: Vec<SupplMeshName>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CitationSubset {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RefSource {
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Note {
    #[serde(rename = "$value")]
    note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CommentsCorrections {
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
struct CommentsCorrectionsList {
    #[serde(rename = "CommentsCorrections")]
    comments_corrections: Vec<CommentsCorrections>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GeneSymbol {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GeneSymbolList {
    #[serde(rename = "GeneSymbol")]
    gene_symbol: Vec<GeneSymbol>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PersonalNameSubject {
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
struct PersonalNameSubjectList {
    #[serde(rename = "PersonalNameSubject")]
    personal_name_subject: Vec<PersonalNameSubject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OtherID {
    #[serde(rename = "@Source")]
    source: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OtherAbstract {
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
struct MedlineCitation {
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

impl MedlineCitation {
    // fn id(&self) -> Result<u32, std::num::ParseIntError> {
    //     self.pmid.id()
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ArticleId {
    #[serde(rename = "@IdType")]
    id_type: String,
    #[serde(rename = "$value")]
    value: Option<String>,
}

impl ArticleId {
    // fn is_pubmed_id(&self) -> bool {
    //     self.id_type == "pubmed"
    // }

    // fn id(&self) -> Option<u32> {
    //     if self.is_pubmed_id() {
    //         match str::parse::<u32>(&self.value) {
    //             Ok(i) => Some(i),
    //             Err(_e) => None,
    //         }
    //     } else {
    //         None
    //     }
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ArticleIdList {
    #[serde(rename = "ArticleId")]
    article_ids: Option<Vec<ArticleId>>,
}

impl ArticleIdList {
    // fn pubmed_id(&self) -> Option<u32> {
    //     let it = self.article_ids.iter();
    //     let mut found = it.filter(|aid| aid.is_pubmed_id()).map(|aid| aid.id());
    //     let r = found.next().flatten();
    //     r
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Reference {
    #[serde(rename = "Citation")]
    citation: Option<String>,
    #[serde(rename = "ArticleIdList")]
    article_id_list: Option<ArticleIdList>,
}

impl Reference {
    // fn pubmed_id(&self) -> Option<u32> {
    //     let mut found = self.article_id_list.iter().map(|list| list.pubmed_id());
    //     let r = found.next().flatten();
    //     r
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ReferenceList {
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
struct PublicationStatus {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Hour {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Minute {
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Second {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PubMedPubDate {
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
struct History {
    #[serde(rename = "PubMedPubDate")]
    pubmed_pub_date: Vec<PubMedPubDate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Param {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "$value")]
    value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Object {
    #[serde(rename = "Type")]
    object_type: String,
    #[serde(rename = "Param")]
    param: Option<Vec<Param>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ObjectList {
    #[serde(rename = "Object")]
    object: Vec<Object>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PubmedData {
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

impl PubmedData {
    // fn pubmed_references(&self) -> Vec<u32> {
    //     match &self.reference_list {
    //         Some(list) => {
    //         let mut v:Vec<u32> = Vec::new();
    //         for rlist in list.iter() {
    //             let mut pmids = rlist.pubmed_ids();
    //             v.append(&mut pmids);
    //         }
    //         v },
    //         _ => Vec::new()
    //     }
    // }

    // fn pmid(&self) -> u32 {
    //     self.article_id_list.pubmed_id().unwrap()
    // }
}

#[derive(Serialize, Deserialize, Debug)]
struct PubmedArticle {
    #[serde(rename = "MedlineCitation")]
    medline_citation: MedlineCitation,
    #[serde(rename = "PubmedData")]
    pubmed_data: Option<PubmedData>,
}

impl PubmedArticle {
    // fn pubmed_id(&self) -> Result<u32, ParseIntError> {
    //     self.medline_citation.id()
    // }

    // fn pubmed_references(&self) -> Vec<u32> {
    //     self.pubmed_data.pubmed_references()
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PublisherName {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PublisherLocation {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Publisher {
    #[serde(rename = "PublisherName")]
    publisher_name: PublisherName,
    publisher_location: Option<PublisherLocation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BookTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Season {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EndingDate {
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
struct BeginningDate {
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
struct Suffix {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Identifier {
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Investigator {
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
struct InvestigatorList {
    #[serde(rename = "Investigator")]
    publisher: Investigator,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct VolumeTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CollectionTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Edition {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Medium {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Isbn {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ReportNumber {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Book {
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
struct LocationLabel {
    #[serde(rename = "@Type")]
    location_label_type: Option<String>,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ArticleTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SectionTitle {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Section {
    location_label: Option<LocationLabel>,
    section_title: SectionTitle,
    section: Option<Vec<Section>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ItemList {
    #[serde(rename = "Item")]
    item: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Sections {
    #[serde(rename = "Section")]
    section: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ContributionDate {
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
struct BookDocument {
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
struct PubmedBookData {
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
struct PubmedBookArticle {
    #[serde(rename = "BookDocument")]
    book_document: BookDocument,
    #[serde(rename = "PubmedBookData")]
    pubmed_book_data: Option<PubmedBookData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DeleteCitation {
    #[serde(rename = "PMID")]
    pmid: Vec<PMID>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DeleteDocument {
    #[serde(rename = "PMID")]
    pmid: Option<Vec<PMID>>,
}

