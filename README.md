# Real Estate Market Insights & Property Management Dashboard

A full-stack real estate dashboard built with Rust (Axum) and Vue 3 (TypeScript) for tracking market insights, managing properties, tenants, and maintenance schedules.

**Currently Hosted at:** https://sauceslinger.github.io/rust-realestate-dashboard/

## 🚀 Features

### Property Management
- **Portfolio Management**: Add, edit, view, and delete properties in your portfolio
- **Tenant Management**: Track tenant information, lease agreements, and contact details
- **Maintenance Tracking**: Log and monitor property maintenance requests and alerts
- **Rent Collection**: Record and track rent payments and financial records

### Market Insights
- **Real Estate Data Aggregation**: Automated scraping of market data from various sources
- **Market Trend Visualizations**: Interactive charts showing price trends, inventory levels, and market metrics
- **Comparative Analytics**: Compare properties and market segments

### Calendar & Reminders
- **Event Management**: Calendar-based system for property-related events
- **Automated Reminders**: Maintenance schedules, rent due dates, lease renewals
- **Notification System**: Stay on top of important deadlines

## 🛠️ Tech Stack

### Backend
- **Rust** with **Axum** web framework
- **SQLite** for lightweight, embedded data storage
- **Tokio** async runtime
- **Serde** for JSON serialization
- **SQLx** for type-safe database queries
- **Tower** middleware for CORS and error handling

### Frontend
- **Vue 3** with **TypeScript** and Composition API
- **Vite** for lightning-fast development and optimized builds
- **Tailwind CSS** for utility-first styling
- **Vue Router** for client-side routing
- **Pinia** for reactive state management
- **Chart.js/ApexCharts** for data visualization
- **Axios** for HTTP client

### DevOps
- **Docker** & **Docker Compose** for containerization
- **GitHub Actions** for CI/CD pipelines
- Automated data scraping workflows
- GitHub Pages deployment ready

## 📁 Project Structure

```
rust-realestate-dashboard/
├── backend/                 # Rust Axum API server
│   ├── src/
│   │   ├── main.rs         # Application entry point
│   │   ├── config.rs       # Configuration management
│   │   ├── routes/         # API route handlers
│   │   ├── models/         # Data models
│   │   ├── db/             # Database layer
│   │   ├── scraper/        # Data scraping modules
│   │   └── error.rs        # Error handling
│   ├── data/               # SQLite database files
│   ├── migrations/         # Database migrations
│   ├── Cargo.toml
│   └── Dockerfile
├── frontend/               # Vue 3 TypeScript SPA
│   ├── src/
│   │   ├── main.ts
│   │   ├── App.vue
│   │   ├── components/     # Reusable components
│   │   ├── views/          # Page components
│   │   ├── stores/         # Pinia stores
│   │   ├── services/       # API client services
│   │   ├── router/         # Vue Router configuration
│   │   └── types/          # TypeScript types
│   ├── public/
│   ├── package.json
│   ├── vite.config.ts
│   ├── tailwind.config.js
│   └── Dockerfile
├── .github/
│   └── workflows/          # CI/CD and scraping workflows
├── docker-compose.yml
├── LICENSE
└── README.md
```

## 🚦 Getting Started

### Prerequisites
- **Rust** 1.70+ ([Install Rust](https://rustup.rs/))
- **Node.js** 18+ and **npm** ([Install Node.js](https://nodejs.org/))
- **Docker** & **Docker Compose** (optional, for containerized deployment)

### Local Development

#### Backend Setup
```bash
# Navigate to backend directory
cd backend

# Create data directory for SQLite
mkdir -p data

# Run database migrations
cargo install sqlx-cli
sqlx database create
sqlx migrate run

# Start the development server
cargo run
# Server runs on http://localhost:3000
```

#### Frontend Setup
```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev
# App runs on http://localhost:5173
```

### Docker Deployment
```bash
# Build and start all services
docker-compose up --build

# Backend: http://localhost:3000
# Frontend: http://localhost:5173
```

## 🔌 API Endpoints

### Properties
- `GET /api/properties` - List all properties
- `GET /api/properties/:id` - Get property details
- `POST /api/properties` - Create new property
- `PUT /api/properties/:id` - Update property
- `DELETE /api/properties/:id` - Delete property

### Tenants
- `GET /api/tenants` - List all tenants
- `GET /api/tenants/:id` - Get tenant details
- `POST /api/tenants` - Create new tenant
- `PUT /api/tenants/:id` - Update tenant
- `DELETE /api/tenants/:id` - Delete tenant

### Calendar & Events
- `GET /api/events` - List all events
- `GET /api/events/:id` - Get event details
- `POST /api/events` - Create new event
- `PUT /api/events/:id` - Update event
- `DELETE /api/events/:id` - Delete event

### Market Data
- `GET /api/market/trends` - Get market trend data
- `GET /api/market/analytics` - Get market analytics
- `POST /api/market/scrape` - Trigger data scraping (admin)

### Maintenance
- `GET /api/maintenance` - List maintenance records
- `POST /api/maintenance` - Create maintenance record
- `PUT /api/maintenance/:id` - Update maintenance record

## 🗄️ Database Schema

The SQLite database includes the following tables:
- **properties** - Property listings with details
- **tenants** - Tenant information and contacts
- **leases** - Lease agreements linking tenants to properties
- **maintenance_records** - Maintenance and repair tracking
- **rent_payments** - Payment history
- **calendar_events** - Events and reminders
- **market_data** - Scraped market insights and trends

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with Rust and Vue.js
- Inspired by modern property management needs
- Community-driven development

---

**Note**: This is an actively developed project. Features and documentation are continuously updated.
Lightweight Rust backend with Vue/TypeScript frontend for real estate market insights and property management dashboard
