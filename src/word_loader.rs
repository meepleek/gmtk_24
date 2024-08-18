use crate::prelude::*;
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    reflect::TypePath,
};

pub(super) fn plugin(app: &mut App) {
    app.init_asset_loader::<WordListLoader>()
        .init_asset::<WordListSource>();
}

#[derive(Asset, TypePath, Debug)]
pub(crate) struct WordListSource(pub Vec<String>);

#[derive(Default)]
struct WordListLoader;

impl AssetLoader for WordListLoader {
    type Asset = WordListSource;
    type Settings = ();
    type Error = std::io::Error;
    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;
        // todo: properly handle UTF8 errors
        let text = String::from_utf8_lossy(&buf);
        let words = text
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|w| !w.is_empty())
            .collect();
        Ok(WordListSource(words))
    }

    fn extensions(&self) -> &[&str] {
        &["words.txt"]
    }
}
