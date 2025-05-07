use bson::oid::ObjectId;
use futures::{AsyncReadExt, AsyncWriteExt};
use liquid::Template;
use serde::{Deserialize, Serialize};
use tokio::fs;
use std::{collections::HashMap, str::FromStr};

use crate::{utils::{error::Error, types::Result}, AppState};


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Templates {
    INDEX,
    PRODUCT,
}

pub struct Registry(HashMap<Templates, (ObjectId, Option<Template>)>);
impl Registry {
    pub async fn hardcoded() -> Result<Self> {
        Ok(Self(HashMap::from([
            (Templates::INDEX, (ObjectId::new(), None)),
            (Templates::PRODUCT, (ObjectId::from_str("681206f7e5c5c95dd5005ed5").unwrap(), None)),
        ])))
    }

    pub async fn get_template(&mut self, state: &AppState, template: Templates) -> Result<&Template> {
        if let Some((id, template)) = self.0.get_mut(&template) {
            if template.is_none() {
                let bucket = state.db.gridfs_bucket(None);
                let mut buf = Vec::new();
                let mut download_stream = bucket.open_download_stream(id.into())
                    .await.map_err(|_| Error::from("template not found".to_string()))?;
                let _ = download_stream.read_to_end(&mut buf)
                    .await.map_err(|_| Error::from("error reading template".to_string()))?;

                let template_string = String::from_utf8(buf)
                    .map_err(|_| Error::from("non valid template found".to_string()))?;
                tracing::info!("template: {}", template_string);
                let temp = liquid::ParserBuilder::with_stdlib()
                    .build().map_err(|e| Error::from(e.to_string()))?
                    .parse(&template_string).map_err(|e| Error::from(e.to_string()))?;

                *template = Some(temp);
            }

            if let Some(content) = template {
                return Ok(content);
            }
        }
        Err("template not found".to_string().into())
    }

    pub async fn upload() -> Result<ObjectId> {
        // create a registry and upload the theme to the bucket then return the registry id
        // this should be acid compliant
        todo!()
    }
}
