# 📚 Module & Naming Conventions

This document outlines the naming and organizational conventions used in this Rust project to maintain clarity, consistency, and scalability across the codebase.

---

## 📦 Module Prefixes

We use **module name prefixes** to quickly identify the purpose of a module at a glance. This is especially helpful in flat module structures or large projects with many interrelated files.

**All prefixes are kept under 4 characters** to keep the namespace clean and easily scannable.

| Prefix     | Meaning                         | Example               | Description                                                                 |
|------------|----------------------------------|-----------------------|------------------------------------------------------------------------------|
| `m_`       | Model / Class-like Struct        | `m_user`, `m_color`   | Structs representing core domain entities. These can be single files or directories with related implementations (`ext_`). |
| `tr_`      | Trait                            | `tr_displayable`      | Traits that define shared behavior or roles in the system.                 |
| `ext_`     | Extensions / Implementations     | `ext_add`, `ext_i8`   | Modules that provide extension traits or additional impl blocks for types. |
| `cfg_`     | Configuration                    | `cfg_env`, `cfg_db`   | Modules for managing environment config, settings, and constants.          |
| `tst_`     | Tests / Fixtures / Helpers       | `tst_user`, `tst_utils` | Modules with test helpers, mock data, or integration test logic.         |

---

## 📁 Folder Structure

Whenever practical, modules are grouped into folders based on their roles. This provides a clean and discoverable layout without sacrificing granularity.

model directory vs model file