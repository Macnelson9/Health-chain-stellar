# API Connection Debug Guide

## Current Status

### Backend
- ✅ Running on port 3001
- ✅ API endpoint: `http://localhost:3001/api/v1/orders`
- ✅ Tested successfully with curl

### Frontend
- ✅ Running on port 3000
- ✅ Environment variables updated in `.env.local`
- ⚠️ Getting "Failed to fetch" error

## Debug Steps

### 1. Check Backend is Running

Open a new terminal and run:
```bash
curl "http://localhost:3001/api/v1/orders?hospitalId=HOSP-001"
```

Expected response:
```json
{"data":[],"pagination":{"currentPage":1,"pageSize":25,"totalCount":0,"totalPages":0}}
```

### 2. Check Frontend Environment Variables

The frontend needs to be restarted after updating `.env.local`. 

**IMPORTANT**: Stop the frontend server (Ctrl+C) and restart it:
```bash
cd frontend/health-chain
npm run dev
```

### 3. Check Browser Console

After restarting the frontend, open the dashboard and check the browser console (F12):

Look for these log messages:
```
Fetching orders from: http://localhost:3001/api/v1/orders?hospitalId=HOSP-001&...
Environment variables: { apiUrl: 'http://localhost:3001', apiPrefix: 'api/v1' }
```

If you see `http://localhost:3000` instead of `3001`, the frontend hasn't picked up the new environment variables.

### 4. Check Network Tab

In browser DevTools:
1. Go to Network tab
2. Refresh the page
3. Look for the request to `orders`
4. Check:
   - Request URL (should be `http://localhost:3001/api/v1/orders?...`)
   - Status code
   - Response

### 5. Common Issues

#### Issue: "Failed to fetch" with no status code
**Cause**: Network error, backend not running, or CORS issue
**Solution**: 
- Verify backend is running: `curl http://localhost:3001/api/v1/orders?hospitalId=HOSP-001`
- Check backend logs for errors
- Restart backend if needed

#### Issue: 400 Bad Request
**Cause**: Missing required parameter (hospitalId)
**Solution**: Check that hospitalId is being sent in the request

#### Issue: CORS error
**Cause**: Backend not allowing requests from frontend origin
**Solution**: Backend `.env` has been updated to include `http://localhost:3000` in CORS_ORIGIN

#### Issue: Still using old port (3000)
**Cause**: Frontend not restarted after `.env.local` update
**Solution**: 
1. Stop frontend (Ctrl+C)
2. Restart: `npm run dev`
3. Hard refresh browser (Ctrl+Shift+R)

### 6. Manual Test

You can test the API directly in the browser:

Open a new tab and navigate to:
```
http://localhost:3001/api/v1/orders?hospitalId=HOSP-001&page=1&pageSize=25&sortBy=placedAt&sortOrder=desc
```

You should see a JSON response with orders data.

### 7. Check Backend Logs

Look at the terminal running the backend server. You should see:
```
Application is running on: http://localhost:3001/api/v1
Environment: development
```

When you access the dashboard, you should see incoming requests logged.

## Quick Fix Checklist

- [ ] Backend is running on port 3001
- [ ] Backend `.env` file exists with PORT=3001
- [ ] Backend CORS includes `http://localhost:3000`
- [ ] Frontend `.env.local` has `NEXT_PUBLIC_API_URL=http://localhost:3001`
- [ ] Frontend server has been restarted after `.env.local` update
- [ ] Browser has been hard refreshed (Ctrl+Shift+R)
- [ ] Browser console shows correct API URL (3001, not 3000)

## Still Not Working?

If you've completed all the above steps and it's still not working:

1. **Check the actual error in browser console**
   - Look for the detailed error logs we added
   - Check if it's a network error, CORS error, or API error

2. **Verify the request in Network tab**
   - What URL is being called?
   - What's the status code?
   - What's the response?

3. **Test backend directly**
   ```bash
   curl -v "http://localhost:3001/api/v1/orders?hospitalId=HOSP-001"
   ```
   The `-v` flag shows verbose output including headers

4. **Check for port conflicts**
   ```powershell
   Get-NetTCPConnection -LocalPort 3001 | Select-Object State, OwningProcess
   ```

5. **Restart both servers**
   - Stop backend (Ctrl+C)
   - Stop frontend (Ctrl+C)
   - Start backend: `cd backend && npm run start:dev`
   - Start frontend: `cd frontend/health-chain && npm run dev`

