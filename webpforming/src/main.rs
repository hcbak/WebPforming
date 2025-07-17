use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use rayon::prelude::*;

use image::{codecs::webp::WebPEncoder, ImageEncoder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n - WebPforming - \n");

    let current_exe_path = env::current_exe()?;
    let current_dir = current_exe_path.parent()
        .ok_or("실행 파일의 상위 경로 지정 불가")?;

    println!("작업 경로: '{}'", current_dir.display());
    println!("이미지 변환 작업 시작\n");

    // 경로 내 파일 탐색
    let entries = fs::read_dir(current_dir)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    // 이미지 변환
    entries.into_par_iter().for_each(|entry| {
        let path = entry.path();
        if path.is_file() && is_image_extension(&path) {
            if let Err(e) = convert_to_webp(&path) {
                eprintln!("[-] '{}' 변환 실패 - {}", get_filename_str(&path), e);
            }
        }
    });

    println!("\n작업 정상 종료");
    Ok(())
}

/// 이미지 확장자 일치 확인
fn is_image_extension(path: &Path) -> bool {
    // 이미지 확장자 종류
    const IMAGE_EXTENSIONS: &[&str] = &["bmp", "gif", "jpeg", "jpg", "png"];

    path.extension()
        .and_then(OsStr::to_str)
        .map(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Path에서 파일 이름 분리 후 반환
fn get_filename_str(path: &Path) -> &str {
    path.file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("")
}

/// 이미지 파일을 WebP로 변환 후 저장
fn convert_to_webp(image_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(image_path)?;

    let mut webp_path = PathBuf::from(image_path);
    webp_path.set_extension("webp");

    let mut file_out = fs::File::create(&webp_path)?;
    
    // 무손실 WebP 변환
    let encoder = WebPEncoder::new_lossless(&mut file_out);
    encoder.write_image(img.as_bytes(), img.width(), img.height(), img.color().into())?;

    println!("[+] 변환 완료 - '{}' → '{}'", get_filename_str(image_path), get_filename_str(&webp_path));
    Ok(())
}
