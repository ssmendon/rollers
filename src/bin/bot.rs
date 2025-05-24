use poise::serenity_prelude as serenity;
use rollers::dice::ast::eval;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn roll(
    ctx: Context<'_>,
    #[rest]
    #[max_length = 2048]
    #[description = "Dice notation"]
    expression: String,
) -> Result<(), Error> {
    use pest::Parser as _;
    use rollers::dice::parser::Rule;
    use rollers::dice::parser::parse_expr;

    let reply: Result<String, anyhow::Error> = {
        let mut pairs = rollers::dice::parser::DiceParser::parse(Rule::equation, &expression)
            .map_err(|_| anyhow::anyhow!("bad dice roll"))?;
        let ast = parse_expr(pairs.next().unwrap().into_inner());
        let result = eval(&ast);
        Ok(format!("{:#?}\n\nTotal: {}", ast, result))
    };
    let reply = reply?;

    ctx.say(reply).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), roll()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
