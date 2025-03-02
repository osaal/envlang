---
name: Bug report
about: Title your report
title: ''
labels: ''
assignees: osaal

---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Actual behaviour**
A clear and concise description of what Envlang produced. Screenshots and output text files are especially appreciated!

Pro tip: Since Envlang writes to `stdout`, you can redirect the output and errors into a text file and upload the text file as is: `envlang file.envl &> bug_report.txt`. This works on Unix-like systems (Linux, MacOSX) - Windows users can figure out a similar implementation, as I don't know it.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**System (please complete the following information):**
 - OS: [e.g. Windows]
 - OS version: [e.g., Windows 10 version 22H2]
- Rustc version: [copy the output from `rustc --version`, e.g., `rustc 1.85.0 (4d91de4e4 2025-02-17)`]
- Cargo version: [copy the output from `cargo --version`, e.g., `cargo 1.85.0 (d73d2caf9 2024-12-31)`]

**Additional context**
Add any other context about the problem here.
