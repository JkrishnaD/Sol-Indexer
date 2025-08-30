# Solana Indexer

A high-performance Solana blockchain indexer that streams real-time blockchain data using yellowstone-grpc  to a PostgreSQL database via Redis for efficient data processing and storage.

## Architecture Overview
![alt text](assets/image.png)

This project implements a multi-crate Rust workspace that processes Solana blockchain data through a pipeline:

```
Solana Network → Geyser Adapter → Redis → Database Consumer → PostgreSQL
```

### Core Components

- **`core`**: Shared data models and Solana gRPC client integration
- **`geyser-adapter`**: Connects to Solana gRPC streams and publishes updates to Redis
- **`redis-adapter`**: Handles Redis pub/sub operations for data distribution
- **`db`**: Database operations and consumer logic for processing Redis messages
- **`config`**: Centralized configuration management

## Data Flow

1. **Data Ingestion**: `geyser-adapter` connects to Solana gRPC streams and receives real-time updates
2. **Data Processing**: Incoming Solana data is filtered and converted to internal models
3. **Data Distribution**: Processed data is published to Redis channels at `redis-adapter` for asynchronous consumption
4. **Data Storage**: `db` consumer reads from Redis channels and batches transactions into PostgreSQL
5. **Data Persistence**: Structured blockchain data is stored with proper indexing for efficient queries

## Prerequisites

- Rust 1.70+ and Cargo
- Docker and Docker Compose
- Diesel cli for database migrations

## Quick Start

### 1. Environment Setup

Create a `.env` file in the project root:

```bash
# Commands are based on the docker-compose.yml config 
DATABASE_URL=postgres://solana_user:secure_password@localhost:5432/indexer_db
REDIS_URL=redis://localhost:6379
RPC_URL=""
```

### 2. Start Infrastructure

```bash
# Start PostgreSQL and Redis containers
docker compose up -d
```

### 3. Database Setup

```bash
# Run database migrations
cd crates/db
diesel migration run
```

### 4. Run the Indexer

#### Terminal 1: Start Geyser Adapter
```bash
# From project root
cd crates/geyser-adapter
cargo run 
```

#### Terminal 2: Start Database Consumer
```bash
# From project root
cd crates/db
cargo run 
```

## Outputs:
- Redis channels will show incoming data
- Logs will display processing status and database insertions
![alt text](assets/image-1.png)
- Database tables will be created and populated as data flows through the system.
![alt text](assets/image-2.png)

## Configuration

### Geyser Filters

Configure what data to index in `crates/geyser-adapter/filters.json`:

```json
# Example filter to index all transactions involving the Vote
{
    "accounts": [
    {
      "accounts": ["Vote111111111111111111111111111111111111111"],
      "owners": ["TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"],
      "filters": []
    }
  ],
  "accounts_memcmp": [],
  "accounts_datasize": null,

  "include_slots": false,
  "include_blocks": false,
  "blocks_include_transactions": false,
  "blocks_include_accounts": false,
  "blocks_include_entries": false,

  "transactions": {
    "vote": false,
    "failed": true,
    "account_include": ["MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr","TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"],
    "account_exclude": [],
    "account_required": []
  },
  // More filter options...
  // like blocks, accounts and slots
}
```

### Database Schema

The system creates three main tables:
- `transactions`: Transaction details and metadata
- `accounts`: Account state changes
- `slots`: Slot information and status
Indexes are created on frequently queried fields for performance.

## Development

### Project Structure

```
sol-indexer/
├── crates/
│   ├── core/             # Shared models and Solana integration
│   ├── config/           # Configuration management
│   ├── geyser-adapter/   # Solana gRPC client and data publisher
│   ├── redis-adapter/    # Redis pub/sub implementation
│   └── db/               # Database operations and consumer
├── docker-compose.yml    # Infrastructure configuration
└── Cargo.toml            # Workspace configuration
```

## Monitoring and Debugging

### Check Redis Data Flow

```bash
# Monitor Redis channels
docker exec -it sol_indexer_redis redis-cli monitor

# Check specific channel
docker exec -it sol_indexer_redis redis-cli subscribe transactions
```

### Database Queries

```bash
# Connect to PostgreSQL
docker exec -it sol_indexer_db psql -U solana_user -d indexer_db

# Check transaction count
SELECT COUNT(*) FROM transactions;

# View recent transactions
SELECT id, slot, signature, fee FROM transactions ORDER BY id DESC LIMIT 10;
```

### Logs

```bash
# View geyser adapter logs
cargo run -p geyser-adapter 2>&1 | tee geyser.log

# View database consumer logs
cargo run -p db 2>&1 | tee db.log
```

## Performance Features

- **Batched Inserts**: Transactions are batched (100 per insert) for optimal database performance
- **Connection Pooling**: Uses r2d2 connection pool for efficient database connections
- **Asynchronous Processing**: Non-blocking Redis pub/sub for high-throughput data handling
- **Configurable Filtering**: Selective data indexing to reduce storage and processing overhead

## Troubleshooting

### Common Issues

1. **PostgreSQL Connection Failed**
   - Ensure Docker containers are running: `docker-compose ps`
   - Check `.env` file has correct `DATABASE_URL`

2. **Redis Connection Failed**
   - Verify Redis container is running: `docker exec -it sol_indexer_redis redis-cli ping`
   - Check `REDIS_URL` in `.env` file

3. **Compilation Errors**
   - Install PostgreSQL client: `brew install postgresql` (macOS)
   - Ensure Rust toolchain is up to date: `rustup update`

### Performance Tuning

- Adjust batch size in `crates/db/src/store.rs` for your database performance
- Modify Redis channel buffer sizes based on memory constraints
- Tune PostgreSQL connection pool settings in `crates/db/src/store.rs`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License.
