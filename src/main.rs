
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let s = detector::DajareSearcher::new("/Users/sizumita/Workspace/dajare/lindera-ipadic");
    detector::rulebase::search_dajare(s, "私の名前はすみどらです。").await;
}
