use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::str::FromStr;

pub fn prompt_with_default<T>(prompt: &str, default: T) -> eyre::Result<T>
where
    T: Clone + ToString + FromStr,
    <T as FromStr>::Err: ToString,
{
    Input::<T>::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .show_default(true)
        .interact_text()
        .map_err(Into::into)
}

pub fn prompt<T>(prompt: &str) -> eyre::Result<T>
where
    T: Clone + ToString + FromStr,
    <T as FromStr>::Err: ToString,
{
    Input::<T>::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .map_err(Into::into)
}

pub fn confirm(prompt: &str) -> eyre::Result<bool> {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .show_default(true)
        .default(false)
        .interact()
        .map_err(Into::into)
}
