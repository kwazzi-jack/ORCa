# ORCa Suite

## Overview
"**O**n-line **R**adio interferometric **Ca**libration" (ORCa) is a research prototype calibration suite designed for real-time or "online" calibration of radio interferometric data. Comprising the `ORCa-Engine` (Rust) and `ORCa-UI` (Python), ORCa is a future-centric project aiming to revolutionize real-time calibration systems in radio astronomy.

## Components

### ORCa-Engine (Work-In-Progress)
The `ORCa-Engine` is the computational backbone of the suite, optimized for performance and designed to handle the complexities of radio interferometric calibration. It aims to operate on local clusters, cloud servers, or standalone machines, offering scalable, real-time calibration solutions with an asynchronous data and task-handling layer.

### ORCa-UI (Work-In-Progress)
The `ORCa-UI` serves as the interactive interface to the `ORCa-Engine`, facilitating calibration tasks such as parameter adjustments, visualizations, and session management. Written in Python, it's designed to integrate seamlessly with the radio astronomy toolchain and potentially extend to Docker, Stimela, and Radio-Padre.

### Future Components
- **ORCa-Mock**: A simulation framework for testing the `ORCa` calibration cycle independently.
- **python-orca**: A Python library encapsulating various `ORCa` components for specialized testing and functionality.

## Documentation
Comprehensive documentation will be provided upon the completion of a functional version of `ORCa`.

## License
This project is licensed under the *GNU General Public License* (GPL), ensuring all adaptations and versions remain free and open-source. Under this license:
- **Usage Rights**: You are free to use, modify, and distribute this software.
- **Share-alike**: Any modifications or derivative works must also be made available under the same GPL license.
- **Freedom Guarantee**: Users are guaranteed the freedom to use, study, share, and modify the software.

For detailed terms and conditions, see the [LICENSE](LICENSE) file. Learn more about the GPL at the [GNU Licensing Page](https://www.gnu.org/licenses/gpl-3.0.en.html).
