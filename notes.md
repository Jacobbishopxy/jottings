# Notes

## Cpp

- setting `clangd` and `clang-format` in VSCode config file `.vscode/settings.json`:

    ```json
    ...
    "clangd.path": "/opt/clangd_18.1.3/bin/clangd",
    "clangd.arguments": [
        "--compile-commands-dir=${workspaceFolder}/build",
        "--completion-style=detailed",
        "--header-insertion=never"
    ],
    "clang-format.language.cpp.style": "file",
    "clang-format.executable": "/usr/bin/clang-format-15",
    ...
    ```

- Setting `-std=c++20` in VSCode config file `.vscode/c_cpp_properties.json`:

    ```json
    ...
    "cStandard": "c17",
    "cppStandard": "c++20",
    "compilerArgs": [
        "-std=c++20"
    ],
    ...
    ```

- Setting VSCode config file `.vscode/tasks.json`:

    ```json
    ...
    "tasks": [
        {
        "type": "cmake",
        "label": "CMake: build",
        "command": "build",
        "targets": [
            "ALL_BUILD"
        ],
        "group": "build",
        "problemMatcher": [],
        "detail": "CMake template build task"
        }
    ]
    ...
    ```

- Package manager:

```sh
mkdir -p cmake
wget -O cmake/CPM.cmake https://github.com/cpm-cmake/CPM.cmake/releases/latest/download/get_cpm.cmake
```
