# Render documentation preview

Deploy the `docs-preview` branch of `ElijahAhianyo/cot` as a Docker web service.
The Dockerfile fetches a pinned commit from the public `ElijahAhianyo/cot-site`
fork and initializes only the submodules required by the development preview.
Both crates build from source; no crates.io publication or upstream repository
settings are required.

## Render settings

Create a Web Service in your own Render account and connect your Cot fork.

| Setting | Value |
| --- | --- |
| Branch | `docs-preview` |
| Language | Docker |
| Root directory | Leave blank |
| Dockerfile path | `docs/site/deploy/Dockerfile` |
| Docker build context | `.` (repository root) |
| Docker command | Leave blank; use the image command |
| Instance type | Free |
| Health check path | `/guide/master/` |
| Auto-deploy | Off |

No database, disk, registry credentials, or application secrets are required.
Render supplies `PORT`; the server binds on `0.0.0.0`. Share the assigned HTTPS
address with `/guide/master/` appended. Free services sleep after 15 minutes
without traffic; the next visitor waits for startup. Build and bandwidth quotas
also apply. Review spending controls before enabling any paid usage.

## Local container check

Run from the Cot repository root:

```sh
docker build -f docs/site/deploy/Dockerfile -t cot-docs-preview .
docker run --rm -p 18081:10000 cot-docs-preview
```

Open `http://127.0.0.1:18081/guide/master/`. On Apple Silicon this builds an ARM
image; Render builds its own image for its platform. To reproduce Render's CPU
architecture locally, add `--platform linux/amd64` to both commands (slower under
emulation).

## Updating

Push page changes to `docs-preview`, then use Render's **Manual Deploy → Deploy
latest commit**. For renderer changes, first push the cot-site commit and update
`COT_SITE_REV` in the Dockerfile. Commit that pin before deploying. The ordinary
local manifests keep using sibling checkout paths.

This Dockerfile is separate from the existing production Docker workflow.
Historical documentation is not included in this development-only preview.

Sources: [Docker on Render](https://render.com/docs/docker) and
[free service limits](https://render.com/docs/free).
