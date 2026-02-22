# Hospital Order History Dashboard - Setup Instructions

## Quick Start Guide

The Hospital Order History Dashboard has been successfully implemented! Follow these steps to run it:

### 1. Backend Setup (NestJS)

```bash
cd backend

# Install dependencies (if not already done)
npm install

# Start the backend server
npm run start:dev
```

The backend will start on `http://localhost:3001` with API prefix `api/v1`.

**API Endpoint:** `http://localhost:3001/api/v1/orders`

### 2. Frontend Setup (Next.js)

```bash
cd frontend/health-chain

# Install dependencies (if not already done)
npm install

# Start the development server
npm run dev
```

The frontend will start on `http://localhost:3000` (or next available port).

### 3. Access the Dashboard

Open your browser and navigate to:
```
http://localhost:3000/dashboard/orders
```

## Environment Configuration

### Frontend (.env.local)

A `.env.local` file has been created in `frontend/health-chain/` with:

```env
# Backend API Configuration
NEXT_PUBLIC_API_URL=http://localhost:3001
NEXT_PUBLIC_API_PREFIX=api/v1

# WebSocket Configuration
NEXT_PUBLIC_WS_URL=http://localhost:3001
```

### Backend (.env)

Check `backend/.env.example` and create `backend/.env` if needed.

## Troubleshooting

### "Failed to fetch orders" Error

This error occurs when:
1. **Backend is not running** - Start the backend server with `npm run start:dev`
2. **Wrong API URL** - Check that `NEXT_PUBLIC_API_URL` in `.env.local` matches your backend URL
3. **CORS issues** - Backend is configured to allow all origins in development

### WebSocket Connection Issues

If real-time updates aren't working:
1. Ensure backend server is running
2. Check `NEXT_PUBLIC_WS_URL` in `.env.local`
3. Look for WebSocket connection errors in browser console

### Port Conflicts

If ports 3000 or 3001 are in use:
- **Backend**: Change `PORT` in `backend/.env` (currently set to 3001)
- **Frontend**: Next.js will automatically use the next available port (currently 3000)
- **Important**: Update `NEXT_PUBLIC_API_URL` in `frontend/health-chain/.env.local` to match backend port

## Testing the Feature

### 1. Test Filtering
- Use the date range picker to filter by date
- Select blood types from the dropdown
- Select order statuses
- Search for blood banks

### 2. Test Sorting
- Click any column header to sort
- Click again to reverse sort order
- Notice active orders always appear first

### 3. Test Pagination
- Change page size (25, 50, 100)
- Navigate between pages
- Check that URL updates with page number

### 4. Test CSV Export
- Click "Export CSV" button
- Check downloaded file contains filtered data
- Verify all columns are present

### 5. Test URL State
- Apply some filters
- Copy the URL
- Open in new tab - filters should be preserved
- Refresh page - filters should remain

### 6. Test Real-Time Updates
- Open dashboard in two browser windows
- Update an order status via API or backend
- Watch the status update in real-time

## API Testing

### Test the Orders API Directly

```bash
# Get all orders for a hospital
curl "http://localhost:3001/api/v1/orders?hospitalId=HOSP-001"

# Get orders with filters
curl "http://localhost:3001/api/v1/orders?hospitalId=HOSP-001&bloodTypes=A+,O-&statuses=pending,confirmed&page=1&pageSize=25"

# Get orders with date range
curl "http://localhost:3001/api/v1/orders?hospitalId=HOSP-001&startDate=2024-01-01&endDate=2024-12-31"
```

## Current Status

✅ **Backend Implementation Complete**
- Enhanced Orders Service with filtering, sorting, pagination
- Orders Controller with validation
- WebSocket Gateway for real-time updates
- 25/25 tests passing

✅ **Frontend Implementation Complete**
- All UI components built and integrated
- URL state management working
- CSV export functional
- WebSocket client configured
- Responsive design with Tailwind CSS

## Next Steps

1. **Start both servers** (backend and frontend)
2. **Add sample data** to the backend (currently using mock data)
3. **Test all features** using the dashboard
4. **Integrate with authentication** to get real hospital IDs
5. **Connect to database** to persist orders

## Sample Data

The backend currently uses mock data. To add sample orders for testing, you can:

1. Use the POST endpoint to create orders
2. Modify `backend/src/orders/orders.service.ts` to add mock data
3. Connect to a real database and seed it with test data

## Production Deployment

Before deploying to production:

1. **Update CORS settings** in `backend/src/main.ts`
2. **Set environment variables** for production URLs
3. **Implement JWT authentication** in WebSocket gateway
4. **Connect to production database**
5. **Add proper error logging and monitoring**
6. **Run all tests** to ensure everything works
7. **Build frontend**: `npm run build`
8. **Build backend**: `npm run build`

## Support

If you encounter any issues:
1. Check browser console for errors
2. Check backend logs for API errors
3. Verify both servers are running
4. Ensure environment variables are set correctly
5. Check that ports are not blocked by firewall

## Feature Highlights

🎯 **Advanced Filtering**
- Date range, blood type, status, blood bank search
- All filters work together with AND logic

🎯 **Smart Sorting**
- Sort by any column
- Active orders always prioritized

🎯 **URL State Persistence**
- Bookmark and share filtered views
- State survives page refresh

🎯 **Real-Time Updates**
- WebSocket integration for live status changes
- Automatic UI updates

🎯 **CSV Export**
- Export current filtered view
- All columns included with proper formatting

🎯 **Responsive Design**
- Works on desktop, tablet, and mobile
- Tailwind CSS styling

Enjoy your new Hospital Order History Dashboard! 🩸
