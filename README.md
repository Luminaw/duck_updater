# Duck Updater

Duck Updater automatically updates your ip address on [DuckDNS](https://duckdns.org/) in rust.

## Prerequisites

To build, you need the following prerequisites:

- Rust toolchain installed on your system. You can install it from the official website: https://www.rust-lang.org/tools/install
- or Docker installed on your system. You can install it from the official website: https://www.docker.com/get-started
- A `.env` file in the project root directory with the following fields:
```env
    DDNS_DOMAIN=your-domain
    DDNS_TOKEN=your-token
    LOG_LOCATION=/var/log/duck_updater.log
```
## Usage

### Running Duck Updater Locally

To run Duck Updater on your local machine, follow these steps:

1. **Clone this repository to your local machine:**
    ```bash
    git clone https://gitlab.com/Luminaw/duck_updater
    cd duck_updater
    ```

2. **Build the project:**
    ```bash
    cargo build --release
    ```

3. **Run Duck Updater:**
    ```bash
    cargo run --release
    ```
    
### Running Duck Updater with Docker

To run Duck Updater using Docker, follow these steps:

1. **Build the Docker image:**
    ```bash
    docker build -t duck_updater .
    ```

2. **Run the Docker container:**
    ```bash
    docker run -d --name duck_updater -v /var/log:/var/log duck_updater
    ```

### Creating a Cron Job

To create a cron job for running the duck_updater at a specific interval, follow these steps:

1. **Open the `create_cronjob.sh` [script](scripts/create_cronjob.sh) in a text editor.**

2. **Modify the `PROGRAM_PATH` variable with the actual path to your duck_updater executable.**

3. **Save the changes and make the script executable:**
    ```bash
    chmod +x create_cronjob.sh
    ```

4. **Run the script to create the cron job:**
    ```bash
    ./create_cronjob.sh
    ```

This script will set up a cron job that runs the Duck Updater every 15 minutes, ensuring that your IP address is kept up to date.

### Creating a Windows Task

To create a windows task fro running duck_updater at a specific interval, follow these steps:

1. **Run the [script](scripts/create_task.bat) `create_task.bat` to create the Windows Task:**
    ```cmd
    create_task.bat
    ```

This script will set up a task that runs the Duck Updater every 15 minutes, ensuring that your IP address is kept up to date.


### Contributing

Contributions to this project are welcome! If you find a bug or want to suggest an enhancement, please open an issue or submit a pull request.

### License

This project is licensed under the MIT License. See the [`LICENSE`](LICENSE) file for details.