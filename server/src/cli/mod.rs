use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "crypto_portfolio")]
pub enum Opt {
    #[structopt(name = "serve")]
    Serve,
    #[structopt(name = "graphql-schema")]
    GraphQlSchema,
}
