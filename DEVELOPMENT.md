# Development Guide

## Quick Start

### Option 1: Local Development

1. **Backend Setup**
   ```bash
   cd backend
   cp .env.example .env
   mkdir -p data
   cargo install sqlx-cli --no-default-features --features sqlite
   sqlx database create
   sqlx migrate run
   cargo run
   ```

2. **Frontend Setup**
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

### Option 2: Docker

```bash
docker-compose up --build
```

## Project Structure

### Backend (`/backend`)
- `src/main.rs` - Application entry point and route configuration
- `src/config.rs` - Environment configuration
- `src/error.rs` - Error handling and types
- `src/models/` - Data models (Property, Tenant, Event, etc.)
- `src/routes/` - API route handlers
- `src/db/` - Database layer
- `src/scraper/` - Market data scraping modules
- `migrations/` - SQLx database migrations

### Frontend (`/frontend`)
- `src/main.ts` - Vue application entry point
- `src/App.vue` - Root component
- `src/router/` - Vue Router configuration
- `src/views/` - Page components
- `src/components/` - Reusable components
- `src/stores/` - Pinia state management
- `src/services/` - API client services
- `src/types/` - TypeScript type definitions

## API Endpoints

### Properties
- `GET /api/properties` - List all properties
- `GET /api/properties/:id` - Get property by ID
- `POST /api/properties` - Create new property
- `PUT /api/properties/:id` - Update property
- `DELETE /api/properties/:id` - Delete property

### Tenants
- `GET /api/tenants` - List all tenants
- `GET /api/tenants/:id` - Get tenant by ID
- `POST /api/tenants` - Create new tenant
- `PUT /api/tenants/:id` - Update tenant
- `DELETE /api/tenants/:id` - Delete tenant

### Calendar Events
- `GET /api/events` - List all events
- `GET /api/events/:id` - Get event by ID
- `POST /api/events` - Create new event
- `PUT /api/events/:id` - Update event
- `DELETE /api/events/:id` - Delete event

### Maintenance
- `GET /api/maintenance` - List maintenance records
- `POST /api/maintenance` - Create maintenance record
- `PUT /api/maintenance/:id` - Update maintenance record

### Market Data
- `GET /api/market/trends` - Get market trend data
- `GET /api/market/analytics` - Get market analytics
- `POST /api/market/scrape` - Trigger data scraping

## Testing

### Backend Tests
```bash
cd backend
cargo test
cargo clippy
cargo fmt --check
```

### Frontend Tests
```bash
cd frontend
npm run type-check
npm run build
```

## Database Management

### Create a new migration
```bash
cd backend
sqlx migrate add <migration_name>
```

### Run migrations
```bash
sqlx migrate run
```

### Revert last migration
```bash
sqlx migrate revert
```

## Environment Variables

### Backend (`.env`)
```
DATABASE_URL=sqlite:./data/realestate.db
HOST=0.0.0.0
PORT=3000
RUST_LOG=info,realestate_backend=debug
CORS_ORIGIN=http://localhost:5173
```

### Frontend (`.env`)
```
VITE_API_BASE_URL=/api
```

## Deployment

### GitHub Pages (Frontend Only)
The frontend can be deployed to GitHub Pages using the included workflow.

1. Enable GitHub Pages in repository settings
2. Push to `main` branch
3. The deployment workflow will automatically build and deploy

### Docker Deployment
```bash
docker-compose up -d
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

## Troubleshooting

### Backend won't start
- Check that port 3000 is not in use
- Verify database file exists in `backend/data/`
- Check `.env` configuration

### Frontend won't connect to backend
- Verify backend is running on port 3000
- Check CORS configuration in backend
- Verify proxy settings in `vite.config.ts`

### Database errors
- Delete `backend/data/realestate.db` and run migrations again
- Check SQLx CLI is installed correctly
- Verify DATABASE_URL in `.env`
