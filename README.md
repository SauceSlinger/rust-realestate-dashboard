# Real Estate Dashboard

A full-stack real estate management dashboard built with Rust (Axum) backend and Vue 3 TypeScript frontend. Features property management, tenant tracking, calendar reminders, and market data analytics.

## Features

### Backend (Rust + Axum)
- 🏠 **Property Management**: Full CRUD operations for property listings
- 📅 **Calendar & Reminders**: Maintenance alerts, lease renewals, and custom reminders
- 👥 **Tenant Management**: Track tenant information, leases, and rent payments
- 📊 **Market Data Analytics**: Track real estate market trends and statistics
- 🔍 **Data Scraper**: Automated market data collection
- 💾 **SQLite Database**: Lightweight, serverless database perfect for GitHub hosting

### Frontend (Vue 3 + TypeScript + Tailwind CSS)
- 📱 **Responsive Design**: Mobile-first, works on all devices
- 📈 **Interactive Charts**: Visual market trend analytics with Chart.js
- 🎨 **Modern UI**: Clean interface with Tailwind CSS
- ⚡ **Fast Performance**: Vue 3 with TypeScript for type safety
- 🔄 **Real-time Updates**: Dynamic data fetching with Axios

## Tech Stack

### Backend
- **Framework**: Axum (Rust web framework)
- **Database**: SQLite with SQLx
- **Web Scraping**: Scraper crate
- **HTTP Client**: Reqwest
- **Async Runtime**: Tokio

### Frontend
- **Framework**: Vue 3 with Composition API
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **Charts**: Chart.js + vue-chartjs
- **HTTP Client**: Axios
- **Build Tool**: Vite
- **Router**: Vue Router
- **State Management**: Pinia

## Getting Started

### Prerequisites
- Rust 1.75 or later
- Node.js 20 or later
- npm or yarn

### Installation

1. **Clone the repository**
```bash
git clone https://github.com/SauceSlinger/rust-realestate-dashboard.git
cd rust-realestate-dashboard
```

2. **Backend Setup**
```bash
cd backend
cargo build
cargo run
```
The backend API will start on `http://localhost:3000`

3. **Frontend Setup**
```bash
cd frontend
npm install
npm run dev
```
The frontend will start on `http://localhost:5173`

### Using Docker

**Development with Docker Compose:**
```bash
docker-compose up
```

**Production Build:**
```bash
docker build -t realestate-dashboard .
docker run -p 3000:3000 -v $(pwd)/data:/app/data realestate-dashboard
```

## API Endpoints

### Properties
- `GET /api/properties` - List all properties
- `POST /api/properties` - Create a new property
- `GET /api/properties/:id` - Get property details
- `PUT /api/properties/:id` - Update property
- `DELETE /api/properties/:id` - Delete property

### Reminders
- `GET /api/reminders` - List all reminders
- `POST /api/reminders` - Create a reminder
- `PUT /api/reminders/:id` - Update reminder
- `DELETE /api/reminders/:id` - Delete reminder

### Tenants
- `GET /api/tenants` - List all tenants
- `POST /api/tenants` - Create a tenant
- `PUT /api/tenants/:id` - Update tenant
- `DELETE /api/tenants/:id` - Delete tenant

### Market Data
- `GET /api/market-data` - Get market data
- `POST /api/market-data` - Add market data
- `GET /api/market-data/trends` - Get market trends
- `POST /api/scrape` - Trigger data scraper

## Project Structure

```
.
├── backend/              # Rust Axum backend
│   ├── src/
│   │   ├── handlers/    # API route handlers
│   │   ├── models/      # Data models
│   │   ├── db.rs        # Database operations
│   │   └── main.rs      # Application entry point
│   └── Cargo.toml       # Rust dependencies
├── frontend/            # Vue 3 TypeScript frontend
│   ├── src/
│   │   ├── views/       # Page components
│   │   ├── services/    # API services
│   │   ├── router/      # Vue Router config
│   │   └── assets/      # Static assets
│   └── package.json     # Node dependencies
├── .github/
│   └── workflows/       # GitHub Actions CI/CD
├── Dockerfile           # Production Docker image
└── docker-compose.yml   # Docker Compose config
```

## Development

### Backend Development
```bash
cd backend
cargo watch -x run  # Auto-reload on changes
```

### Frontend Development
```bash
cd frontend
npm run dev  # Hot module replacement enabled
```

### Building for Production

**Backend:**
```bash
cd backend
cargo build --release
```

**Frontend:**
```bash
cd frontend
npm run build
```

## GitHub Actions

This project includes two GitHub Actions workflows:

1. **CI/CD Pipeline** (`.github/workflows/ci-cd.yml`)
   - Runs on push and pull requests
   - Tests backend and frontend
   - Builds Docker image on main branch

2. **Data Scraper** (`.github/workflows/scraper.yml`)
   - Scheduled to run daily at 2 AM UTC
   - Scrapes market data automatically
   - Can be triggered manually

## Database Schema

### Properties
- Address, city, state, zip code
- Property type, bedrooms, bathrooms, square feet
- Purchase price, current value, status
- Notes, timestamps

### Reminders
- Title, description, due date
- Reminder type, completion status
- Optional property association

### Tenants
- Name, email, phone
- Lease start/end dates
- Monthly rent, deposit, status
- Property association

### Market Data
- Location, median price
- Average price per sq ft
- Inventory count, days on market
- Data date, source

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Rust and Vue.js
- Styled with Tailwind CSS
- Charts powered by Chart.js
- Database powered by SQLite

## Support

For issues and questions, please open an issue on GitHub.
