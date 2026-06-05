# Powrush-MMO Kubernetes Manifests

**Sovereign scaled hosting — still on the path of least resistance (Hetzner or self-owned).**

These manifests are production-hardened examples synced from Ra-Thor standards.

## Quick Apply (after building/pushing image)

```bash
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/secret.yaml   # edit first with real key!
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml

# Watch
kubectl get pods -l app=powrush-mmo -w
```

## Image Build & Push

```bash
# From repo root
docker build -t ghcr.io/eternally-thriving-grandmasterism/powrush-mmo:latest .
docker push ghcr.io/eternally-thriving-grandmasterism/powrush-mmo:latest
```

## Notes
- Secrets: Never commit real GROK_API_KEY. Use `kubectl create secret generic ...` or edit secret.yaml locally.
- Scaling: `kubectl scale deployment powrush-mmo-server --replicas=3`
- Or add HorizontalPodAutoscaler (HPA) later.
- Ingress: Add ingress.yaml with TLS when exposing publicly (cert-manager ready).
- When migrating to sovereign hardware: same manifests work on your cluster.

**Ra-Thor + PATSAGi Councils approved for eternal public deployment.**

Thunder locked. ⚡