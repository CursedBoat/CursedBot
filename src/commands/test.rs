use std::str::FromStr;

use crate::common::{color_conversion::Color, concatenate_integers, framework::{Context, Error}};

/// Pong
#[poise::command(slash_command, prefix_command)]
pub async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error>{
    ctx.reply("pong!").await?;
    Ok(())
}

/// Converts hexadecimal colors to decimal colors.
#[poise::command(slash_command, prefix_command)]
pub async fn convert_color(
    ctx: Context<'_>,
    #[description = "Hex color. (Without #)"] hex: String,
) -> Result<(), Error>{
    let color = Color::from_str(&hex).unwrap();

    let concatenated_int = concatenate_integers(&[ 
        color.color_vec[0], // r
        color.color_vec[1], // g
        color.color_vec[2], // b
    ]);

    ctx.reply(format!("Color: {}", concatenated_int)).await?;
    Ok(())
}