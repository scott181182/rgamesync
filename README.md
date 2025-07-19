# Rgamesync

Rgamesync is an in-development program for syncing game save data between devices using cloud storage and [`rclone`](https://rclone.org/).

## Roadmap

- Solidify configuration structure
- Add interactive configuration creation (`rgamesync config`)
- Actually sync data using Rclone CLI (`rgamesync sync`)
- Add CI to run tests
- Add CD to make releases for different platforms (i.e. Windows, MacOS, and Linux)
- Eventually switch to using `librclone`, bundled with the application

## Inspirations

- [OpenCloudSaves](https://github.com/DavidDeSimone/OpenCloudSaves)
