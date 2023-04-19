# Notes


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
