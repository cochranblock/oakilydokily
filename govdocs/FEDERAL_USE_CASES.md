<!-- Unlicense — cochranblock.org -->

# Federal Use Cases

> Which agencies could use oakilydokily, and how.
> Last updated: 2026-03-27

## VA — Department of Veterans Affairs

**Strongest fit.** The VA operates veterinary services through the VA Veterinary Medical Unit and supports service dog programs.

| Use Case | Description |
|----------|-------------|
| Service dog intake waivers | ESIGN-compliant waiver system for K-9 program intake. Liability release before training or veterinary procedures. |
| Veteran service provider portal | Freelance vet techs serving VA facilities need a professional services site with waiver capture. |
| VR&E self-employment showcase | oakilydokily itself is a product of VR&E Chapter 31 self-employment track — proof of concept. |

## DoD — Department of Defense

| Use Case | Description |
|----------|-------------|
| Military Working Dog (MWD) programs | Waiver and liability management for MWD handler facilities. Each base kennel needs signed liability releases. |
| Zero-cloud deployment | Single binary with no external dependencies deploys on NIPR/SIPR networks without cloud authorization. Air-gapped operation possible with SQLite backend. |
| Rapid deployment | 9.1 MB binary + systemd service = production in minutes. No Docker, no Kubernetes, no cloud account needed. |

## DHS — Department of Homeland Security

| Use Case | Description |
|----------|-------------|
| CBP K-9 operations | Customs and Border Protection K-9 units need waiver management for handler liability, training waivers, and third-party facility access. |
| TSA screening dog programs | Similar waiver needs for explosive detection dog teams visiting third-party training facilities. |

## DOJ — Department of Justice

| Use Case | Description |
|----------|-------------|
| ATF K-9 programs | Accelerant detection dog programs need signed liability waivers for facility visits and training exercises. |
| BOP institutional waivers | Bureau of Prisons animal-assisted therapy programs need ESIGN-compliant waiver capture. |

## GSA — General Services Administration

| Use Case | Description |
|----------|-------------|
| Template for shared services | oakilydokily's architecture (single-binary, ESIGN-compliant, multi-auth) could serve as a template for lightweight waiver/consent capture across agencies. |
| Procurement waiver system | GSA property disposal or surplus equipment pickup could use waiver capture for liability release. |

## USDA — Department of Agriculture

| Use Case | Description |
|----------|-------------|
| APHIS facility inspections | Animal and Plant Health Inspection Service inspectors visiting private facilities need signed access/liability waivers. |
| NRCS conservation programs | Natural Resources Conservation Service field work on private land needs landowner consent/waiver capture. |

## Why oakilydokily for Federal

| Advantage | Detail |
|-----------|--------|
| **Zero cloud** | No AWS, Azure, or GCP. Deploys on any Linux/macOS host. No FedRAMP needed for on-prem. |
| **Single binary** | 9.1 MB. No runtime dependencies. No Docker. No package manager. |
| **ESIGN compliant** | Terms versioning (SHA-256), IP/UA logging, typed signature, 7-year retention. |
| **Unlicense** | Public domain. No licensing fees. No vendor lock-in. Agency can fork and modify freely. |
| **SDVOSB** | Service-Disabled Veteran-Owned Small Business (pending certification). Eligible for SDVOSB set-asides. |
| **Air-gap capable** | SQLite backend, embedded assets. Works without internet after deployment. Email features degrade gracefully. |
