use flate2::read::GzDecoder;
use quick_xml::de::Deserializer;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::Writer;
use serde::{Deserialize, Serialize};
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;

extern crate directories;
use directories::UserDirs;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PMID {
    #[serde(rename = "@Version")]
    version: String,
    #[serde(rename = "$value")]
    value: Option<String>,
}

impl PMID {
    // fn id(&self) -> Result<u32, std::num::ParseIntError> {
    //     str::parse::<u32>(&self.value)
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Title {
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Abbreviation {
    #[serde(rename = "$value")]
    value: Option<String>,
}
// <Journal>
//     <ISSN IssnType="Print">0017-0011</ISSN>
//     <JournalIssue CitedMedium="Print">
//         <Volume>67</Volume>
//         <Issue>1</Issue>
//         <PubDate>
//             <Year>1996</Year>
//             <Month>Jan</Month>
//         </PubDate>
//     </JournalIssue>
//     <Title>Ginekologia polska</Title>
//     <ISOAbbreviation>Ginekol. Pol.</ISOAbbreviation>
// </Journal>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Journal {
    #[serde(rename = "ISSN")]
    issn: Option<ISSN>,
    #[serde(rename = "JournalIssue")]
    journal_issue: JournalIssue,
    #[serde(rename = "Title")]
    title: Title,
    #[serde(rename = "ISOAbbreviation")]
    abbreviation: Option<Abbreviation>,
}

impl Journal {
    // fn year(&self) -> Result<u32, std::num::ParseIntError> {
    //     match self.journal_issue.year() {
    //         Some(v) => str::parse::<u32>(&v),
    //         _ => str::parse::<u32>(""),
    //     }
    // }
}

//<ISSN IssnType="Print">0095-3814</ISSN>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ISSN {
    #[serde(rename = "@IssnType")]
    issn_type: String,
    #[serde(rename = "$value")]
    value: Option<String>,
}

