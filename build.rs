extern crate cpp_build;
#[cfg(feature = "download-models")]
extern crate reqwest;
#[cfg(feature = "download-models")]
extern crate bzip2;
#[cfg(feature = "download-models")]
extern crate tokio;

#[cfg(feature = "download-models")]
fn download_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("files")
}

#[cfg(feature = "download-models")]
async fn download_and_unzip(url: &str) -> Result<(), reqwest::Error> {
    use bzip2::read::*;

    let url: reqwest::Url = url.parse().unwrap();

    let filename = url
        .path_segments().unwrap()
        .last().unwrap()
        .replace(".bz2", "");

    let path = download_path().join(&filename);

    if path.exists() {
        println!("Already got '{}'", path.display());
        return Ok(());
    }

    println!("Downloading '{}'...", url);

    let response = reqwest::get(url).await?.bytes().await?;
    let mut decoded = BzDecoder::new(response.as_ref());
    let mut file = std::fs::File::create(&path).unwrap();
    std::io::copy(&mut decoded, &mut file).unwrap();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("cargo:rustc-link-lib=dlib");
    
    // I _believe_ osx requires lapack and blas
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=lapack");
        println!("cargo:rustc-link-lib=blas");
    }

    cpp_build::build("src/lib.rs");

    #[cfg(feature = "download-models")]
    {
        if !download_path().exists() {
            std::fs::create_dir(download_path()).unwrap();
        }

        // Download the data files
        // I'm not sure if doing this in the build script is such a good idea, seeing as it happens silently,
        // but I dont think adding the files to the repo is good either

        download_and_unzip("http://dlib.net/files/mmod_human_face_detector.dat.bz2").await;
        download_and_unzip("http://dlib.net/files/shape_predictor_68_face_landmarks.dat.bz2").await;
        download_and_unzip("http://dlib.net/files/dlib_face_recognition_resnet_model_v1.dat.bz2").await;
    }

    Ok(())
}