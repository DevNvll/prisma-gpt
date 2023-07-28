use gpt::GPTService;
use prisma::PrismaSchema;

mod gpt;
mod prisma;

#[tokio::main]
async fn main() {
    let question = std::env::args().nth(1).expect("Question must be provided");

    let schema = PrismaSchema::load_from_file()
        .expect("Error loading schema file")
        .get_cleaned_schema();

    let prompt = format!("{}
    As a senior analyst, given the above schemas and data, write a detailed and correct mySQL sql query to answer the analytical question:    {}
    ", schema, &question);

    print!("{}", &prompt);

    let openai_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let gpt = GPTService::new(&openai_key);

    let completion = gpt
        .get_gpt_completion(&prompt)
        .await
        .expect("Error getting completion");

    println!("{}", completion);
}
