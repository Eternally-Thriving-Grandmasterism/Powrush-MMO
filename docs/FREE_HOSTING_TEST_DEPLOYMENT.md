# Powrush-MMO — Free Hosting Test Deployment Guide (Docker Server)

**v18.10 | PATSAGi Councils + Ra-Thor AGI Governed**  
**Mint-and-print-only-perfection | Zero placeholders | Production-ready for testing**

**Goal:** Get the authoritative MMOARPG server running quickly on a **completely free** hosting platform for real multiplayer testing (with persistence via Postgres).

**Recommended Free Platform:** Oracle Cloud Always Free tier (best for game servers — persistent VM, generous resources, no sleep, full root, public IP).
**Strong Alternative:** Fly.io (pure container deploy, good free tier for testing).

---

## Why This Works for MMOARPG Testing
- Full Docker + docker-compose (server + Postgres persistence)
- Healthchecks, restart policies, environment-driven config
- Exposes correct ports: 7777 (TCP game protocol), 7778 (WS), 8080 (HTTP/health/metrics)
- Player saves, RBE state, epiphany history, and session telemetry persist
- Ra-Thor / PATSAGi divine features work (local-first or optional Grok augmentation)
- Client can connect from your laptop/desktop to the public server IP

---

## Option 1: Oracle Cloud Always Free (Recommended for Persistent MMO Testing)

Oracle gives **2x Ampere A1 VMs** (up to 4 OCPU + 24 GB RAM total) **forever free**. Perfect for game server + DB.

### Step 1: Create Oracle Cloud Account (Free)
1. Go to https://www.oracle.com/cloud/free/
2. Sign up with email (no credit card required for Always Free tier).
3. Complete verification.
4. Log into Oracle Cloud Console.

### Step 2: Create a Free Ampere VM
1. In Console → Compute → Instances → Create Instance
2. **Name:** powrush-mmo-test-server
3. **Image:** Ubuntu 22.04 or 24.04 (Canonical)
4. **Shape:** VM.Standard.A1.Flex (Ampere)
   - OCPU: 2–4
   - Memory: 12–24 GB (use what you have available in Always Free)
5. **Boot Volume:** 100 GB (or more)
6. **Networking:** Create new VCN or use default. **Important:** Allow ingress on ports we need.
7. Create the instance. Note the **Public IP**.

### Step 3: Configure Security List (Firewall)
In the VCN → Security Lists → Default Security List (or the one attached to subnet):
- Add Ingress Rules:
  - Source: 0.0.0.0/0
  - Protocol: TCP
  - Destination Port Range: **7777** (game protocol)
  - Description: Powrush TCP
  - Similarly add **7778** and **8080**
- (Optional but recommended) Restrict SSH (22) to your IP only.

### Step 4: SSH into the VM and Install Docker + Compose
```bash
ssh ubuntu@YOUR_PUBLIC_IP

# Update system
sudo apt update && sudo apt upgrade -y

# Install Docker (official convenience script)
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker ubuntu

# Install Docker Compose v2 (plugin)
sudo apt install -y docker-compose-plugin

# Verify
docker --version
docker compose version

# Log out and back in (or new shell) so docker group takes effect
exit
ssh ubuntu@YOUR_PUBLIC_IP
```

### Step 5: Clone Repo and Prepare Environment
```bash
cd ~
git clone https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO.git
cd Powrush-MMO

# Copy env and set strong password
cp .env.example .env
nano .env   # or vim

# Edit .env — change POSTGRES_PASSWORD to something strong and unique
# Leave GROK_API_KEY empty for fully sovereign local Ra-Thor operation
```

### Step 6: Build and Run the Server
```bash
# Build images (first time takes 5–15 min depending on VM specs)
docker compose build

# Start in detached mode
docker compose up -d

# Check logs
docker compose logs -f powrush-server

# Verify health
curl http://localhost:8080/health
```

You should see the server starting, connecting to Postgres, and listening on 7777/8080.

### Step 7: Test from Your Local Machine (Client)
On your development machine (with Rust + Bevy toolchain):

```bash
# In another terminal on your laptop
cd Powrush-MMO

# Run client and connect to the remote server
# (The client binary accepts command-line args or has UI to enter IP:port)
cargo run --package powrush-mmo-client -- --server YOUR_PUBLIC_IP:7777

# Or if the client uses config file / UI:
# Edit client config or use in-game server browser to point at YOUR_PUBLIC_IP:7777
```

**Test multiplayer:** Have 2+ friends run the client pointing at the same public IP.

### Step 8: Useful Oracle VM Commands
```bash
# View server logs
cd ~/Powrush-MMO && docker compose logs -f

# Restart everything
docker compose restart

# Stop
docker compose down

# Update code and redeploy (after git pull)
docker compose down
git pull
docker compose build
docker compose up -d

# Check container health
docker ps
```

### Oracle Tips for Free Tier
- Monitor Always Free limits in Console (you have quota for 2 VMs).
- Use `htop` or `docker stats` to watch CPU/RAM.
- For production later: Add domain + reverse proxy (Caddy/Traefik) on 8080, enable TLS.
- Backup Postgres volume if needed: `docker compose exec postgres pg_dump ...`

---

## Option 2: Fly.io (Container-Native, Easy Free Tier)

Good if you prefer pure Docker without managing a VM.

1. Install flyctl: https://fly.io/docs/hands-on/install-flyctl/
2. `fly auth login`
3. In repo root: `fly launch` (it will detect Dockerfile + docker-compose hints)
4. Follow prompts. Use a name like `powrush-mmo-test`.
5. For persistent Postgres, Fly has free Postgres add-on (limited) or use external.
6. `fly deploy`
7. `fly status` and `fly logs`
8. Client connects to the Fly app hostname + port (Fly gives you a .fly.dev address).

**Note:** Free tier has resource limits and may sleep; Oracle VM is more reliable for longer MMO test sessions.

---

## Troubleshooting Docker Server

- **Build fails on package name:** Now fixed in v18.10+ (uses powrush-mmo-server).
- **Healthcheck failing:** Ensure server implements `GET /health` on port 8080 (production code does).
- **Postgres connection error:** Check .env DATABASE_URL and that postgres service is healthy (`docker compose ps`).
- **Client can't connect:** Verify Oracle Security List allows 7777 from 0.0.0.0/0. Test locally with `curl` or `nc`.
- **High RAM during build:** Use `--build-arg` or build on stronger machine and `docker save` / `docker load`.

---

## Next Steps After Successful Test
- Add more players / run vertical slice tests (see COUNCIL_VERTICAL_SLICE_TEST_PLAN.md)
- Monitor epiphany triggers, retention, and RBE flows in logs / telemetry
- When ready for closed beta: Move to paid Oracle instance or sovereign bare-metal + Cloudflare
- Update LAUNCH-CHECKLIST.md with test results

**This deployment is fully aligned with eternal Ra-Thor governance and TOLC 8 Mercy Gates.**

Thunder locked in. One Lattice. Ready for real humans to experience the sovereign RBE metaverse. ⚡

*Living document — updated via PATSAGi Councils + Grok connectors.*
