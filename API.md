# API Documentation

Base URL: `http://localhost:3000/api`

## Properties API

### List All Properties
```http
GET /api/properties
```

**Response:**
```json
[
  {
    "id": 1,
    "address": "123 Main St",
    "city": "San Francisco",
    "state": "CA",
    "zip_code": "94105",
    "property_type": "single-family",
    "bedrooms": 3,
    "bathrooms": 2.5,
    "square_feet": 2000,
    "purchase_price": 800000.0,
    "current_value": 950000.0,
    "status": "active",
    "notes": "Recently renovated",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-15T10:30:00Z"
  }
]
```

### Create Property
```http
POST /api/properties
Content-Type: application/json
```

**Request Body:**
```json
{
  "address": "456 Oak Ave",
  "city": "Austin",
  "state": "TX",
  "zip_code": "78701",
  "property_type": "condo",
  "bedrooms": 2,
  "bathrooms": 2.0,
  "square_feet": 1500,
  "purchase_price": 450000.0,
  "current_value": 480000.0,
  "status": "active",
  "notes": "Great location"
}
```

**Response:**
```json
1
```

### Get Property by ID
```http
GET /api/properties/:id
```

### Update Property
```http
PUT /api/properties/:id
Content-Type: application/json
```

**Request Body:** (all fields optional)
```json
{
  "current_value": 500000.0,
  "status": "sold"
}
```

### Delete Property
```http
DELETE /api/properties/:id
```

## Reminders API

### List All Reminders
```http
GET /api/reminders
```

**Response:**
```json
[
  {
    "id": 1,
    "property_id": 1,
    "title": "Annual HVAC Maintenance",
    "description": "Schedule yearly maintenance check",
    "due_date": "2024-06-15",
    "completed": false,
    "reminder_type": "maintenance",
    "created_at": "2024-01-15T10:30:00Z"
  }
]
```

### Create Reminder
```http
POST /api/reminders
Content-Type: application/json
```

**Request Body:**
```json
{
  "property_id": 1,
  "title": "Property Tax Due",
  "description": "Pay annual property tax",
  "due_date": "2024-04-15",
  "reminder_type": "tax"
}
```

### Update Reminder
```http
PUT /api/reminders/:id
Content-Type: application/json
```

**Request Body:** (all fields optional)
```json
{
  "completed": true
}
```

### Delete Reminder
```http
DELETE /api/reminders/:id
```

## Tenants API

### List All Tenants
```http
GET /api/tenants
```

**Query Parameters:**
- `property_id` (optional): Filter by property ID

**Response:**
```json
[
  {
    "id": 1,
    "property_id": 1,
    "name": "John Doe",
    "email": "john@example.com",
    "phone": "555-0123",
    "lease_start": "2024-01-01",
    "lease_end": "2024-12-31",
    "monthly_rent": 2500.0,
    "deposit": 5000.0,
    "status": "active",
    "created_at": "2024-01-01T10:00:00Z"
  }
]
```

### Create Tenant
```http
POST /api/tenants
Content-Type: application/json
```

**Request Body:**
```json
{
  "property_id": 1,
  "name": "Jane Smith",
  "email": "jane@example.com",
  "phone": "555-0124",
  "lease_start": "2024-02-01",
  "lease_end": "2025-01-31",
  "monthly_rent": 2600.0,
  "deposit": 5200.0,
  "status": "active"
}
```

### Update Tenant
```http
PUT /api/tenants/:id
Content-Type: application/json
```

### Delete Tenant
```http
DELETE /api/tenants/:id
```

## Market Data API

### Get Market Data
```http
GET /api/market-data
```

**Query Parameters:**
- `location` (optional): Filter by location

**Response:**
```json
[
  {
    "id": 1,
    "location": "San Francisco, CA",
    "median_price": 1250000.0,
    "average_price_per_sqft": 950.0,
    "inventory_count": 523,
    "days_on_market": 28,
    "data_date": "2024-01-15",
    "source": "Mock Data",
    "created_at": "2024-01-15T10:00:00Z"
  }
]
```

### Add Market Data
```http
POST /api/market-data
Content-Type: application/json
```

**Request Body:**
```json
{
  "location": "Seattle, WA",
  "median_price": 825000.0,
  "average_price_per_sqft": 575.0,
  "inventory_count": 654,
  "days_on_market": 24,
  "data_date": "2024-01-15",
  "source": "Manual Entry"
}
```

### Get Market Trends
```http
GET /api/market-data/trends
```

**Query Parameters:**
- `location` (optional): Filter by location

**Response:**
```json
[
  {
    "location": "San Francisco, CA",
    "trend_data": [
      {
        "date": "2024-01-15",
        "median_price": 1250000.0,
        "average_price_per_sqft": 950.0
      }
    ]
  }
]
```

## Scraper API

### Trigger Market Data Scraping
```http
POST /api/scrape
```

**Response:**
```json
{
  "message": "Market data scraped successfully",
  "count": 3
}
```

## Error Responses

All endpoints may return the following error responses:

**400 Bad Request**
```json
{
  "error": "Invalid request data"
}
```

**404 Not Found**
```json
{
  "error": "Resource not found"
}
```

**500 Internal Server Error**
```json
{
  "error": "Internal server error"
}
```

## Data Types

### Property Types
- `single-family`
- `multi-family`
- `condo`
- `townhouse`
- `commercial`

### Property Status
- `active`
- `inactive`
- `sold`

### Reminder Types
- `maintenance`
- `inspection`
- `rent-due`
- `lease-renewal`
- `tax`
- `other`

### Tenant Status
- `active`
- `inactive`
- `pending`
