mod parser;
mod agent;
use agent::agent::Agent;
use parser::parser::Parser;


fn main() {
    let parser = Parser::new(None);
    let agent = Agent::new(&parser);
    agent.run()
}
