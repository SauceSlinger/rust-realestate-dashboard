# GitHub Pages Deployment Guide

## 🚀 Deploying to GitHub Pages - Step by Step

### Prerequisites
- ✅ Code is committed to your GitHub repository
- ✅ You have admin access to the repository

---

## Step 1: Commit Your Code

First, let's commit all the files we've created:

```bash
cd /workspaces/rust-realestate-dashboard

# Stage all files
git add .

# Commit
git commit -m "Initial commit: Complete real estate dashboard framework"

# Push to GitHub
git push origin main
```

---

## Step 2: Enable GitHub Pages

1. **Go to your repository on GitHub**
   - Navigate to: `https://github.com/SauceSlinger/rust-realestate-dashboard`

2. **Open Settings**
   - Click on the "Settings" tab at the top of your repository

3. **Navigate to Pages**
   - In the left sidebar, scroll down and click on "Pages"

4. **Configure Source**
   - Under "Build and deployment"
   - Set **Source** to: `GitHub Actions`
   - (This will use our workflow file instead of the old branch-based deployment)

---

## Step 3: Configure Repository Secrets (Optional - for API)

If you have a backend API deployed somewhere:

1. **Go to Settings > Secrets and variables > Actions**

2. **Add Repository Secret**
   - Click "New repository secret"
   - Name: `API_BASE_URL`
   - Value: Your backend API URL (e.g., `https://api.yourdomain.com/api`)
   - Click "Add secret"

---

## Step 4: Trigger the Deployment

The deployment will happen automatically when you push to `main`, but you can also trigger it manually:

1. **Go to Actions tab** in your repository

2. **Select "Deploy to GitHub Pages"** workflow

3. **Click "Run workflow"** button
   - Select branch: `main`
   - Click "Run workflow"

4. **Wait for deployment** (usually 2-3 minutes)
   - You'll see the workflow running
   - Green checkmark means success ✅

---

## Step 5: Access Your Site

Once deployed, your site will be available at:

```
https://sauceslinger.github.io/rust-realestate-dashboard/
```

---

## 🔧 Important Configuration Notes

### Frontend-Only Deployment

Since GitHub Pages can only host static files (frontend), you have two options:

#### Option A: Frontend Only (Demo Mode)
The frontend will work as a demo with the seeded data displayed, but API calls won't work without a backend.

- **Best for**: Portfolio/demo purposes
- **Limitations**: No live data updates, CRUD operations won't persist

#### Option B: Frontend + Separate Backend
Deploy the backend separately and configure the frontend to use it:

1. **Deploy Backend** to a service like:
   - Render.com (Free tier available)
   - Railway.app (Free tier available)
   - Fly.io (Free tier available)
   - DigitalOcean App Platform
   - Heroku

2. **Update Frontend Configuration**:
   Edit `frontend/.env.production`:
   ```env
   VITE_API_BASE_URL=https://your-backend-url.com/api
   ```

3. **Update Backend CORS** in `backend/.env`:
   ```env
   CORS_ORIGIN=https://sauceslinger.github.io
   ```

---

## 🎯 Quick Backend Deployment Options

### Option 1: Deploy to Render.com (Recommended - Free)

1. **Create account** at https://render.com

2. **New Web Service**:
   - Connect your GitHub repo
   - Root directory: `backend`
   - Build command: `cargo build --release`
   - Start command: `./target/release/realestate-backend`

3. **Environment Variables**:
   ```
   DATABASE_URL=sqlite:./data/realestate.db
   HOST=0.0.0.0
   PORT=3000
   CORS_ORIGIN=https://sauceslinger.github.io
   ```

4. **Copy the URL** Render gives you (e.g., `https://your-app.onrender.com`)

5. **Update frontend/.env.production**:
   ```env
   VITE_API_BASE_URL=https://your-app.onrender.com/api
   ```

6. **Redeploy frontend** (push to GitHub or manually trigger workflow)

### Option 2: Deploy to Railway.app

1. **Create account** at https://railway.app

