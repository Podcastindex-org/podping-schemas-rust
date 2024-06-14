#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Iri"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Iri(pub String);
impl std::ops::Deref for Iri {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Iri> for String {
    fn from(value: Iri) -> Self {
        value.0
    }
}
impl From<&Iri> for Iri {
    fn from(value: &Iri) -> Self {
        value.clone()
    }
}
impl From<String> for Iri {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for Iri {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for Iri {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "IriList"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"string\","]
#[doc = "    \"format\": \"uri\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IriList(pub Vec<String>);
impl std::ops::Deref for IriList {
    type Target = Vec<String>;
    fn deref(&self) -> &Vec<String> {
        &self.0
    }
}
impl From<IriList> for Vec<String> {
    fn from(value: IriList) -> Self {
        value.0
    }
}
impl From<&IriList> for IriList {
    fn from(value: &IriList) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for IriList {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}
#[doc = "MediumsV10"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"podcast\","]
#[doc = "    \"music\","]
#[doc = "    \"video\","]
#[doc = "    \"film\","]
#[doc = "    \"audiobook\","]
#[doc = "    \"newsletter\","]
#[doc = "    \"blog\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum MediumsV10 {
    #[serde(rename = "podcast")]
    Podcast,
    #[serde(rename = "music")]
    Music,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "film")]
    Film,
    #[serde(rename = "audiobook")]
    Audiobook,
    #[serde(rename = "newsletter")]
    Newsletter,
    #[serde(rename = "blog")]
    Blog,
}
impl From<&MediumsV10> for MediumsV10 {
    fn from(value: &MediumsV10) -> Self {
        value.clone()
    }
}
impl ToString for MediumsV10 {
    fn to_string(&self) -> String {
        match *self {
            Self::Podcast => "podcast".to_string(),
            Self::Music => "music".to_string(),
            Self::Video => "video".to_string(),
            Self::Film => "film".to_string(),
            Self::Audiobook => "audiobook".to_string(),
            Self::Newsletter => "newsletter".to_string(),
            Self::Blog => "blog".to_string(),
        }
    }
}
impl std::str::FromStr for MediumsV10 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "podcast" => Ok(Self::Podcast),
            "music" => Ok(Self::Music),
            "video" => Ok(Self::Video),
            "film" => Ok(Self::Film),
            "audiobook" => Ok(Self::Audiobook),
            "newsletter" => Ok(Self::Newsletter),
            "blog" => Ok(Self::Blog),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for MediumsV10 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MediumsV10 {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MediumsV10 {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "MediumsV11"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"mixed\","]
#[doc = "    \"podcastL\","]
#[doc = "    \"musicL\","]
#[doc = "    \"videoL\","]
#[doc = "    \"filmL\","]
#[doc = "    \"audiobookL\","]
#[doc = "    \"newsletterL\","]
#[doc = "    \"blogL\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum MediumsV11 {
    #[serde(rename = "mixed")]
    Mixed,
    #[serde(rename = "podcastL")]
    PodcastL,
    #[serde(rename = "musicL")]
    MusicL,
    #[serde(rename = "videoL")]
    VideoL,
    #[serde(rename = "filmL")]
    FilmL,
    #[serde(rename = "audiobookL")]
    AudiobookL,
    #[serde(rename = "newsletterL")]
    NewsletterL,
    #[serde(rename = "blogL")]
    BlogL,
}
impl From<&MediumsV11> for MediumsV11 {
    fn from(value: &MediumsV11) -> Self {
        value.clone()
    }
}
impl ToString for MediumsV11 {
    fn to_string(&self) -> String {
        match *self {
            Self::Mixed => "mixed".to_string(),
            Self::PodcastL => "podcastL".to_string(),
            Self::MusicL => "musicL".to_string(),
            Self::VideoL => "videoL".to_string(),
            Self::FilmL => "filmL".to_string(),
            Self::AudiobookL => "audiobookL".to_string(),
            Self::NewsletterL => "newsletterL".to_string(),
            Self::BlogL => "blogL".to_string(),
        }
    }
}
impl std::str::FromStr for MediumsV11 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "mixed" => Ok(Self::Mixed),
            "podcastL" => Ok(Self::PodcastL),
            "musicL" => Ok(Self::MusicL),
            "videoL" => Ok(Self::VideoL),
            "filmL" => Ok(Self::FilmL),
            "audiobookL" => Ok(Self::AudiobookL),
            "newsletterL" => Ok(Self::NewsletterL),
            "blogL" => Ok(Self::BlogL),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for MediumsV11 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MediumsV11 {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MediumsV11 {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Podping"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Podping\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/podping-v0\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/podping-v0.2\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/podping-v0.3\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/podping-v1.0\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/podping-v1.1\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Podping {
    V0(PodpingV0),
    V02(PodpingV02),
    V03(PodpingV03),
    V10(PodpingV10),
    V11(PodpingV11),
}
impl From<&Podping> for Podping {
    fn from(value: &Podping) -> Self {
        value.clone()
    }
}
impl From<PodpingV0> for Podping {
    fn from(value: PodpingV0) -> Self {
        Self::V0(value)
    }
}
impl From<PodpingV02> for Podping {
    fn from(value: PodpingV02) -> Self {
        Self::V02(value)
    }
}
impl From<PodpingV03> for Podping {
    fn from(value: PodpingV03) -> Self {
        Self::V03(value)
    }
}
impl From<PodpingV10> for Podping {
    fn from(value: PodpingV10) -> Self {
        Self::V10(value)
    }
}
impl From<PodpingV11> for Podping {
    fn from(value: PodpingV11) -> Self {
        Self::V11(value)
    }
}
#[doc = "PodpingV0"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"required\": ["]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"url\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PodpingV0 {
    pub url: String,
}
impl From<&PodpingV0> for PodpingV0 {
    fn from(value: &PodpingV0) -> Self {
        value.clone()
    }
}
impl PodpingV0 {
    pub fn builder() -> builder::PodpingV0 {
        Default::default()
    }
}
#[doc = "PodpingV02"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"num_urls\","]
#[doc = "        \"url\","]
#[doc = "        \"version\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"num_urls\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"const\": 1"]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"$ref\": \"#/definitions/iri\""]
#[doc = "        },"]
#[doc = "        \"version\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"0.2\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"num_urls\","]
#[doc = "        \"urls\","]
#[doc = "        \"version\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"num_urls\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"minimum\": 1.0"]
#[doc = "        },"]
#[doc = "        \"urls\": {"]
#[doc = "          \"$ref\": \"#/definitions/iri-list\""]
#[doc = "        },"]
#[doc = "        \"version\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"0.2\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum PodpingV02 {
    Variant0 {
        num_urls: i64,
        url: Iri,
        version: String,
    },
    Variant1 {
        num_urls: std::num::NonZeroU64,
        urls: IriList,
        version: String,
    },
}
impl From<&PodpingV02> for PodpingV02 {
    fn from(value: &PodpingV02) -> Self {
        value.clone()
    }
}
#[doc = "PodpingV03"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"num_urls\","]
#[doc = "        \"reason\","]
#[doc = "        \"url\","]
#[doc = "        \"version\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"num_urls\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"const\": 1"]
#[doc = "        },"]
#[doc = "        \"reason\": {"]
#[doc = "          \"$ref\": \"#/definitions/reasons-v0.3\""]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"$ref\": \"#/definitions/iri\""]
#[doc = "        },"]
#[doc = "        \"version\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"0.3\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"num_urls\","]
#[doc = "        \"reason\","]
#[doc = "        \"urls\","]
#[doc = "        \"version\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"num_urls\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"minimum\": 1.0"]
#[doc = "        },"]
#[doc = "        \"reason\": {"]
#[doc = "          \"$ref\": \"#/definitions/reasons-v0.3\""]
#[doc = "        },"]
#[doc = "        \"urls\": {"]
#[doc = "          \"$ref\": \"#/definitions/iri-list\""]
#[doc = "        },"]
#[doc = "        \"version\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"0.3\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum PodpingV03 {
    Variant0 {
        num_urls: i64,
        reason: ReasonsV03,
        url: Iri,
        version: String,
    },
    Variant1 {
        num_urls: std::num::NonZeroU64,
        reason: ReasonsV03,
        urls: IriList,
        version: String,
    },
}
impl From<&PodpingV03> for PodpingV03 {
    fn from(value: &PodpingV03) -> Self {
        value.clone()
    }
}
#[doc = "PodpingV10"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"required\": ["]
#[doc = "    \"iris\","]
#[doc = "    \"medium\","]
#[doc = "    \"reason\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"iris\": {"]
#[doc = "      \"$ref\": \"#/definitions/iri-list\""]
#[doc = "    },"]
#[doc = "    \"medium\": {"]
#[doc = "      \"$ref\": \"#/definitions/mediums-v1.0\""]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"$ref\": \"#/definitions/reasons-v1.0\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"1.0\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PodpingV10 {
    pub iris: IriList,
    pub medium: MediumsV10,
    pub reason: ReasonsV10,
    pub version: String,
}
impl From<&PodpingV10> for PodpingV10 {
    fn from(value: &PodpingV10) -> Self {
        value.clone()
    }
}
impl PodpingV10 {
    pub fn builder() -> builder::PodpingV10 {
        Default::default()
    }
}
#[doc = "PodpingV11"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"required\": ["]
#[doc = "    \"iris\","]
#[doc = "    \"medium\","]
#[doc = "    \"reason\","]
#[doc = "    \"sessionId\","]
#[doc = "    \"timestampNs\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"iris\": {"]
#[doc = "      \"$ref\": \"#/definitions/iri-list\""]
#[doc = "    },"]
#[doc = "    \"medium\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/mediums-v1.0\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/mediums-v1.1\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"$ref\": \"#/definitions/reasons-v1.0\""]
#[doc = "    },"]
#[doc = "    \"sessionId\": {"]
#[doc = "      \"$ref\": \"#/definitions/uint64\""]
#[doc = "    },"]
#[doc = "    \"timestampNs\": {"]
#[doc = "      \"$ref\": \"#/definitions/uint64\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"1.1\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PodpingV11 {
    pub iris: IriList,
    pub medium: PodpingV11Medium,
    pub reason: ReasonsV10,
    #[serde(rename = "sessionId")]
    pub session_id: Uint64,
    #[serde(rename = "timestampNs")]
    pub timestamp_ns: Uint64,
    pub version: String,
}
impl From<&PodpingV11> for PodpingV11 {
    fn from(value: &PodpingV11) -> Self {
        value.clone()
    }
}
impl PodpingV11 {
    pub fn builder() -> builder::PodpingV11 {
        Default::default()
    }
}
#[doc = "PodpingV11Medium"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/mediums-v1.0\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/mediums-v1.1\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PodpingV11Medium {
    V10(MediumsV10),
    V11(MediumsV11),
}
impl From<&PodpingV11Medium> for PodpingV11Medium {
    fn from(value: &PodpingV11Medium) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PodpingV11Medium {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::V10(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::V11(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for PodpingV11Medium {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PodpingV11Medium {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PodpingV11Medium {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ToString for PodpingV11Medium {
    fn to_string(&self) -> String {
        match self {
            Self::V10(x) => x.to_string(),
            Self::V11(x) => x.to_string(),
        }
    }
}
impl From<MediumsV10> for PodpingV11Medium {
    fn from(value: MediumsV10) -> Self {
        Self::V10(value)
    }
}
impl From<MediumsV11> for PodpingV11Medium {
    fn from(value: MediumsV11) -> Self {
        Self::V11(value)
    }
}
#[doc = "ReasonsV03"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"const\": \"feed_update\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ReasonsV03(pub String);
impl std::ops::Deref for ReasonsV03 {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ReasonsV03> for String {
    fn from(value: ReasonsV03) -> Self {
        value.0
    }
}
impl From<&ReasonsV03> for ReasonsV03 {
    fn from(value: &ReasonsV03) -> Self {
        value.clone()
    }
}
impl From<String> for ReasonsV03 {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for ReasonsV03 {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for ReasonsV03 {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "ReasonsV10"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"update\","]
#[doc = "    \"live\","]
#[doc = "    \"liveEnd\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ReasonsV10 {
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "liveEnd")]
    LiveEnd,
}
impl From<&ReasonsV10> for ReasonsV10 {
    fn from(value: &ReasonsV10) -> Self {
        value.clone()
    }
}
impl ToString for ReasonsV10 {
    fn to_string(&self) -> String {
        match *self {
            Self::Update => "update".to_string(),
            Self::Live => "live".to_string(),
            Self::LiveEnd => "liveEnd".to_string(),
        }
    }
}
impl std::str::FromStr for ReasonsV10 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "update" => Ok(Self::Update),
            "live" => Ok(Self::Live),
            "liveEnd" => Ok(Self::LiveEnd),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ReasonsV10 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ReasonsV10 {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ReasonsV10 {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Uint64"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"maximum\": 1.8446744073709552e19,"]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Uint64(pub u64);
impl std::ops::Deref for Uint64 {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}
impl From<Uint64> for u64 {
    fn from(value: Uint64) -> Self {
        value.0
    }
}
impl From<&Uint64> for Uint64 {
    fn from(value: &Uint64) -> Self {
        value.clone()
    }
}
impl From<u64> for Uint64 {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for Uint64 {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for Uint64 {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Uint64 {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Uint64 {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for Uint64 {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct PodpingV0 {
        url: Result<String, String>,
    }
    impl Default for PodpingV0 {
        fn default() -> Self {
            Self {
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl PodpingV0 {
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<PodpingV0> for super::PodpingV0 {
        type Error = super::error::ConversionError;
        fn try_from(value: PodpingV0) -> Result<Self, super::error::ConversionError> {
            Ok(Self { url: value.url? })
        }
    }
    impl From<super::PodpingV0> for PodpingV0 {
        fn from(value: super::PodpingV0) -> Self {
            Self { url: Ok(value.url) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PodpingV10 {
        iris: Result<super::IriList, String>,
        medium: Result<super::MediumsV10, String>,
        reason: Result<super::ReasonsV10, String>,
        version: Result<String, String>,
    }
    impl Default for PodpingV10 {
        fn default() -> Self {
            Self {
                iris: Err("no value supplied for iris".to_string()),
                medium: Err("no value supplied for medium".to_string()),
                reason: Err("no value supplied for reason".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl PodpingV10 {
        pub fn iris<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IriList>,
            T::Error: std::fmt::Display,
        {
            self.iris = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for iris: {}", e));
            self
        }
        pub fn medium<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::MediumsV10>,
            T::Error: std::fmt::Display,
        {
            self.medium = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for medium: {}", e));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ReasonsV10>,
            T::Error: std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<PodpingV10> for super::PodpingV10 {
        type Error = super::error::ConversionError;
        fn try_from(value: PodpingV10) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                iris: value.iris?,
                medium: value.medium?,
                reason: value.reason?,
                version: value.version?,
            })
        }
    }
    impl From<super::PodpingV10> for PodpingV10 {
        fn from(value: super::PodpingV10) -> Self {
            Self {
                iris: Ok(value.iris),
                medium: Ok(value.medium),
                reason: Ok(value.reason),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PodpingV11 {
        iris: Result<super::IriList, String>,
        medium: Result<super::PodpingV11Medium, String>,
        reason: Result<super::ReasonsV10, String>,
        session_id: Result<super::Uint64, String>,
        timestamp_ns: Result<super::Uint64, String>,
        version: Result<String, String>,
    }
    impl Default for PodpingV11 {
        fn default() -> Self {
            Self {
                iris: Err("no value supplied for iris".to_string()),
                medium: Err("no value supplied for medium".to_string()),
                reason: Err("no value supplied for reason".to_string()),
                session_id: Err("no value supplied for session_id".to_string()),
                timestamp_ns: Err("no value supplied for timestamp_ns".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl PodpingV11 {
        pub fn iris<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IriList>,
            T::Error: std::fmt::Display,
        {
            self.iris = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for iris: {}", e));
            self
        }
        pub fn medium<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::PodpingV11Medium>,
            T::Error: std::fmt::Display,
        {
            self.medium = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for medium: {}", e));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ReasonsV10>,
            T::Error: std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {}", e));
            self
        }
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Uint64>,
            T::Error: std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session_id: {}", e));
            self
        }
        pub fn timestamp_ns<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Uint64>,
            T::Error: std::fmt::Display,
        {
            self.timestamp_ns = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp_ns: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<PodpingV11> for super::PodpingV11 {
        type Error = super::error::ConversionError;
        fn try_from(value: PodpingV11) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                iris: value.iris?,
                medium: value.medium?,
                reason: value.reason?,
                session_id: value.session_id?,
                timestamp_ns: value.timestamp_ns?,
                version: value.version?,
            })
        }
    }
    impl From<super::PodpingV11> for PodpingV11 {
        fn from(value: super::PodpingV11) -> Self {
            Self {
                iris: Ok(value.iris),
                medium: Ok(value.medium),
                reason: Ok(value.reason),
                session_id: Ok(value.session_id),
                timestamp_ns: Ok(value.timestamp_ns),
                version: Ok(value.version),
            }
        }
    }
}