//<JournalIssue CitedMedium="Print">
//    <Volume>3</Volume>
//    <Issue>1</Issue>
//    <PubDate>
//        <Year>1976</Year>
//        <Season>Fall</Season>
//    </PubDate>
//</JournalIssue>
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
    // fn year(&self) -> Option<String> {
    //     self.pubdate.year()
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Volume {
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Issue {
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PubDate {
    #[serde(rename = "Year")]
    year_op: Option<String>,
    #[serde(rename = "MedlineDate")]
    medline_date_op: Option<MedlineDate>,
}

impl PubDate {
    // fn year(&self) -> Option<String> {
    //     if self.year_op.is_some() {
    //         self.year_op.clone()
    //     } else if self.medline_date_op.is_some() {
    //         let op = self.medline_date_op.clone();
    //         op.map(|v| v.year())
    //     } else {
    //         self.year_op.clone()
    //     }
    // }
}

// <MedlineDate>1998 Mar-Apr</MedlineDate>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MedlineDate {
    #[serde(rename = "$value")]
    value: Option<String>,
}

impl MedlineDate {
    // fn year(&self) -> String {
    //     let it = self.value.chars();
    //     String::from_iter(
    //         it.skip_while(|c| !c.is_ascii_digit())
    //             .take_while(|c| c.is_ascii_digit()),
    //     )
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AtticleTitle {
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AbstractText {
    #[serde(rename = "@Label")]
    label: Option<String>,
    #[serde(rename = "@NlmCategory")]
    nlm_category: Option<String>,
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Summary {
    #[serde(rename = "AbstractText")]
    parts: Vec<AbstractText>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MedlinePgn {
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Pagination {
    #[serde(rename = "MedlinePgn")]
    pages: MedlinePgn,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct LastName {
    #[serde(rename = "$value")]
    value: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ForeName {
    #[serde(rename = "$value")]
    value: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Initials {
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CollectiveName {
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Author {
    #[serde(rename = "@ValidYN")]
    valid_flag: String,
    #[serde(rename = "LastName")]
    the_last_name: Option<LastName>,
    #[serde(rename = "ForeName")]
    the_fore_name: Option<ForeName>,
    #[serde(rename = "Initials")]
    the_initials: Option<Initials>,
    #[serde(rename = "CollectiveName")]
    the_collective: Option<CollectiveName>,
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

    fn is_validated(&self) -> bool {
        self.valid_flag == "Y"
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AffiliationInfo {
    #[serde(rename = "Affiliation")]
    affiliation: Vec<Affiliation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Affiliation {
    #[serde(rename = "$value")]
    value: Option<String>,
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
    fn is_complete(&self) -> bool {
        self.complete_flag == "Y"
    }
}

// <Language>eng</Language>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Language {
    #[serde(rename = "$value")]
    value: Option<String>,
}

// <PublicationType UI="D016446">Consensus Development Conference</PublicationType>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct PublicationType {
    #[serde(rename = "@UI")]
    ui: String,
    #[serde(rename = "$value")]
    value: Option<String>,
}
// <PublicationTypeList>
// <PublicationType UI="D016446">Consensus Development Conference</PublicationType>
// <PublicationType UI="D016447">Consensus Development Conference, NIH</PublicationType>
// <PublicationType UI="D016431">Guideline</PublicationType>
// <PublicationType UI="D016428">Journal Article</PublicationType>
// <PublicationType UI="D016454">Review</PublicationType>
// </PublicationTypeList>
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
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Article {
    #[serde(rename = "Journal")]
    journal: Journal,
    #[serde(rename = "ArticleTitle")]
    title: AtticleTitle,
    #[serde(rename = "Abstract")]
    summary: Option<Summary>,
    #[serde(rename = "Pagination")]
    pagination: Option<Pagination>,
    #[serde(rename = "AuthorList")]
    author_list: Option<AuthorList>,
    #[serde(rename = "Language")]
    language: Vec<Language>,
    #[serde(rename = "DataBankList")]
    data_bank_list: Option<DataBankList>,
    #[serde(rename = "PublicationTypeList")]
    publication_type_list: Option<PublicationTypeList>,
    #[serde(rename = "ELocationID")]
    elocation_id: Option<Vec<ELocationID>>,
}

impl Article {
    // fn year(&self) -> Result<u32, std::num::ParseIntError> {
    //     self.journal.year()
    // }

    fn authors(&self) -> Vec<Author> {
        match &self.author_list {
            Some(list) => list.authors.clone(),
            _ => Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Country {
    #[serde(rename = "$value")]
    value: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MedlineTA {
    #[serde(rename = "$value")]
    value: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct NlmUniqueID {
    #[serde(rename = "$value")]
    value: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ISSNLinking {
    #[serde(rename = "$value")]
    value: Option<String>,
}

// <MedlineJournalInfo>
// <Country>United States</Country>
// <MedlineTA>JAMA</MedlineTA>
// <NlmUniqueID>7501160</NlmUniqueID>
// <ISSNLinking>0098-7484</ISSNLinking>
// </MedlineJournalInfo>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MedlineJournalInfo {
    #[serde(rename = "Country")]
    country: Country,
    #[serde(rename = "MedlineTA")]
    medline_ta: MedlineTA,
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
    major_topic_flag: String,
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct QualifierName {
    #[serde(rename = "@UI")]
    ui: String,
    #[serde(rename = "@MajorTopicYN")]
    major_topic_flag: String,
    #[serde(rename = "$value")]
    value: Option<String>,
}

//<MeshHeading>
//    <DescriptorName UI="D018290" MajorTopicYN="N">Cervical Intraepithelial Neoplasia</DescriptorName>
//    <QualifierName UI="Q000145" MajorTopicYN="N">classification</QualifierName>
//    <QualifierName UI="Q000473" MajorTopicYN="Y">pathology</QualifierName>
//</MeshHeading>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MeshHeading {
    #[serde(rename = "DescriptorName")]
    descriptor_name: DescriptorName,
    #[serde(rename = "QualifierName")]
    qualifier_name: Option<Vec<QualifierName>>,
}

// <MeshHeadingList>
//     <MeshHeading>
//         <DescriptorName UI="D018290" MajorTopicYN="N">Cervical Intraepithelial Neoplasia</DescriptorName>
//         <QualifierName UI="Q000145" MajorTopicYN="N">classification</QualifierName>
//         <QualifierName UI="Q000473" MajorTopicYN="Y">pathology</QualifierName>
//     </MeshHeading>
// </MeshHeadingList>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MeshHeadingList {
    #[serde(rename = "MeshHeading")]
    mesh_heading: Vec<MeshHeading>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RegistryNumber {
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NameOfSubstance {
    #[serde(rename = "@UI")]
    ui: String,
    #[serde(rename = "$value")]
    value: Option<String>,
}

//<Chemical>
//    <RegistryNumber>0</RegistryNumber>
//    <NameOfSubstance UI="D001426">Bacterial Proteins</NameOfSubstance>
//</Chemical>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Chemical {
    #[serde(rename = "RegistryNumber")]
    registry_number: RegistryNumber,
    #[serde(rename = "NameOfSubstance")]
    name_of_substance: NameOfSubstance,
}

// <ChemicalList>
//     <Chemical>
//         <RegistryNumber>0</RegistryNumber>
//         <NameOfSubstance UI="D001426">Bacterial Proteins</NameOfSubstance>
//     </Chemical>
//     <Chemical>
//         <RegistryNumber>0</RegistryNumber>
//         <NameOfSubstance UI="D003598">Cytoskeletal Proteins</NameOfSubstance>
//     </Chemical>
// </ChemicalList>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChemicalList {
    #[serde(rename = "Chemical")]
    chemical: Vec<Chemical>,
}

//<Keyword MajorTopicYN="N">Bioethics and Professional Ethics</Keyword>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Keyword {
    #[serde(rename = "@MajorTopicYN")]
    major_topic_flag: String,
    #[serde(rename = "$value")]
    value: Option<Vec<String>>,
}

// <KeywordList Owner="KIE">
//   <Keyword MajorTopicYN="N">Bioethics and Professional Ethics</Keyword>
//   <Keyword MajorTopicYN="N">National Bioethics Advisory Commission</Keyword>
//   <Keyword MajorTopicYN="N">Popular Approach/Source</Keyword>
// </KeywordList>
#[derive(Serialize, Deserialize, Debug, Clone)]
struct KeywordList {
    #[serde(rename = "Keyword")]
    keyword: Vec<Keyword>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataBankName {
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccessionNumber {
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AccessionNumberList {
    #[serde(rename = "AccessionNumber")]
    accession_number: Vec<AccessionNumber>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataBank {
    #[serde(rename = "DataBankName")]
    data_bank_name: DataBankName,
    #[serde(rename = "AccessionNumberList")]
    accession_number_list: AccessionNumberList,
}

// <DataBankList CompleteYN="N">
// <DataBank>
//     <DataBankName>GENBANK</DataBankName>
//     <AccessionNumberList>
//         <AccessionNumber>Z93128</AccessionNumber>
//         <AccessionNumber>Z93157</AccessionNumber>
//     </AccessionNumberList>
// </DataBank>
// </DataBankList>

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataBankList {
    #[serde(rename = "@CompleteYN")]
    complete_flag: String,
    #[serde(rename = "DataBank")]
    data_bank: Vec<DataBank>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MedlineCitation {
    // #[serde(rename = "@Status")]
    // status: String,
    // #[serde(rename = "@IndexingMethod")]
    // indexing_method: Option<String>,
    // #[serde(rename = "@Owner")]
    // owner: String,
    #[serde(rename = "PMID")]
    pmid: PMID,
    // #[serde(rename = "Article")]
    // article: Article,
    // #[serde(rename = "MedlineJournalInfo")]
    // medline_journal_info: MedlineJournalInfo,
    // #[serde(rename = "MeshHeadingList")]
    // mesh_heading_list: Option<MeshHeadingList>,
    // #[serde(rename = "ChemicalList")]
    // chemical_list: Option<ChemicalList>,
    // #[serde(rename = "KeywordList")]
    // keyword_list: Option<Vec<KeywordList>>,
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
    article_ids: Vec<ArticleId>,
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
    #[serde(rename = "Reference")]
    references: Option<Vec<Reference>>,
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
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PubmedData {
    // #[serde(rename = "ReferenceList")]
    // reference_list: Option<Vec<ReferenceList>>,
    // #[serde(rename = "PublicationStatus")]
    // publication_status: PublicationStatus,
    #[serde(rename = "ArticleIdList")]
    article_id_list: ArticleIdList,
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
    // #[serde(rename = "MedlineCitation")]
    // medline_citation: MedlineCitation,
    #[serde(rename = "PubmedData")]
    pubmed_data: PubmedData,
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
struct PubmedGraphNode {
    id: u32,
    ids: Vec<u32>,
}

impl PubmedGraphNode {
    // fn from_pubmed_article(
    //     pubmed_article: &PubmedArticle,
    // ) -> Result<PubmedGraphNode, ParseIntError> {

    //     let id = pubmed_article.pubmed_data.pmid();
    //     let ids = pubmed_article.pubmed_data.pubmed_references();
    //     Ok(PubmedGraphNode { id, ids })
    // }
}

// reads from a start tag all the way to the corresponding end tag,
// returns the bytes of the whole tag
fn read_to_end_into_buffer<R: BufRead>(
    reader: &mut Reader<R>,
    start_tag: &BytesStart,
    buf: &mut Vec<u8>,
) -> Result<Vec<u8>, quick_xml::Error> {
    let mut depth = 0;
    let mut output_buf: Vec<u8> = Vec::new();
    let mut w = Writer::new(&mut output_buf);
    let tag_name = start_tag.name();
    w.write_event(Event::Start(start_tag.clone()))?;
    loop {
        buf.clear();
        let event = reader.read_event_into(buf)?;
        w.write_event(&event)?;

        match event {
            Event::Start(e) if e.name() == tag_name => depth += 1,
            Event::End(e) if e.name() == tag_name => {
                if depth == 0 {
                    return Ok(output_buf);
                }
                depth -= 1;
            }
            Event::Eof => {
                panic!("oh no")
            }
            _ => {}
        }
    }
}

fn read(file: &File) {
    let buf_reader = BufReader::new(file);
    let decoder = GzDecoder::new(buf_reader);
    let gz = BufReader::new(decoder);
    let mut reader = Reader::from_reader(gz);
    let mut buf: Vec<u8> = Vec::new();
    let mut article_buffer: Vec<u8> = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"PubmedArticle" => {
                    let article_bytes =
                        read_to_end_into_buffer(&mut reader, &e, &mut article_buffer).unwrap();
                    let str = std::str::from_utf8(&article_bytes).unwrap();
                    let mut deserializer = Deserializer::from_str(str);
                    match PubmedArticle::deserialize(&mut deserializer) {
                        Ok(pubmed_article) => {
                            //    let id = pubmed_article.medline_citation.pmid;
                            // println!("{:?}", &pubmed_article);
                            // println!("{:?}",pubmed_article.medline_citation.article.year());
                            // println!(
                            //     "{:?}",
                            //     PubmedGraphNode::from_pubmed_article(&pubmed_article)
                            // );
                            // println!("{}",serde_json::to_string_pretty(&pubmed_article).unwrap())
                        }
                        Err(e) => {
                            println!("{}", String::from_utf8(article_bytes).unwrap());
                            panic!("{:?}", e)
                        }
                    };
                }
                _ => (),
            },
            _ => (),
        }
        buf.clear();
    }
}

fn read_directory(dir: &Path) -> Result<(), std::io::Error> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;

            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "gz" {
                println!("{:?}", path);
                let file = File::open(path)?;
                read(&file);
            }
            // if path.is_dir() {
            //     visit_dirs(&path, cb)?;
            // } else {
            //     cb(&entry);
            // }
        }
    }
    Ok(())
}

fn main() {
    if let Some(user) = UserDirs::new() {
        let home_dir = user.home_dir();
        // let test_file = home_dir.join("workspace/test-data/pubmedsample.xml.gz");
        // let opened = File::open(test_file);

        // let opened = File::open("/Users/sdoronin/Downloads/baseline/pubmed22n0001.xml.gz");
        // match opened {
        //     Ok(file) => read(&file),
        //     Err(e) => {
        //         panic!("{:?}", e)
        //     }
        // }

        let _ = read_directory(Path::new("/Users/sdoronin/Downloads/baseline"));
    }
}