2. **New Project** > Deploy from GitHub repo

3. **Select backend directory**

4. **Add environment variables** (same as Render)

5. **Get your Railway URL** and update frontend config

### Option 3: Deploy to Fly.io

```bash
# Install flyctl
curl -L https://fly.io/install.sh | sh

# Login
flyctl auth login

# Initialize (in backend directory)
cd backend
flyctl launch

# Deploy
flyctl deploy
```

---

## 📝 Complete Deployment Checklist

### For Frontend-Only (Demo):
- [ ] Commit all code to GitHub
- [ ] Push to `main` branch
- [ ] Enable GitHub Pages in Settings
- [ ] Select "GitHub Actions" as source
- [ ] Wait for workflow to complete
- [ ] Access your site at `https://sauceslinger.github.io/rust-realestate-dashboard/`

### For Full-Stack:
- [ ] Deploy backend to Render/Railway/Fly.io
- [ ] Get backend URL
- [ ] Update `frontend/.env.production` with backend URL
- [ ] Update `backend/.env` with GitHub Pages URL for CORS
- [ ] Commit changes
- [ ] Push to GitHub
- [ ] Verify GitHub Actions deployment succeeds
- [ ] Test your site with working API

---

## 🐛 Troubleshooting

### GitHub Pages Not Showing

1. **Check Actions Tab**:
   - Make sure the "Deploy to GitHub Pages" workflow ran successfully
   - Look for any errors in the workflow logs

2. **Check Pages Settings**:
   - Verify "Source" is set to "GitHub Actions"
   - Check if there's a deployment listed

3. **Check Repository Visibility**:
   - Repository must be public for GitHub Pages (unless you have GitHub Pro)

### 404 Errors on Refresh

This is normal for single-page apps. The `.github/workflows/deploy.yml` workflow handles this, but if you see issues:

1. Check that the deployment created a `404.html` file
2. Verify the `base` path in `vite.config.ts` matches your repo name

### API Calls Failing

1. **Check CORS**: Make sure backend CORS_ORIGIN includes your GitHub Pages URL
2. **Check API URL**: Verify `VITE_API_BASE_URL` in `.env.production`
3. **Check Network Tab**: Open browser DevTools and check for CORS errors

### Build Fails

1. **Check Node Version**: GitHub Actions uses Node 20 (we're using v22 locally, should be compatible)
2. **Check Dependencies**: Make sure `package.json` is committed
3. **Check Logs**: Look at the Actions workflow logs for specific errors

---

## 🎨 Customization After Deployment

### Update Site Title/Description

Edit `frontend/index.html`:
```html
<title>Your Custom Title</title>
<meta name="description" content="Your description">
```

### Add Custom Domain

1. Go to Settings > Pages
2. Under "Custom domain", enter your domain
3. Follow GitHub's instructions for DNS configuration

### Update API Endpoint

Update `frontend/.env.production` and redeploy

---

## 📊 Monitoring Your Deployment

### Check Deployment Status
- Go to Actions tab
- Click on latest workflow run
- Check each step's status

### View Deployment
- Go to Settings > Pages
- Click "Visit site" button
- Or go directly to `https://sauceslinger.github.io/rust-realestate-dashboard/`

### Check Logs
- Actions tab > Select workflow > View logs
- Look for errors or warnings

---

## ✨ Next Steps After Deployment

1. **Test the site** - Make sure all pages load
2. **Check responsiveness** - Test on mobile devices
3. **Share the link** - Add to your portfolio
4. **Monitor usage** - Check GitHub Insights
5. **Keep updating** - Every push to `main` auto-deploys!

---

## 🎉 You're Done!

Your real estate dashboard is now live on GitHub Pages!

**Your Site**: `https://sauceslinger.github.io/rust-realestate-dashboard/`

Remember:
- Frontend-only = Demo mode with sample data
- Frontend + Backend = Fully functional with live data
- Every push to `main` triggers automatic deployment
- Deployment takes about 2-3 minutes

Happy deploying! 🚀
