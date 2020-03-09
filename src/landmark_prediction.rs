//! Structs for predicting face landmark locations from images and face rectangles.

use super::*;
use std::path::Path;

cpp_class!(unsafe struct LandmarkPredictorInner as "shape_predictor");

/// A face landmark predictor.
#[derive(Clone)]
pub struct LandmarkPredictor {
    inner: LandmarkPredictorInner
}

impl LandmarkPredictor {
    /// Deserialize the landmark predictor from a file path.
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, String> {
        let string = path_as_cstring(filename.as_ref())?;
        
        let inner = LandmarkPredictorInner::default();

        let deserialized = unsafe {
            let filename = string.as_ptr();
            let predictor = &inner;

            cpp!([filename as "char*", predictor as "shape_predictor*"] -> bool as "bool" {
                try {
                    deserialize(filename) >> *predictor;
                    return true;
                } catch (const error& exception) {
                    return false;
                }
            })
        };

        if !deserialized {
            Err(format!("Failed to deserialize '{}'", filename.as_ref().display()))
        } else {
            Ok(Self {inner})
        }
    }

    /// Detect face landmarks.
    /// 
    /// This will generally always return the number of landmarks as defined by the model.
    pub fn face_landmarks(&self, image: &ImageMatrix, rect: &Rectangle) -> FaceLandmarks {
        let predictor = &self.inner;

        unsafe {
            cpp!([predictor as "shape_predictor*", image as "matrix<rgb_pixel>*", rect as "rectangle*"] -> FaceLandmarks as "full_object_detection" {
                return (*predictor)(*image, *rect);
            })
        }
    }
}

#[cfg(feature = "download-models")]
impl Default for LandmarkPredictor {
    fn default() -> Self {
        Self::new(path_for_file("shape_predictor_68_face_landmarks.dat")).unwrap()
    }
}