use std::collections::HashMap;

pub struct DescriptionLocalizations {
    pub fallback: String,
    pub localizations: HashMap<String, String>,
}

impl DescriptionLocalizations {
    pub fn new<I, K, V>(fallback: impl ToString, localizations: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: ToString,
        V: ToString,
    {
        Self {
            fallback: fallback.to_string(),
            localizations: localizations
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

pub struct NameLocalizations(pub HashMap<String, String>);

impl NameLocalizations {
    pub fn new<I, K, V>(localizations: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: ToString,
        V: ToString,
    {
        Self(
            localizations
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        )
    }
}
