# 🌏WebPforming🌏

## About

바야흐로 대용량 이미지의 시대. 이제 과거의 이미지 포맷은 서버의 저장 공간과 클라이언트의 웹 페이지 로딩 속도에 부담을 주어 그 힘을 잃었다.

2010년에 구글에서 만든 [WebP 포맷](https://developers.google.com/speed/webp?hl=ko)은 대부분의 현대 웹 브라우저에서 원활하게 지원하고 있고, 이를 활용하면 이 문제를 일부 해결할 수 있다.

이 프로젝트는 실행 파일이 존재하는 경로의 가능한 모든 이미지 파일을 WebP로 변환하는 것을 목표로 한다. 대상 폴더에 실행 파일을 떨어뜨리고 실행하는 것으로 **WebPforming**을 실현하는 것이다.

## Support

- Windows
  - ✅11
- Linux
  - Ubuntu LTS
    - ✅24.04 

## How to use

### Build

1. Rust 개발 환경 준비 - [Install Rust](https://www.rust-lang.org/tools/install)
2. 빌드 후 실행 파일 확인

```
cargo build --release
```

- Windows: webpforming\target\release\WebPforming.exe
- Linux: webpforming\target\release\WebPforming

### Execution

#### Windows

1. 과거의 이미지 포맷으로 오염된 경로에 WebPforming.exe 파일을 놓는다.
2. 실행시켜 정화한다.

#### Ubuntu

1. 과거의 이미지 포맷으로 오염된 경로에 WebPforming 파일을 놓는다.
2. 실행시켜 정화한다.

## ⚠️Caution⚠️

- 이미지 파일의 **메타데이터는 전부 제거**합니다.
- 변환을 정상적으로 진행한 **원본 파일은 제거**합니다.
  - 원본이 중요한 파일은 백업 후 변환해야 합니다.
- 원본이 손실 압축(JPEG 계열)인 경우 손실 압축으로 변환합니다.
  - 손실 품질은 기본 70으로 설정하였으나 지나친 품질 저하가 우려된다면 소스를 받아 80 ~ 90 등으로 조정하여 재빌드하여야 합니다.
- **여러 프레임이 존재하는(움직이는) 이미지는 정상적인 변환을 지원하지 않아, 첫 프레임만 남을 수 있습니다.**
