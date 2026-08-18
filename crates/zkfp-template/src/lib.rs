//! # zkfp-template
//! Fingerprint template serialization and types based on nbis-rs.

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Invalid template data: {0}")]
    InvalidData(String),

    #[error("NBIS Error: {0}")]
    Nbis(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FingerPosition {
    Unknown = 0,
    RightThumb = 1,
    RightIndex = 2,
    RightMiddle = 3,
    RightRing = 4,
    RightLittle = 5,
    LeftThumb = 6,
    LeftIndex = 7,
    LeftMiddle = 8,
    LeftRing = 9,
    LeftLittle = 10,
}

impl FingerPosition {
    pub fn from_i32(value: i32) -> Option<Self> {
        Some(match value {
            0 => FingerPosition::Unknown,
            1 => FingerPosition::RightThumb,
            2 => FingerPosition::RightIndex,
            3 => FingerPosition::RightMiddle,
            4 => FingerPosition::RightRing,
            5 => FingerPosition::RightLittle,
            6 => FingerPosition::LeftThumb,
            7 => FingerPosition::LeftIndex,
            8 => FingerPosition::LeftMiddle,
            9 => FingerPosition::LeftRing,
            10 => FingerPosition::LeftLittle,
            _ => return None,
        })
    }
}

/// FingerprintTemplate represents a fingerprint's ISO template.
#[derive(Clone, Debug)]
pub struct FingerprintTemplate {
    pub iso_bytes: Vec<u8>,
    pub quality: u32,
}

impl FingerprintTemplate {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.iso_bytes.clone()
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, TemplateError> {
        Ok(Self {
            iso_bytes: data.to_vec(),
            quality: 50, // Default quality when loading from DB
        })
    }

    pub fn to_compact(&self, _limit: usize) -> CompactTemplate {
        CompactTemplate {
            iso_bytes: self.iso_bytes.clone(),
        }
    }

    pub fn merge(templates: &[&FingerprintTemplate]) -> Result<FingerprintTemplate, TemplateError> {
        // NBIS does not easily merge minutiae templates.
        // As a fallback, we just return the highest quality template,
        // or the first one if quality is equal.
        let mut best = templates.first()
            .ok_or_else(|| TemplateError::InvalidData("No templates to merge".into()))?;
        
        for t in templates {
            if t.quality > best.quality {
                best = t;
            }
        }
        
        Ok((*best).clone())
    }
}

#[derive(Clone, Debug)]
pub struct CompactTemplate {
    pub iso_bytes: Vec<u8>,
}

impl CompactTemplate {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.iso_bytes.clone()
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, TemplateError> {
        Ok(Self {
            iso_bytes: data.to_vec(),
        })
    }
}

// Stubs for native format compatibility if needed
#[derive(Clone, Debug)]
pub struct NativeTemplateContainer {}
impl NativeTemplateContainer {
    pub fn parse(_data: &[u8]) -> Result<Self, TemplateError> {
        Err(TemplateError::InvalidData("Native parsing unsupported".into()))
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, TemplateError> {
        Err(TemplateError::InvalidData("Native format unsupported".into()))
    }
}

#[derive(Clone, Debug)]
pub enum TemplateMemoryBlock {
    Logical(FingerprintTemplate),
    Native(NativeTemplateContainer),
}
impl TemplateMemoryBlock {
    pub fn parse(data: &[u8]) -> Result<Self, TemplateError> {
        Ok(TemplateMemoryBlock::Logical(FingerprintTemplate::from_bytes(data)?))
    }
}
