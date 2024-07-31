use crate::common::{framework::{Context, Error}, read_json::dhar_mann};
use rand::Rng;

/// Generate a Dhar Mann video title
#[poise::command(slash_command, prefix_command)]
pub async fn generate_dharmann_title(
    ctx: Context<'_>,
) -> Result<(), Error>{
    let dhar_mann_json = dhar_mann();

    // calculate len for rng
    let person_len = dhar_mann_json.person.len()-1;
    let action_len = dhar_mann_json.action.len()-1;
    let closer_len = dhar_mann_json.closer.len()-1;

    // grab random value from vector
    let person_rng = rand::thread_rng().gen_range(0..person_len);
    let person_rng_alt = rand::thread_rng().gen_range(0..person_len);
    let action_rng = rand::thread_rng().gen_range(0..action_len);
    let closer_rng = rand::thread_rng().gen_range(0..closer_len);

    // add strings together to form a meaningful title
    let generated_title = format!("{} {} {}, {}", 
        dhar_mann_json.person[person_rng], 
        dhar_mann_json.action[action_rng], 
        dhar_mann_json.person[person_rng_alt], 
        dhar_mann_json.closer[closer_rng]
    );

    ctx.reply(generated_title).await?;
    Ok(())
}