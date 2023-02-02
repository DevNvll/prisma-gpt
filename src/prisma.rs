pub struct PrismaSchema {
    string: String,
}

impl PrismaSchema {
    pub fn load_from_file() -> Result<PrismaSchema, Box<dyn std::error::Error>> {
        let path_to_prisma_schema = get_prisma_schema_path();
        let schema = std::fs::read_to_string(path_to_prisma_schema)?;
        Ok(PrismaSchema { string: schema })
    }

    pub fn get_cleaned_schema(&self) -> String {
        let mut model_definitions = String::new();
        let lines = &self.string.lines();
        let mut in_model_block = false;

        for line in lines.clone().into_iter() {
            if line.trim().starts_with("model") || line.trim().starts_with("enum") {
                in_model_block = true;
            }
            if in_model_block {
                model_definitions.push_str(line);
                model_definitions.push_str("\n");
            }
            if line.trim().starts_with("}") {
                in_model_block = false;
            }
        }

        model_definitions
    }
}

fn get_prisma_schema_path() -> String {
    let mut path = std::env::current_dir().unwrap();
    path.push("prisma");
    path.push("schema.prisma");
    path.to_str().unwrap().to_string()
}
