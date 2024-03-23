<h1 align="center"> sqcr </h1>

`sqcr` is a simple application that reads SQL files from the command line and executes the queries within them. 

This tool allows for efficient database management and data manipulation directly from the command line.

---

## Features

- Read and execute SQL queries from files specified in the command line.

## Prerequisites

Please note that `sqcr` currently only supports MySQL databases.
Ensure that your database and queries are compatible with MySQL.

Before using `sqcr`, you need to set up a `.env` file with the following format for your database connection:


```
DATABASE_URL=mysql://<user-name>:<password>@<host>:<port>/<database-name>
```

## Usage

`sqcr` currently only supports MySQL.

 To use `sqcr`, simply pass the file containing your SQL queries as an argument to the command line. 

 Ensure that the queries in the file are compatible with MySQL. For example:

```bash
sqcr query-file.sql
```

## Installation

Binary distributions are not provided, so you will need to build the application in your own environment. Follow these steps to build:

1. Clone the repository:

```bash
git clone <repository-url>
```

2. Move to the project's root path:

```bash
cd sqcr
```

3. Build the project using `cargo build`:

```bash
cargo build --release
```

This command compiles the project in release mode, creating an optimized executable binary. 

After building, the executable file can be found in the target/release directory within your project folder.

## Author

[kip2](https://github.com/kip2)
