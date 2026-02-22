# Restart Both Servers - Complete Fix

## Why Restart is Needed

1. **Backend**: Updated CORS configuration in `.env` to include `http://localhost:3000`
2. **Frontend**: Updated API URL in `.env.local` to point to `http://localhost:3001`

Both servers need to be restarted to pick up these environment variable changes.

## Step-by-Step Instructions

### 1. Stop Both Servers

In each terminal window running the servers:
- Press `Ctrl+C` to stop the server

### 2. Restart Backend

```bash
cd backend
npm run start:dev
```

**Expected output:**
```
Application is running on: http://localhost:3001/api/v1
Environment: development
```

### 3. Restart Frontend

```bash
cd frontend/health-chain
npm run dev
```

**Expected output:**
```
- ready started server on 0.0.0.0:3000, url: http://localhost:3000
```

### 4. Test the Dashboard

1. Open browser to: `http://localhost:3000/dashboard/orders`
2. Open DevTools (F12)
3. Check Console tab for:
   ```
   Fetching orders from: http://localhost:3001/api/v1/orders?hospitalId=HOSP-001...
   Environment variables: { apiUrl: 'http://localhost:3001', apiPrefix: 'api/v1' }
   ```

### 5. Verify Connection

The dashboard should now:
- ✅ Load without errors
- ✅ Show empty state (no orders yet) or mock data
- ✅ Allow filtering and sorting
- ✅ Show pagination controls

## Configuration Summary

### Backend (`.env`)
```env
PORT=3001
API_PREFIX=api/v1
CORS_ORIGIN=http://localhost:3000,http://localhost:3001,http://localhost:3002
```

### Frontend (`.env.local`)
```env
NEXT_PUBLIC_API_URL=http://localhost:3001
NEXT_PUBLIC_API_PREFIX=api/v1
NEXT_PUBLIC_WS_URL=http://localhost:3001
```

## Troubleshooting

### If backend won't start on 3001
```powershell
# Check what's using port 3001
Get-NetTCPConnection -LocalPort 3001 -ErrorAction SilentlyContinue

# Kill the process if needed (replace XXXX with process ID)
Stop-Process -Id XXXX -Force
```

### If frontend still shows old port
1. Stop the frontend server
2. Clear Next.js cache: `rm -rf .next` (or delete `.next` folder)
3. Restart: `npm run dev`
4. Hard refresh browser: `Ctrl+Shift+R`

### If CORS errors persist
Check backend logs for CORS-related messages. The backend should show:
```
CORS enabled for origins: http://localhost:3000, http://localhost:3001, http://localhost:3002
```

## Quick Verification Commands

### Test Backend API
```bash
curl "http://localhost:3001/api/v1/orders?hospitalId=HOSP-001"
```

### Check Ports
```powershell
# Backend on 3001
Get-NetTCPConnection -LocalPort 3001 -ErrorAction SilentlyContinue | Select-Object State

# Frontend on 3000
Get-NetTCPConnection -LocalPort 3000 -ErrorAction SilentlyContinue | Select-Object State
```

Both should show "Listen" state.

